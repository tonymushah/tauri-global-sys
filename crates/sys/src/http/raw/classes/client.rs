use js_sys::Number;
use wasm_bindgen::prelude::*;

use super::{response::Response, RawBody as Body};

#[wasm_bindgen(js_namespace = ["window","__TAURI__","http"])]
extern "C" {
    /// Ref: <http://v1.tauri.app/v1/api/js/http#client>
    #[derive(Clone, Debug)]
    pub type Client;

    /// Client id
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#id>
    #[wasm_bindgen(method, js_class = Client, getter)]
    pub fn id(this: &Client) -> Number;

    /// Makes a DELETE request.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#delete>
    #[wasm_bindgen(method, js_class = Client, catch)]
    pub async fn delete(this: &Client, url: &str, options: JsValue) -> Result<Response, JsValue>;

    /// Drops the client instance.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#delete>
    #[wasm_bindgen(method, js_class = Client, catch)]
    pub async fn drop(this: &Client) -> Result<(), JsValue>;

    /// Makes a GET request.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#get>
    #[wasm_bindgen(method, js_class = Client, catch)]
    pub async fn get(this: &Client, url: &str, options: JsValue) -> Result<Response, JsValue>;

    /// Makes a PATCH request.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#patch>
    #[wasm_bindgen(method, js_class = Client, catch)]
    pub async fn patch(this: &Client, url: &str, options: JsValue) -> Result<Response, JsValue>;

    /// Makes a POST request.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#post>
    #[wasm_bindgen(method, js_class = Client, catch)]
    pub async fn post(
        this: &Client,
        url: &str,
        body: Body,
        options: JsValue,
    ) -> Result<Response, JsValue>;

    /// Makes a PuT request.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#put>
    #[wasm_bindgen(method, js_class = Client, catch)]
    pub async fn put(
        this: &Client,
        url: &str,
        body: Body,
        options: JsValue,
    ) -> Result<Response, JsValue>;

    /// Makes an HTTP request.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#request>
    #[wasm_bindgen(method, js_class = Client, catch)]
    pub async fn request(this: &Client, options: JsValue) -> Result<Response, JsValue>;
}
