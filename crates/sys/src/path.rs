//! The path module provides utilities for working with file and directory paths.
//!
//! This module only work with `window.__TAURI__.path` when [`build.withGlobalTauri`](https://v1.tauri.app/v1/api/config/#buildconfig.withglobaltauri) in `tauri.conf.json` is set to `true`.
//!
//! The APIs must be added to [`tauri.allowlist.path`](https://v1.tauri.app/v1/api/config/#allowlistconfig.path) in tauri.conf.json:
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "path": {
//!        "all": true, // enable all Path APIs
//!      }
//!    }
//!  }
//! }
//! ```
//!
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.
pub mod raw;

/// Provides the platform-specific path segment delimiter:
/// - `;` on Windows
/// - `:` on POSIX
///
/// Ref: <http://v1.tauri.app/v1/api/js/path#delimiter>
pub fn delimiter() -> String {
    raw::DELIMITER.with(Clone::clone)
}

/// Provides the platform-specific path segment separator:
/// - `\` on Windows
/// - `/` on POSIX
///
/// Ref: <http://v1.tauri.app/v1/api/js/path#sep>
pub fn sep() -> String {
    raw::SEP.with(Clone::clone)
}

/// Returns the directory name of a path.
/// Trailing directory separators are ignored.
///
/// Ref: <http://v1.tauri.app/v1/api/js/path#dirname>
pub async fn dirname(dir: &str) -> crate::Result<String> {
    raw::dirname(dir)
        .await?
        .as_string()
        .ok_or(crate::Error::JsStringToString)
}
