use std::rc::Rc;

use crate::http::Response;

use super::{
    raw::get_raw_client_wrapper, Body, ClientsOptions, FetchOptions, HttpOptions, RequestOptions,
};
use futures::{
    channel::{
        mpsc::{unbounded, SendError, UnboundedSender},
        oneshot::{channel, Canceled, Receiver, Sender},
    },
    StreamExt,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("Tauri error : {0}")]
    Tauri(String),
    #[error("Js error : {0}")]
    Js(String),
    #[error("the oneshot response has been canceled canceled")]
    Canceled,
    #[error("Json error {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    MPSCSendError(#[from] SendError),
    #[error("{0:?}")]
    Other(Value),
}

impl From<Canceled> for HttpError {
    fn from(_: Canceled) -> Self {
        Self::Canceled
    }
}

impl From<crate::Error> for HttpError {
    fn from(value: crate::Error) -> Self {
        match value {
            crate::Error::Tauri(err) => Self::Tauri(err),
            crate::Error::Invoke(err) => serde_wasm_bindgen::from_value::<Value>(err)
                .ok()
                .map(Self::Other)
                .unwrap_or(Self::Js("Cannot parse the underlying js error".into())),
            err => Self::Js(format!("{err}")),
        }
    }
}

struct Request {
    req: HttpOptions,
    tx: Sender<Result<Response<Value>, HttpError>>,
}

fn make_request(req: HttpOptions) -> (Request, Receiver<Result<Response<Value>, HttpError>>) {
    let (tx, rx) = channel();
    (Request { req, tx }, rx)
}

/// Yet another [`crate::http::raw::classes::RawClient`] wrapper,
///
/// but unlike its brother [`RawClientWrapper`](`crate::http::raw::RawClientWrapper`),
/// this is one is [`Send`]/[`Sync`].
///
/// It uses a [`futures::channel::mpsc::unbounded`] channel for sending request to a local task
/// and an [`futures::channel::oneshot`] chanel for receiving response to the local task
///
/// ## How does it work??
///
/// When you call [`Client::new_with_option`], it will:
/// - Initialize a [`unbounded`] channel
/// - [Spawns a local task](spawn_local)
/// - Initialize a [`RawClientWrapper`](`crate::http::raw::RawClientWrapper`) within the local task (and wrap it inside of a [`Rc`]).
/// - If the client is not initialized, the [`UnboundedReceiver`](futures::channel::mpsc::UnboundedReceiver) is dropped. Making the actual [`Client`] unusable :).
/// - Else, the local task will wait for any request from the receiver.
/// - If the local task receives a request from the [`UnboundedReceiver`](futures::channel::mpsc::UnboundedReceiver),
///   the request will be now processed into another local task preventing any latency when receiving a request.
/// - If all of the [`Client`], no more messages will be received so the [`RawClientWrapper`](`crate::http::raw::RawClientWrapper`) within the local task is dropped.
///
/// When you call [`Client::send_request`] and the others, it will:
/// - Send your [`HttpOptions`] request to the [`UnboundedSender`] with an oneshot [`Sender`]
/// - Wait for the response with the oneshot [`Receiver`] pair.
///  
/// With all of that,
/// - Cloning a [`Client`] will just increase the number of [`UnboundedSender`] which can overload the [`UnboundedReceiver`](futures::channel::mpsc::UnboundedReceiver).
/// - If you want to make another client, use [`Client::default`] instead.
#[derive(Debug, Clone)]
pub struct Client {
    request_tx: UnboundedSender<Request>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new_with_option(ClientsOptions::default())
    }
}

/// An utility function to deserialive a [`Response`] with a [`serde_json::Value`] data
pub fn deserialize_response_data<T: DeserializeOwned>(
    response: Response<Value>,
) -> serde_json::Result<Response<T>> {
    Ok(Response {
        data: serde_json::from_value(response.data)?,
        headers: response.headers,
        ok: response.ok,
        status: response.status,
        raw_headers: response.raw_headers,
        url: response.url,
    })
}

