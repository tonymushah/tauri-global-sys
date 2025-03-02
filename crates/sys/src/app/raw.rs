use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","app"])]
extern "C" {
    /// Gets the application name.
    ///
    /// Usable since Tauri 1.0.0
    #[wasm_bindgen(js_name = getName)]
    pub async fn get_name() -> js_sys::JsString;

    /// Get the Tauri Version.
    ///
    /// Usable since Tauri 1.0.0
    #[wasm_bindgen(js_name = getTauriVersion)]
    pub async fn get_tauri_version() -> js_sys::JsString;

    /// Gets the application version.
    ///
    /// Usable since Tauri 1.0.0
    #[wasm_bindgen(js_name = getVersion)]
    pub async fn get_version() -> js_sys::JsString;

    /// Hides the application on macOS.
    ///
    /// Usable Tauri 1.2.0
    ///
    /// Require `tauri.allowlist.app.hide` set to `true`
    #[wasm_bindgen(catch)]
    pub async fn hide() -> Result<(), js_sys::JsString>;

    /// Show the application on macOS.
    ///
    /// Usable Tauri 1.2.0
    ///
    /// Require `tauri.allowlist.app.show` set to `true`
    #[wasm_bindgen(catch)]
    pub async fn show() -> Result<(), js_sys::JsString>;

}
