use std::collections::HashMap;

use classes::{RawBody, RawClient, RawResponse};
use js_sys::{JsString, Map as JsMap, Object, Uint8Array};
use serde::{de::DeserializeOwned, Deserialize, Serialize, Serializer};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use super::{HttpVerb, ResponseType};

pub mod classes;
pub mod functions;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum ConnectTimout {
    Number(f32),
    Duration { nanos: u64, secs: u64 },
}

impl From<(u64, u64)> for ConnectTimout {
    fn from((secs, nanos): (u64, u64)) -> Self {
        Self::Duration { nanos, secs }
    }
}

impl From<f32> for ConnectTimout {
    fn from(value: f32) -> Self {
        Self::Number(value)
    }
}

/// Ref: <http://v1.tauri.app/v1/api/js/http#clientoptions>
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redirection: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timout: Option<ConnectTimout>,
}

/// Get a [`RawClient`] with a specified [`ClientsOptions`].
pub async fn get_raw_client(options: Option<ClientsOptions>) -> crate::Result<RawClient> {
    Ok(functions::getRawClient(serde_wasm_bindgen::to_value(&options)?).await?)
}

/// Get a [`RawClientWrapper`] with a specified [`ClientsOptions`].
pub async fn get_raw_client_wrapper(
    options: Option<ClientsOptions>,
) -> crate::Result<RawClientWrapper> {
    Ok(get_raw_client(options).await?.into())
}

/// A `safe` and [`Send`]/[`Sync`] abstraction for the [`RawResponse`] class.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response<T> {
    pub data: T,
    pub headers: HashMap<String, String>,
    pub ok: bool,
    pub raw_headers: HashMap<String, Vec<String>>,
    pub status: u16,
    pub url: String,
}

impl<T: DeserializeOwned> TryFrom<RawResponse> for Response<T> {
    type Error = serde_wasm_bindgen::Error;
    fn try_from(value: RawResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            data: serde_wasm_bindgen::from_value(value.data())?,
            headers: serde_wasm_bindgen::from_value(value.headers())?,
            ok: value.ok().value_of(),
            raw_headers: serde_wasm_bindgen::from_value(value.rawHeaders())?,
            status: serde_wasm_bindgen::from_value(value.status().into())?,
            url: value.url(),
        })
    }
}

impl<T: DeserializeOwned> Response<T> {
    pub fn parse_response(resp: RawResponse) -> Result<Self, serde_wasm_bindgen::Error> {
        resp.try_into()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum FilePartFile<T> {
    Path(String),
    File(T),
}

/// Ref: <http://v1.tauri.app/v1/api/js/http#filepartt>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePart<T> {
    pub file: FilePartFile<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
}

impl FilePart<Vec<u8>> {
    pub fn into_js_value(&self) -> JsValue {
        let map = JsMap::new();
        map.set(
            &JsString::from("fileName"),
            &self
                .file_name
                .as_ref()
                .map(JsValue::from)
                .unwrap_or(JsValue::undefined()),
        );
        map.set(
            &JsString::from("mime"),
            &self
                .file_name
                .as_ref()
                .map(JsValue::from)
                .unwrap_or(JsValue::undefined()),
        );
        let file = match &self.file {
            FilePartFile::Path(s) => JsValue::from(s),
            FilePartFile::File(file) => u8_slice_to_uint_array(file).into(),
        };
        map.set(&JsString::from("file"), &file);
        Object::from_entries(&map).unwrap().into()
    }
}

/// Ref: <http://v1.tauri.app/v1/api/js/http#part>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Part {
    String(String),
    Bytes(Vec<u8>),
    File(FilePart<Vec<u8>>),
}

impl Part {
    pub fn into_js_value(&self) -> JsValue {
        match self {
            Part::String(s) => JsString::from(s.as_str()).into(),
            Part::Bytes(items) => u8_slice_to_uint_array(items).into(),
            Part::File(file_part) => file_part.into_js_value(),
        }
    }
}

