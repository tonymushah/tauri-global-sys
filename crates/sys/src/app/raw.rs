use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","app"])]
extern "C" {
    /// Gets the application name.
    ///
    /// Usable since Tauri 1.0.0
    #[wasm_bindgen(js_name = getName, catch)]
    pub async fn get_name() -> Result<js_sys::JsString, JsValue>;

    /// Get the Tauri Version.
    ///
    /// Usable since Tauri 1.0.0
    #[wasm_bindgen(js_name = getTauriVersion, catch)]
    pub async fn get_tauri_version() -> Result<js_sys::JsString, JsValue>;

    /// Gets the application version.
    ///
    /// Usable since Tauri 1.0.0
    #[wasm_bindgen(js_name = getVersion, catch)]
    pub async fn get_version() -> Result<js_sys::JsString, JsValue>;

    /// Hides the application on macOS.
    ///
    /// Usable Tauri 1.2.0
    ///
    /// Require `tauri.allowlist.app.hide` set to `true`
    #[wasm_bindgen(catch)]
    pub async fn hide() -> Result<(), JsValue>;

    /// Show the application on macOS.
    ///
    /// Usable Tauri 1.2.0
    ///
    /// Require `tauri.allowlist.app.show` set to `true`
    #[wasm_bindgen(catch)]
    pub async fn show() -> Result<(), JsValue>;

}
