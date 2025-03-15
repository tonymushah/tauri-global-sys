use wasm_bindgen::JsCast;
use web_sys::{Event, FormData, HtmlFormElement};

pub fn extract_form_data(ev: &Event) -> FormData {
    let form = ev
        .target()
        .and_then(|t| t.dyn_into::<HtmlFormElement>().ok())
        .expect("Should be a form element");
    FormData::new_with_form(&form).expect("Cannot make a form data")
}
