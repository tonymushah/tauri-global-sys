use js_sys::JsString;
use serde::de::DeserializeOwned;
use wasm_bindgen::{JsCast, JsValue};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Tauri(String),
    #[error(transparent)]
    SerdeWasm(#[from] serde_wasm_bindgen::Error),
    #[error("Cannot convert a JsString to an Rust string")]
    JsStringToString,
    #[error("invalid type expected {0}")]
    InvalidType(String),
    #[error("invoke error: {:?}", .0)]
    Invoke(JsValue),
    #[error(transparent)]
    FuturesOneshotCanceled(#[from] futures::channel::oneshot::Canceled),
    #[error("{}", js_sys::Error::message(.0))]
    Js(js_sys::Error),
}

impl Error {
    pub(crate) fn tauri(message: String) -> Self {
        Self::Tauri(message)
    }
    pub(crate) fn tauri_js_string_ref(message: &JsString) -> Self {
        if let Some(message) = message.as_string() {
            Self::tauri(message)
        } else {
            Self::JsStringToString
        }
    }
    pub(crate) fn tauri_js_string(message: JsString) -> Self {
        Self::tauri_js_string_ref(&message)
    }
    pub fn parse_invoke_error<O>(&self) -> Option<O>
    where
        O: DeserializeOwned,
    {
        if let Self::Invoke(value) = self {
            serde_wasm_bindgen::from_value(value.clone()).ok()
        } else {
            None
        }
    }
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        match value.dyn_into::<JsString>() {
            Ok(message) => Self::tauri_js_string(message),
            Err(other) => match other.dyn_into::<js_sys::Error>() {
                Ok(error) => Self::Js(error),
                Err(other) => Self::Invoke(other),
            },
        }
    }
}