impl From<String> for Part {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<u8>> for Part {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}

impl From<FilePart<Vec<u8>>> for Part {
    fn from(value: FilePart<Vec<u8>>) -> Self {
        Self::File(value)
    }
}

fn u8_slice_to_uint_array(bytes: &[u8]) -> Uint8Array {
    let array = Uint8Array::new_with_length(bytes.len() as u32);
    for (idx, byte) in bytes.iter().enumerate() {
        array.set_index(idx as u32, *byte);
    }
    array
}

fn body_form_to_object(map: &HashMap<String, Part>) -> Object {
    let js_map = JsMap::new();
    for (key, part) in map {
        js_map.set(&key.into(), &part.into_js_value());
    }
    Object::from_entries(&js_map).unwrap()
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Body {
    Bytes(Vec<u8>),
    Form(HashMap<String, Part>),
    Text(String),
}

impl Default for Body {
    fn default() -> Self {
        Body::Bytes(Default::default())
    }
}

impl Body {
    pub fn into_raw(&self) -> RawBody {
        match self {
            Body::Bytes(items) => RawBody::bytes(u8_slice_to_uint_array(items).into()),
            Body::Form(hash_map) => RawBody::form(body_form_to_object(hash_map).into()),
            Body::Text(text) => RawBody::text(text),
        }
    }
}

impl From<Body> for RawBody {
    fn from(value: Body) -> Self {
        value.into_raw()
    }
}

fn js_ser_body<S: Serializer>(body: &Option<Body>, serializer: S) -> Result<S::Ok, S::Error> {
    let body: JsValue = match body {
        Some(inner) => inner.into_raw().into(),
        None => JsValue::undefined(),
    };
    serde_wasm_bindgen::preserve::serialize(&body, serializer)
}

/// Ref: <http://v1.tauri.app/v1/api/js/http#httpoptions>
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HttpOptions {
    #[serde(serialize_with = "js_ser_body")]
    pub body: Option<Body>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
    pub method: HttpVerb,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub query: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<ResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ConnectTimout>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub url: String,
}

/// Ref: <http://v1.tauri.app/v1/api/js/http#fetchoptions>
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchOptions {
    #[serde(serialize_with = "js_ser_body")]
    pub body: Option<Body>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
    pub method: HttpVerb,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub query: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<ResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ConnectTimout>,
}

/// Ref: <http://v1.tauri.app/v1/api/js/http#requestoptions>
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestOptions {
    #[serde(serialize_with = "js_ser_body")]
    pub body: Option<Body>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub query: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<ResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ConnectTimout>,
}

/// A [`RawClient`] wrapper.
///
/// Unlike the [`RawClient`] where you need to drop it manually with [`drop`](RawClient::drop),
/// this wrapper will automatically drop the underlaying client when it is goes out-of-scope.
#[derive(Debug)]
pub struct RawClientWrapper {
    pub(in crate::http) inner: Option<RawClient>,
}

impl RawClientWrapper {
    pub fn id(&self) -> Option<u64> {
        self.inner
            .as_ref()
            .and_then(|d| d.id().as_f64())
            .map(|d| d as u64)
    }
    fn inner(&self) -> crate::Result<&RawClient> {
        self.inner
            .as_ref()
            .ok_or(crate::Error::RawClientWrapperInnerEmpty)
    }
    /// Perform a `DELETE` request with the actual client.
    pub async fn raw_delete(
        &self,
        url: &str,
        option: RequestOptions,
    ) -> crate::Result<RawResponse> {
        let inner = self.inner()?;
        Ok(inner
            .delete(url, serde_wasm_bindgen::to_value(&option)?)
            .await?)
    }
    /// Perform a `DELETE` request
    /// but unlike [`raw_delete`](Self::raw_delete), this allows you to deserialize the response data.
    pub async fn delete<T: DeserializeOwned>(
        &self,
        url: &str,
        option: RequestOptions,
    ) -> crate::Result<Response<T>> {
        let raw = self.raw_delete(url, option).await?;
        Ok(raw.try_into()?)
    }

    /// Perform a `GET` request with the actual client.
    pub async fn raw_get(&self, url: &str, option: RequestOptions) -> crate::Result<RawResponse> {
        let inner = self.inner()?;
        Ok(inner
            .get(url, serde_wasm_bindgen::to_value(&option)?)
            .await?)
    }
    /// Perform a `GET` request
    /// but unlike [`raw_get`](Self::raw_get), this allows you to deserialize the response data.
    pub async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        option: RequestOptions,
    ) -> crate::Result<Response<T>> {
        let raw = self.raw_get(url, option).await?;
        Ok(raw.try_into()?)
    }

    /// Perform a `PATCH` request with the actual client.
    pub async fn raw_patch(&self, url: &str, option: RequestOptions) -> crate::Result<RawResponse> {
        let inner = self.inner()?;
        Ok(inner
            .patch(url, serde_wasm_bindgen::to_value(&option)?)
            .await?)
    }
    /// Perform a `PATCH` request
    /// but unlike [`raw_patch`](Self::raw_patch), this allows you to deserialize the response data.
    pub async fn patch<T: DeserializeOwned>(
        &self,
        url: &str,
        option: RequestOptions,
    ) -> crate::Result<Response<T>> {
        let raw = self.raw_patch(url, option).await?;
        Ok(raw.try_into()?)
    }

    /// Perform a `POST` request with the actual client.
    pub async fn raw_post<B>(
        &self,
        url: &str,
        body: B,
        option: RequestOptions,
    ) -> crate::Result<RawResponse>
    where
        B: Into<RawBody>,
    {
        let inner = self.inner()?;
        Ok(inner
            .post(url, body.into(), serde_wasm_bindgen::to_value(&option)?)
            .await?)
    }
    /// Perform a `POST` request
    /// but unlike [`raw_post`](Self::raw_post), this allows you to deserialize the response data.
    pub async fn post<T, B>(
        &self,
        url: &str,
        body: B,
        option: RequestOptions,
    ) -> crate::Result<Response<T>>
    where
        T: DeserializeOwned,
        B: Into<RawBody>,
    {
        let raw = self.raw_post(url, body, option).await?;
        Ok(raw.try_into()?)
    }

    /// Perform a `POST` request with the actual client.
    pub async fn raw_put<B>(
        &self,
        url: &str,
        body: B,
        option: RequestOptions,
    ) -> crate::Result<RawResponse>
    where
        B: Into<RawBody>,
    {
        let inner = self.inner()?;
        Ok(inner
            .put(url, body.into(), serde_wasm_bindgen::to_value(&option)?)
            .await?)
    }
    /// Perform a `PUT` request
    /// but unlike [`raw_put`](Self::raw_put), this allows you to deserialize the response data.
    pub async fn put<T, B>(
        &self,
        url: &str,
        body: B,
        option: RequestOptions,
    ) -> crate::Result<Response<T>>
    where
        T: DeserializeOwned,
        B: Into<RawBody>,
    {
        let raw = self.raw_put(url, body, option).await?;
        Ok(raw.try_into()?)
    }

    /// Perform an HTTP request with the actual client.
    pub async fn raw_request(&self, option: HttpOptions) -> crate::Result<RawResponse> {
        if option.url.is_empty() {
            return Err(crate::Error::tauri(
                "Invalid url: an empty url was given".into(),
            ));
        }
        let inner = self.inner()?;
        Ok(inner
            .request(serde_wasm_bindgen::to_value(&option)?)
            .await?)
    }

    /// Perform an HTTP request
    /// but unlike [`raw_request`](Self::raw_request), this allows you to deserialize the response data.
    pub async fn request<T: DeserializeOwned>(
        &self,
        option: HttpOptions,
    ) -> crate::Result<Response<T>> {
        let resp = self.raw_request(option).await?;
        Ok(resp.try_into()?)
    }
}

impl From<RawClient> for RawClientWrapper {
    fn from(value: RawClient) -> Self {
        Self { inner: Some(value) }
    }
}

impl Drop for RawClientWrapper {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            spawn_local(async move {
                let _re = inner.drop().await;
                #[cfg(feature = "log")]
                if let Err(err) = _re {
                    log::error!("Error on dropping client {:?} ({err:?})", err.as_string());
                }
            });
        } else {
            #[cfg(feature = "log")]
            log::warn!("Invalid logic: The inner client was already dropped");
        }
    }
}

/// Perform an HTTP request using the default client.
pub async fn raw_fetch(url: &str, options: FetchOptions) -> crate::Result<RawResponse> {
    Ok(functions::rawFetch(url, serde_wasm_bindgen::to_value(&options)?).await?)
}

/// Same as [`raw_fetch`] but this one allows you to deserialize the response data.
pub async fn fetch<T: DeserializeOwned>(
    url: &str,
    options: FetchOptions,
) -> crate::Result<Response<T>> {
    let resp = raw_fetch(url, options).await?;
    Ok(resp.try_into()?)
}

#[cfg(test)]
mod connect_timout {
    use serde_json::json;

    use super::ConnectTimout;

    #[test]
    fn ser_test_number() {
        let val = ConnectTimout::Number(2.0);

        assert_eq!(json!(2.0), serde_json::to_value(val).unwrap());
    }

    #[test]
    fn ser_test_duration() {
        let val = ConnectTimout::Duration {
            nanos: 234,
            secs: 5,
        };

        assert_eq!(
            json!({
                "nanos": 234,
                "secs": 5
            }),
            serde_json::to_value(val).unwrap()
        )
    }
}
