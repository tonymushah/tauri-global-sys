use js_sys::JsString;
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

    /// Returns the directory name of a path.
    /// Trailing directory separators are ignored.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/path#dirname>
    #[wasm_bindgen(catch)]
    pub async fn dirname(dir: &str) -> Result<JsString, JsValue>;

    /// Returns the extension of the `path`.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/path#extname>
    #[wasm_bindgen(catch)]
    pub async fn extname(path: &str) -> Result<JsString, JsValue>;
}
