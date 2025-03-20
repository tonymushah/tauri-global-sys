use serde_wasm_bindgen::to_value as to_js_value;
use wasm_bindgen::prelude::*;
use web_sys::FormData;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","http"])]
extern "C" {
    /// The body object to be used on POST and PUT requests.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#body>
    #[derive(Clone, Debug)]
    pub type Body;

    /// Getter for the [`Body.payload`](http://v1.tauri.app/v1/api/js/http#payload) property.
    #[wasm_bindgen(method, getter, js_class = Body, js_name = payload)]
    pub fn payload(this: &Body) -> JsValue;

    /// Setter for the [`Body.payload`](http://v1.tauri.app/v1/api/js/http#payload) property.
    #[wasm_bindgen(method, setter, js_class = Body, js_name = payload)]
    pub fn set_payload(this: &Body, payload: JsValue);

    /// Getter for the [`Body.type`](http://v1.tauri.app/v1/api/js/http#type) property.
    #[wasm_bindgen(method, getter, js_class = Body, js_name = type)]
    pub fn type_(this: &Body) -> String;

    /// Setter for the [`Body.type`](http://v1.tauri.app/v1/api/js/http#type) property.
    #[wasm_bindgen(method, setter, js_class = Body, js_name = type)]
    pub fn set_type(this: &Body, type_: &str);

    /// Create a new bytes array body.
    ///
    /// ```rust
    /// use tauri_global_sys::http::raw::classes::body::Body;
    /// use serde_wasm_bindgen::to_value;
    ///
    /// let bytes: Vec<u8> = vec![1, 2, 3];
    ///
    /// let _body = Body::bytes(to_value(&bytes).unwrap());
    ///
    /// ```
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#bytes>
    #[wasm_bindgen(static_method_of = Body)]
    pub fn bytes(bytes: JsValue) -> Body;

    /// Creates a new form data body. The form data is an object where each key is the entry name, and the value is either a string or a file object.
    ///
    /// By default it sets the `application/x-www-form-urlencoded` Content-Type header, but you can set it to `multipart/form-data` if the **Tauri** Cargo feature `http-multipart` is enabled.
    ///
    /// Note that a file path must be allowed in the fs allowlist scope.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/http#form>
    #[wasm_bindgen(static_method_of = Body)]
    pub fn form(data: JsValue) -> Body;

    /// Creates a new UTF-8 string body.
    ///    
    /// Ref: <http://v1.tauri.app/v1/api/js/http#text>
    #[wasm_bindgen(static_method_of = Body)]
    pub fn text(value: &str) -> Body;
}

impl Body {
    /// Same as [`Body::bytes`] but only accept a `&[u8]` instead.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Body> {
        Ok(Self::bytes(to_js_value(bytes)?))
    }
    /// Same as [`Body::form`] but only accept a [`web_sys::FormData`].
    pub fn from_form_data(data: FormData) -> Body {
        Self::form(data.into())
    }
}
