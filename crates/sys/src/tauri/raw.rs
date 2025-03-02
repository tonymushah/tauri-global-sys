use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
extern "C" {

    /// Sends a message to the backend.
    ///
    /// Since Tauri 1.0.0
    #[wasm_bindgen(catch)]
    pub async fn invoke(command: &str, args: JsValue) -> Result<JsValue, JsValue>;

    /// Convert a device file path to an URL that can be loaded by the webview.
    /// Note that asset: and https://asset.localhost must be added to tauri.security.csp in tauri.conf.json.
    /// Example CSP value: "csp": "default-src 'self'; img-src 'self' asset: https://asset.localhost" to use the asset protocol on image sources.
    ///
    /// Additionally, asset must be added to tauri.allowlist.protocol in tauri.conf.json and its access scope must be defined on the assetScope array on the same protocol object.
    ///
    /// More details to https://v1.tauri.app/v1/api/js/tauri#convertfilesrc
    ///
    /// Since Tauri 1.0.0
    pub fn convertFileSrc(file_path: &str, protocol: Option<&str>) -> String;

    /// Transforms a callback function to a string identifier that can be passed to the backend.
    /// The backend uses the identifier to `eval()` the callback.
    ///
    /// Since Tauri 1.0.0
    pub fn transformCallback(callback: &mut dyn FnMut(JsValue), once: Option<bool>) -> usize;
}
