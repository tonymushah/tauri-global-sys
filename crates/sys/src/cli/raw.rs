use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","cli"])]
extern "C" {
    /// Parse the arguments provided to the current process and get the matches using the configuration defined [tauri.cli](https://tauri.app/v1/api/config/#tauriconfig.cli) in tauri.conf.json
    #[wasm_bindgen(catch)]
    pub async fn getMatches() -> Result<JsValue, JsValue>;
}
