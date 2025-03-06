use semver::Version;

/// `wasm-bindgen` raw bindings
pub mod raw;

/// Gets the application name.
///
/// Usable since Tauri 1.0.0
pub async fn get_name() -> crate::Result<String> {
    Ok(raw::get_name().await?.into())
}

/// Get the Tauri Version.
///
/// Usable since Tauri 1.0.0
pub async fn get_tauri_version() -> crate::Result<Version> {
    Ok(serde_wasm_bindgen::from_value(
        raw::get_tauri_version().await?.into(),
    )?)
}

/// Gets the application version.
///
/// Usable since Tauri 1.0.0
pub async fn get_version() -> crate::Result<Version> {
    Ok(serde_wasm_bindgen::from_value(
        raw::get_version().await?.into(),
    )?)
}

/// Show the application on macOS.
///
/// Usable Tauri 1.2.0
///
/// Require `tauri.allowlist.app.show` set to `true`
pub async fn show() -> Result<(), crate::Error> {
    raw::show().await?;
    Ok(())
}

/// Hides the application on macOS.
///
/// Usable Tauri 1.2.0
///
/// Require `tauri.allowlist.app.hide` set to `true`
pub async fn hide() -> Result<(), crate::Error> {
    raw::hide().await?;
    Ok(())
}
