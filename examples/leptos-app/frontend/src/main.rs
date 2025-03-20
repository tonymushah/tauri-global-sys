mod app;

use app::*;
use js_sys::Function;
use leptos::{prelude::*, tachys::dom::body};
use wasm_bindgen::{prelude::Closure, JsCast};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).unwrap();
    let handle = mount_to(body(), || {
        view! { <App /> }
    });
    let listener = Closure::once_into_js(|| {
        drop(handle);
    })
    .dyn_into::<Function>()
    .unwrap();
    let _ = window().add_event_listener_with_callback("beforeunload", &listener);
}
