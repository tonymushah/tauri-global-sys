//! Perform operations on the current process.
//!
//! This module only works with `window.__TAURI__.process` when [`build.withGlobalTauri`](https://tauri.app/v1/api/config/#buildconfig.withglobaltauri) in `tauri.conf.json` is set to `true`.
//!
pub mod raw;

/// Exits immediately with the given `exitCode`.
///
/// Ref: <http://v1.tauri.app/v1/api/js/process#exit>
pub async fn exit(code: i32) -> crate::Result<()> {
    raw::exit(code).await?;
    Ok(())
}

/// Exits the current instance of the app then relaunches it.
///
/// Ref: <http://v1.tauri.app/v1/api/js/process#relaunch>
pub async fn relaunch() -> crate::Result<()> {
    raw::relaunch().await?;
    Ok(())
}
