use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
extern "C" {
    /// _Raw binding for `https://v1.tauri.app/v1/api/js/dialog/#ask`_
    ///
    /// Shows a question dialog with Yes and No buttons.
    ///
    /// Since: 1.0.0
    #[wasm_bindgen(catch)]
    pub async fn ask(message: &str, options: JsValue) -> Result<JsValue, JsValue>;

    /// _Raw binding for `https://v1.tauri.app/v1/api/js/dialog/#confirm`_
    ///
    /// Shows a question dialog with Ok and Cancel buttons.
    ///
    /// Since: 1.0.0
    #[wasm_bindgen(catch)]
    pub async fn confirm(message: &str, options: JsValue) -> Result<JsValue, JsValue>;

    /// _Raw binding for `https://v1.tauri.app/v1/api/js/dialog/#message`_
    ///
    /// Shows a message dialog with an Ok button.
    ///
    /// Since: 1.0.0
    #[wasm_bindgen(catch)]
    pub async fn message(message: &str, options: JsValue) -> Result<(), JsValue>;

    /// _Raw binding for `https://v1.tauri.app/v1/api/js/dialog/#open`_
    ///
    /// Open a file/directory selection dialog.
    ///
    /// The selected paths are added to the filesystem and asset protocol allowlist scopes.
    /// When security is more important than the easy of use of this API, prefer writing a dedicated command instead.
    ///
    /// Note that the allowlist scope change is not persisted,
    /// so the values are cleared when the application is restarted.
    /// You can save it to the filesystem using [`tauri-plugin-persisted-scope`](https://github.com/tauri-apps/plugins-workspace/tree/v1/plugins/persisted-scope).
    ///
    /// Since: 1.0.0
    ///
    #[wasm_bindgen(catch)]
    pub async fn open(options: JsValue) -> Result<JsValue, JsValue>;

    /// _Raw binding for `https://v1.tauri.app/v1/api/js/dialog/#save`_
    ///
    /// Open a file/directory save dialog.
    ///
    /// The selected path is added to the filesystem and asset protocol allowlist scopes.
    /// When security is more important than the easy of use of this API, prefer writing a dedicated command instead.
    ///
    /// Note that the allowlist scope change is not persisted, so the values are cleared when the application is restarted.
    /// You can save it to the filesystem using [`tauri-plugin-persisted-scope`](https://github.com/tauri-apps/plugins-workspace/tree/v1/plugins/persisted-scope).
    ///
    /// Since: 1.0.0
    ///
    #[wasm_bindgen(catch)]
    pub async fn save(options: JsValue) -> Result<JsValue, JsValue>;
}
