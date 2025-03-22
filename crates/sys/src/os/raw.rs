use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","os"])]
extern "C" {
    #[wasm_bindgen(thread_local_v2)]
    pub static EOL: String;

    /// Returns the operating system CPU architecture for which the tauri app was compiled.
    /// Possible values are `x86`, `x86_64`, `arm`, `aarch64`, `mips`, `mips64`, `powerpc`, `powerpc64`, `riscv64`, `s390x`, `sparc64`.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/os#arch-1>
    #[wasm_bindgen(catch)]
    pub async fn arch() -> Result<JsValue, JsValue>;

    /// Returns a String with a `BCP-47` language tag inside. If the locale couldn’t be obtained, null is returned instead.
    ///
    /// Since: 1.4.0
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/os#locale>
    #[wasm_bindgen(catch)]
    pub async fn locale() -> Result<JsValue, JsValue>;

    /// Returns a string identifying the operating system platform. The value is set at compile time. Possible values are `linux`, `darwin`, `ios`, `freebsd`, `dragonfly`, `netbsd`, `openbsd`, `solaris`, `android`, `win32`
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/os#platform-1>
    #[wasm_bindgen(catch)]
    pub async fn platform() -> Result<JsValue, JsValue>;

    /// Returns the operating system's default directory for temporary files as a string.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/os#tempdir>
    #[wasm_bindgen(catch)]
    pub async fn tempdir() -> Result<JsString, JsValue>;

    /// Returns `Linux` on Linux, `Darwin` on macOS, and `Windows_NT` on Windows.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/os#type>
    #[wasm_bindgen(catch, js_name = "type")]
    pub async fn type_() -> Result<JsValue, JsValue>;

    /// Returns a string identifying the kernel version.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/os#version>
    #[wasm_bindgen(catch)]
    pub async fn version() -> Result<JsString, JsValue>;
}
