use std::future::Future;

use futures::{
    channel::{mpsc, oneshot},
    stream::Stream,
    FutureExt, StreamExt,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod raw;
pub mod tauri_events;

#[derive(Debug, Deserialize, Clone)]
pub struct Event<T> {
    pub event: String,
    pub id: usize,
    pub payload: T,
    #[serde(alias = "windowLabel")]
    pub window_label: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EventJsCasted<T>
where
    T: JsCast,
{
    pub event: String,
    pub id: usize,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub payload: T,
    #[serde(alias = "windowLabel")]
    pub window_label: Option<String>,
}

pub async fn emit<P: Serialize>(event: &str, payload: &P) -> Result<(), crate::Error> {
    let value = serde_wasm_bindgen::to_value(payload)?;
    raw::emit(event, &value).await?;
    Ok(())
}

/// An event listener stream
pub struct Listen<T> {
    receiver: mpsc::UnboundedReceiver<Event<T>>,
    unlisten: js_sys::Function,
    _closure: Closure<dyn FnMut(JsValue)>,
}

impl<T> Drop for Listen<T> {
    fn drop(&mut self) {
        #[cfg(feature = "log")]
        log::trace!("Calling unlisten for event listener stream");
        let _res = self.unlisten.call0(&JsValue::undefined());
        #[cfg(feature = "log")]
        {
            match _res {
                Ok(resp) => log::debug!("{:?}", resp),
                Err(err) => log::error!("{:#?}", err),
            }
        }
    }
}

impl<T> Stream for Listen<T> {
    type Item = Event<T>;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.receiver.poll_next_unpin(cx)
    }
}

pub async fn listen<T>(event: &str) -> crate::Result<Listen<T>>
where
    T: DeserializeOwned + 'static,
{
    let (tx, rx) = mpsc::unbounded::<Event<T>>();
    let handler = Closure::new(move |event: JsValue| {
        match serde_wasm_bindgen::from_value::<Event<T>>(event) {
            Ok(value) => {
                let _res = tx.unbounded_send(value);
                #[cfg(feature = "log")]
                if let Err(err) = _res {
                    log::error!("{err}");
                }
            }
            Err(_err) => {
                #[cfg(feature = "log")]
                log::error!("{_err}");
            }
        }
    });
    let unlisten = raw::listen(event, &handler).await?;

    Ok(Listen {
        receiver: rx,
        unlisten,
        _closure: handler,
    })
}

pub struct Once<T> {
    rx: oneshot::Receiver<Event<T>>,
    unlisten: js_sys::Function,
    _closure: Closure<dyn FnMut(JsValue)>,
}

impl<T> Drop for Once<T> {
    fn drop(&mut self) {
        #[cfg(feature = "log")]
        log::trace!("Calling unlisten for once event");
        let _res = self.unlisten.call0(&JsValue::undefined());
        #[cfg(feature = "log")]
        {
            match _res {
                Ok(resp) => log::debug!("{:?}", resp),
                Err(err) => log::error!("{:#?}", err),
            }
        }
    }
}

impl<T> Future for Once<T> {
    type Output = crate::Result<Event<T>>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(crate::Error::from)
    }
}

impl<T> Once<T>
where
    T: DeserializeOwned + 'static,
{
    pub async fn new(event: &str) -> crate::Result<Self> {
        let (tx, rx) = oneshot::channel::<Event<T>>();
        let handler = Closure::once(move |event: JsValue| {
            match serde_wasm_bindgen::from_value::<Event<T>>(event) {
                Ok(value) => {
                    let _res = tx.send(value);
                    #[cfg(feature = "log")]
                    if _res.is_err() {
                        log::error!("Droped Receiver");
                    }
                }
                Err(_err) => {
                    #[cfg(feature = "log")]
                    log::error!("{_err}");
                }
            }
        });
        let unlisten = raw::once(event, &handler).await?;
        Ok(Self {
            rx,
            unlisten,
            _closure: handler,
        })
    }
}

pub async fn once<T>(event: &str) -> crate::Result<Event<T>>
where
    T: DeserializeOwned + 'static,
{
    Once::<T>::new(event).await?.await
}
