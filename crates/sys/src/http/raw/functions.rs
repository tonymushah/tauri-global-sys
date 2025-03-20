use wasm_bindgen::prelude::*;

use super::classes::{RawClient, RawResponse};

#[wasm_bindgen(js_namespace = ["window","__TAURI__","http"])]
extern "C" {
    /// Creates a new client using the specified options.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#getclient>
    #[wasm_bindgen(catch, js_name = getClient)]
    pub async fn getRawClient(options: JsValue) -> Result<RawClient, JsValue>;

    /// Perform an HTTP request using the default client.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#fetch>
    #[wasm_bindgen(catch, js_name = fetch)]
    pub async fn rawFetch(url: &str, options: JsValue) -> Result<RawResponse, JsValue>;
}
