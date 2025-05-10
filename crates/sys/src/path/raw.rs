use js_sys::{Boolean, JsString};
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

    /// Returns whether the `path` is absolute or not.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/path#isabsolute>
    #[wasm_bindgen(catch)]
    pub async fn isAbsolute(path: &str) -> Result<Boolean, JsValue>;

    /// Joins all given `path` segments together
    /// using the platform-specific separator as a delimiter,
    /// then normalizes the resulting path.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/path#join>
    #[wasm_bindgen(catch)]
    pub async fn join(paths: JsValue) -> Result<JsString, JsValue>;

    /// Normalizes the given path, resolving `..` and `.` segments and resolve symbolic links.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/path#normalize>
    #[wasm_bindgen(catch)]
    pub async fn normalize(path: &str) -> Result<JsString, JsValue>;

    /// Resolve the path to a resource file.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/path#resolveresource>
    #[wasm_bindgen(catch)]
    pub async fn resolveResource(resource_path: &str) -> Result<JsString, JsValue>;

    /// Resolves a sequence of paths or path segments into an absolute path.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/path#resolve>
    #[wasm_bindgen(catch)]
    pub async fn resolve(paths: JsValue) -> Result<JsString, JsValue>;
}

macro_rules! dirs {
    ($($func_name:ident,)*) => {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "path"])]
        extern "C" {
            $(
                #[wasm_bindgen(catch)]
                pub async fn $func_name() -> Result<JsString, JsValue>;
            )*
        }
    };
}

dirs! {
    appCacheDir,
    appConfigDir,
    appDataDir,
    appDir,
    appLocalDataDir,
    appLogDir,
    audioDir,
    cacheDir,
    configDir,
    dataDir,
    desktopDir,
    documentDir,
    downloadDir,
    executableDir,
    fontDir,
    homeDir,
    localDataDir,
    logDir,
    pictureDir,
    publicDir,
    resourceDir,
    runtimeDir,
    templateDir,
    videoDir,
}
