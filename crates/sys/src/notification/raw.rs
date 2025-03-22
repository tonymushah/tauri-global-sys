use js_sys::Boolean;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "notification"])]
extern "C" {
    /// Checks if the permission to send notifications is granted.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/notification#ispermissiongranted>
    #[wasm_bindgen(catch)]
    pub async fn isPermissionGranted() -> Result<Boolean, JsValue>;

    /// Requests the permission to send notifications.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/notification#requestpermission>
    #[wasm_bindgen(catch)]
    pub async fn requestPermission() -> Result<JsValue, JsValue>;

    /// Sends a notification to the user.
    ///
    /// Ref: <http://v1.tauri.app/v1/api/js/notification#sendnotification>
    #[wasm_bindgen(catch)]
    pub fn sendNotification(options: JsValue) -> Result<(), JsValue>;
}
