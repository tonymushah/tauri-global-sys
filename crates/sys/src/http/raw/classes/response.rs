use js_sys::{Boolean, Number};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","http"])]
extern "C" {
    /// Response object
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#responset>
    #[derive(Clone, Debug)]
    pub type Response;

    /// The response data.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#data>
    #[wasm_bindgen(method, js_class = Response, getter)]
    pub fn data(this: &Response) -> JsValue;

    /// The response headers.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#headers>
    #[wasm_bindgen(method, js_class = Response, getter)]
    pub fn headers(this: &Response) -> JsValue;

    /// A boolean indicating whether the response was successful (status in the range 200–299) or not.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#ok>
    #[wasm_bindgen(method, js_class = Response, getter)]
    pub fn ok(this: &Response) -> Boolean;

    /// The response raw headers.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#rawheaders>
    #[wasm_bindgen(method, js_class = Response, getter)]
    pub fn rawHeaders(this: &Response) -> JsValue;

    /// The response status code.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#status>
    #[wasm_bindgen(method, js_class = Response, getter)]
    pub fn status(this: &Response) -> Number;

    /// The request URL.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#url>
    #[wasm_bindgen(method, js_class = Response, getter)]
    pub fn url(this: &Response) -> String;
}
