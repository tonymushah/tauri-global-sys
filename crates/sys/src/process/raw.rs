use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "path"])]
extern "C" {

    /// Exits immediately with the given `exitCode`.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/process#exit>
    #[wasm_bindgen(catch)]
    pub async fn exit(exitCode: i32) -> Result<(), JsValue>;

    /// Exits the current instance of the app then relaunches it.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/process#relaunch>
    #[wasm_bindgen(catch)]
    pub async fn relaunch() -> Result<(), JsValue>;
}