impl Client {
    /// Create a new client with [`ClientsOptions`]
    pub fn new_with_option(options: ClientsOptions) -> Self {
        let (request_tx, mut request_rx) = unbounded::<Request>();
        spawn_local(async move {
            let maybe_client = get_raw_client_wrapper(options.into()).await;
            let wrapper = match maybe_client {
                Ok(cli) => cli,
                Err(_err) => {
                    #[cfg(feature = "log")]
                    log::error!("{_err}");
                    return;
                }
            };
            let client = Rc::new(wrapper);
            while let Some(request) = request_rx.next().await {
                let client = client.clone();
                spawn_local(async move {
                    let _resp = request
                        .tx
                        .send(client.request(request.req).await.map_err(HttpError::from));
                    #[cfg(feature = "log")]
                    if _resp.is_err() {
                        log::warn!("unable to send the actual result...");
                    }
                });
            }
        });
        Self { request_tx }
    }

    /// Check if the underlying
    pub fn is_closed(&self) -> bool {
        self.request_tx.is_closed()
    }

    /// Send an HTTP request without deserializing it.
    ///
    /// Note that this function return a [`serde_json::Value`] instead of the [`wasm_bindgen::JsValue`].
    /// This is because the [`JsValue`](wasm_bindgen::JsValue) isn't `thread-safe`, but [`serde_json::Value`] is.
    pub async fn send_request(&self, req: HttpOptions) -> Result<Response<Value>, HttpError> {
        let (rq, rx) = make_request(req);
        self.request_tx
            .unbounded_send(rq)
            .map_err(|err| err.into_send_error())?;
        rx.await?
    }

    /// Send an HTTP request.
    pub async fn request<T: DeserializeOwned>(
        &self,
        req: HttpOptions,
    ) -> Result<Response<T>, HttpError> {
        let response = self.send_request(req).await?;
        Ok(deserialize_response_data(response)?)
    }

    /// Perform a `GET` request
    pub async fn get<T: DeserializeOwned>(
        &self,
        url: String,
        options: RequestOptions,
    ) -> Result<Response<T>, HttpError> {
        self.request(HttpOptions {
            body: options.body,
            headers: options.headers,
            method: super::HttpVerb::Get,
            query: options.query,
            response_type: options.response_type,
            timeout: options.timeout,
            url,
        })
        .await
    }

    /// Perform a `DELTE` request
    pub async fn delete<T: DeserializeOwned>(
        &self,
        url: String,
        options: RequestOptions,
    ) -> Result<Response<T>, HttpError> {
        self.request(HttpOptions {
            body: options.body,
            headers: options.headers,
            method: super::HttpVerb::Delete,
            query: options.query,
            response_type: options.response_type,
            timeout: options.timeout,
            url,
        })
        .await
    }

    /// Perform a `PATCH` request
    pub async fn patch<T: DeserializeOwned>(
        &self,
        url: String,
        options: RequestOptions,
    ) -> Result<Response<T>, HttpError> {
        self.request(HttpOptions {
            body: options.body,
            headers: options.headers,
            method: super::HttpVerb::Patch,
            query: options.query,
            response_type: options.response_type,
            timeout: options.timeout,
            url,
        })
        .await
    }

    /// Perform a `POST` request
    pub async fn post<T: DeserializeOwned, B: Into<Body>>(
        &self,
        url: String,
        body: B,
        options: RequestOptions,
    ) -> Result<Response<T>, HttpError> {
        self.request(HttpOptions {
            body: Some(body.into()),
            headers: options.headers,
            method: super::HttpVerb::Post,
            query: options.query,
            response_type: options.response_type,
            timeout: options.timeout,
            url,
        })
        .await
    }

    /// Perform a `PUT` request
    pub async fn put<T: DeserializeOwned, B: Into<Body>>(
        &self,
        url: String,
        body: B,
        options: RequestOptions,
    ) -> Result<Response<T>, HttpError> {
        self.request(HttpOptions {
            body: Some(body.into()),
            headers: options.headers,
            method: super::HttpVerb::Put,
            query: options.query,
            response_type: options.response_type,
            timeout: options.timeout,
            url,
        })
        .await
    }
}

pub async fn fetch<T: DeserializeOwned>(
    url: String,
    options: FetchOptions,
) -> Result<Response<T>, HttpError> {
    let client = Client::default();
    client
        .request(HttpOptions {
            body: options.body,
            headers: options.headers,
            method: options.method,
            query: options.query,
            response_type: options.response_type,
            timeout: options.timeout,
            url,
        })
        .await
}
