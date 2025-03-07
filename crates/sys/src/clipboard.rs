//! Read and write to the system clipboard.
//!
//! This module only works if [`build.withGlobalTauri`](https://tauri.app/v1/api/config/#buildconfig.withglobaltauri) in `tauri.conf.json` is set to true.
//!
//! The APIs must be added to [`tauri.allowlist.clipboard`](https://tauri.app/v1/api/config/#allowlistconfig.clipboard) in tauri.conf.json:
//!
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "clipboard": {
//!        "all": true, // enable all Clipboard APIs
//!        "writeText": true,
//!        "readText": true
//!      }
//!    }
//!  }
//! }
//! ```

pub mod raw;

/// Writes plain text to the clipboard.
///
/// Since v1.0.0
pub async fn write_text(text: &str) -> crate::Result<()> {
    raw::writeText(text).await?;
    Ok(())
}

/// Gets the clipboard content as plain text.
///
/// Since v1.0.0
pub async fn read_text() -> crate::Result<Option<String>> {
    Ok(serde_wasm_bindgen::from_value(raw::readText().await?)?)
}
