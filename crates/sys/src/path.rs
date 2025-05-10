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

/// Returns the extension of the `path`.
///
/// Ref: <http://v1.tauri.app/v1/api/js/path#extname>
pub async fn extname(path: &str) -> crate::Result<String> {
    raw::extname(path)
        .await?
        .as_string()
        .ok_or(crate::Error::JsStringToString)
}

/// Returns whether the `path` is absolute or not.
///
/// Ref: <http://v1.tauri.app/v1/api/js/path#isabsolute>
pub async fn is_absolute(path: &str) -> crate::Result<bool> {
    Ok(raw::isAbsolute(path).await?.into())
}

/// Joins all given `path` segments together
/// using the platform-specific separator as a delimiter,
/// then normalizes the resulting path.
///
/// Ref: <http://v1.tauri.app/v1/api/js/path#join>
pub async fn join(paths: Vec<String>) -> crate::Result<String> {
    Ok(raw::join(serde_wasm_bindgen::to_value(&paths)?)
        .await?
        .into())
}

/// Normalizes the given path, resolving `..` and `.` segments and resolve symbolic links.
///
/// Ref: <http://v1.tauri.app/v1/api/js/path#normalize>
pub async fn normalize(path: &str) -> crate::Result<String> {
    Ok(raw::normalize(path).await?.into())
}

/// Resolve the path to a resource file.
///
/// Ref: <http://v1.tauri.app/v1/api/js/path#resolveresource>
pub async fn resolve_resource(path: &str) -> crate::Result<String> {
    Ok(raw::resolveResource(path).await?.into())
}

/// Resolves a sequence of paths or path segments into an absolute path.
///
/// Ref: <http://v1.tauri.app/v1/api/js/path#resolve>
pub async fn resolve(paths: Vec<String>) -> crate::Result<String> {
    Ok(raw::resolve(serde_wasm_bindgen::to_value(&paths)?)
        .await?
        .into())
}

macro_rules! dirs {
    ($($name:ident => $raw:ident),*) => {
        $(
            pub async fn $name() -> crate::Result<String> {
                Ok(raw::$raw().await?.into())
            }
        )*
    };
}
