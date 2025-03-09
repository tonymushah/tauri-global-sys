use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window","__TAURI__","fs"])]
extern "C" {
    /// Ref: https://v1.tauri.app/v1/api/js/fs#copyfile
    #[wasm_bindgen(catch)]
    pub async fn copyFile(source: &str, destination: &str, options: JsValue)
        -> Result<(), JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#createdir
    #[wasm_bindgen(catch)]
    pub async fn createDir(dir: &str, options: JsValue) -> Result<(), JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#exists
    #[wasm_bindgen(catch)]
    pub async fn exists(path: &str, options: JsValue) -> Result<JsValue, JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#readbinaryfile
    #[wasm_bindgen(catch)]
    pub async fn readBinaryFile(
        filePath: &str,
        options: JsValue,
    ) -> Result<js_sys::Uint8Array, JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#readdir
    #[wasm_bindgen(catch)]
    pub async fn readDir(dir: &str, options: JsValue) -> Result<JsValue, JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#readtextfile
    #[wasm_bindgen(catch)]
    pub async fn readTextFile(file_path: &str, options: JsValue) -> Result<JsValue, JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#removedir
    #[wasm_bindgen(catch)]
    pub async fn removeDir(dir: &str, option: JsValue) -> Result<(), JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#removefile
    #[wasm_bindgen(catch)]
    pub async fn removeFile(file: &str, option: JsValue) -> Result<(), JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#renamefile
    #[wasm_bindgen(catch)]
    pub async fn renameFile(
        old_path: &str,
        new_path: &str,
        options: JsValue,
    ) -> Result<(), JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#writebinaryfile
    #[wasm_bindgen(catch)]
    pub async fn writeBinaryFile(
        path: &str,
        contents: JsValue,
        options: JsValue,
    ) -> Result<(), JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#writebinaryfile
    #[wasm_bindgen(catch, js_name = "writeBinaryFile")]
    pub async fn writeBinaryFile2(file: JsValue, options: JsValue) -> Result<(), JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#writetextfile
    #[wasm_bindgen(catch)]
    pub async fn writeTextFile(path: &str, contents: &str, options: JsValue)
        -> Result<(), JsValue>;

    /// Ref: https://v1.tauri.app/v1/api/js/fs#writetextfile
    #[wasm_bindgen(catch, js_name = "writeTextFile")]
    pub async fn writeTextFile2(file: JsValue, options: JsValue) -> Result<(), JsValue>;
}
