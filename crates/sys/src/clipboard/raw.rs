use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","clipboard"])]
extern "C" {

    /// Gets the clipboard content as plain text.
    ///
    /// Since v1.0.0
    #[wasm_bindgen(catch)]
    pub async fn readText() -> Result<JsValue, JsValue>;

    /// Writes plain text to the clipboard.
    ///
    /// Since v1.0.0
    #[wasm_bindgen(catch)]
    pub async fn writeText(text: &str) -> Result<(), JsValue>;
}
