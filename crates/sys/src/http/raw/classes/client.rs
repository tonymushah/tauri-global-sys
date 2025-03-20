use js_sys::Number;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","http"])]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Client;

    /// Client id
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#id>
    #[wasm_bindgen(method, js_class = Client, getter)]
    pub fn id(this: &Client) -> Number;

}
