use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// `wasm-bindgen` raw bindings
pub mod raw;

/// Send a message to the backend
pub async fn invoke<I, O>(command: &str, args: &I) -> Result<O, crate::Error>
where
    I: Serialize,
    O: for<'a> Deserialize<'a>,
{
    match raw::invoke(command, serde_wasm_bindgen::to_value(args)?).await {
        Ok(result) => serde_wasm_bindgen::from_value(result).map_err(crate::Error::from),
        Err(error) => {
            if let Some(err) = error.as_string() {
                Err(crate::Error::tauri(err))
            } else {
                Err(crate::Error::Invoke(error))
            }
        }
    }
}

/// Convert a device file path to an URL that can be loaded by the webview.
pub fn convert_file_src(file_path: &str, protocol: Option<&str>) -> String {
    raw::convertFileSrc(file_path, protocol)
}

/// Transforms a callback function to a string identifier that can be passed to the backend.
pub fn transform_callback<C>(callback: &mut C) -> usize
where
    C: FnMut(JsValue),
{
    raw::transformCallback(callback, None)
}

/// Transforms a callback function to a string identifier that can be passed to the backend but can only be called once.
pub fn transform_callback_once<C>(callback: &mut C) -> usize
where
    C: FnMut(JsValue),
{
    raw::transformCallback(callback, Some(true))
}
