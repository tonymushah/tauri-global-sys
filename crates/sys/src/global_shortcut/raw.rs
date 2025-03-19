use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","globalShortcut"])]
extern "C" {
    /// Ref: <https://v1.tauri.app/v1/api/js/globalShortcut#isregistered>
    #[wasm_bindgen(catch)]
    pub async fn isRegistered(shortcut: &str) -> Result<JsValue, JsValue>;

    /// Ref: <https://v1.tauri.app/v1/api/js/globalShortcut#register>
    #[wasm_bindgen(catch)]
    pub async fn register(
        shortcut: &str,
        handler: &Closure<dyn FnMut(String)>,
    ) -> Result<(), JsValue>;

    /// Ref: <https://v1.tauri.app/v1/api/js/globalShortcut#registerall>
    #[wasm_bindgen(catch)]
    pub async fn registerAll(
        shortcuts: JsValue,
        handler: &Closure<dyn FnMut(String)>,
    ) -> Result<(), JsValue>;

    /// Ref: <https://v1.tauri.app/http:/v1/api/js/globalShortcut#unregister>
    #[wasm_bindgen(catch)]
    pub async fn unregister(shortcut: &str) -> Result<(), JsValue>;

    /// Ref: <https://v1.tauri.app/v1/api/js/globalShortcut#unregisterall>
    #[wasm_bindgen(catch)]
    pub async fn unregisterAll() -> Result<(), JsValue>;
}
