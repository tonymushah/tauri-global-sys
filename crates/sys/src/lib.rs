use js_sys::JsString;

pub mod app;

#[derive(Debug, thiserror::Error)]
pub enum Error{
    #[error("{0}")]
    Tauri(String),
    #[error(transparent)]
    SerdeWasm(#[from] serde_wasm_bindgen::Error),
    #[error("Cannot convert a JsString to an Rust string")]
    JsStringToString,
    #[error("invalid type expected {0}")]
    InvalidType(String)
}

impl Error {
    pub(crate) fn tauri(message: String) -> Self {
        Self::Tauri(message)
    }
    pub(crate) fn tauri_js_string_ref(message: &JsString) -> Self {
        if let Some(message) = message.as_string() {
            Self::tauri(message)
        }else {
            Self::JsStringToString
        }
    }
    pub(crate) fn tauri_js_string(message: JsString) -> Self {
        Self::tauri_js_string_ref(&message)
    }
}