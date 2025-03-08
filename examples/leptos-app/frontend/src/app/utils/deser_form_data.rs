use js_sys::Object;
use serde::de::DeserializeOwned;
use web_sys::FormData;

pub fn deser_form_data<T>(form_data: &FormData) -> Result<T, serde_wasm_bindgen::Error>
where
    T: DeserializeOwned,
{
    serde_wasm_bindgen::from_value(Object::from_entries(form_data)?.into())
}
