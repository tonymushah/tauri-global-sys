use wasm_bindgen::prelude::*;

pub mod tauri_event {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event", "TauriEvent"])]
    extern "C" {
        #[wasm_bindgen(thread_local_v2)]
        pub static CHECK_UPDATE: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static DOWNLOAD_PROGRESS: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static INSTALL_UPDATE: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static MENU: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static STATUS_UPDATE: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static UPDATE_AVAILABLE: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_BLUR: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_CLOSE_REQUESTED: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_CREATED: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_DESTROYED: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_FILE_DROP: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_FILE_DROP_CANCELLED: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_FILE_DROP_HOVER: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_FOCUS: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_MOVED: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_RESIZED: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_SCALE_FACTOR_CHANGED: String;

        #[wasm_bindgen(thread_local_v2)]
        pub static WINDOW_THEME_CHANGED: String;
    }
}

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
extern "C" {

    /// Emits an event to the backend and all Tauri windows.
    ///
    /// Since 1.0.0
    pub async fn emit(event: &str, payload: &JsValue);

    /// Listen to an event. The event can be either global or window-specific.
    pub async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> js_sys::Function;

    // Listen to an event once.
    pub async fn once(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> js_sys::Function;
}
