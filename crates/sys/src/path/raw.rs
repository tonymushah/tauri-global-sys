use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "path"])]
extern "C" {
    /// Provides the platform-specific path segment delimiter:
    /// - `;` on Windows
    /// - `:` on POSIX
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/path#delimiter>
    #[wasm_bindgen(thread_local_v2, js_name = "delimiter")]
    pub static DELIMITER: String;

    /// Provides the platform-specific path segment separator:
    /// - `\` on Windows
    /// - `/` on POSIX
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/path#sep>
    #[wasm_bindgen(thread_local_v2, js_name = "sep")]
    pub static SEP: String;
}
