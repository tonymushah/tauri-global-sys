//! Register global shortcuts.
//!
//! This module is only accessible with `window.__TAURI__.globalShortcut` when [`build.withGlobalTauri`](https://tauri.app/v1/api/config/#buildconfig.withglobaltauri) in tauri.conf.json is set to `true`.
//!
//! The APIs must be added to [`tauri.allowlist.globalShortcut`](https://tauri.app/v1/api/config/#allowlistconfig.globalshortcut) in tauri.conf.json:
//!
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "globalShortcut": {
//!        "all": true // enable all global shortcut APIs
//!      }
//!    }
//!  }
//! }
//! ```
//!
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.

use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    Stream, StreamExt,
};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::spawn_local;
pub mod raw;

/// Ref: <https://v1.tauri.app/v1/api/js/globalShortcut#isregistered>
pub async fn is_registered(shortcut: &str) -> crate::Result<bool> {
    Ok(serde_wasm_bindgen::from_value(
        raw::isRegistered(shortcut).await?,
    )?)
}

/// Ref: <https://v1.tauri.app/v1/api/js/globalShortcut#register>
pub async fn register(shortcut: &str, handler: &Closure<dyn FnMut(String)>) -> crate::Result<()> {
    raw::register(shortcut, handler).await?;
    Ok(())
}

/// Ref: <https://v1.tauri.app/v1/api/js/globalShortcut#registerall>
pub async fn register_all(
    shortcuts: &Vec<String>,
    handler: &Closure<dyn FnMut(String)>,
) -> crate::Result<()> {
    raw::registerAll(serde_wasm_bindgen::to_value(shortcuts)?, handler).await?;
    Ok(())
}

/// Ref: <https://v1.tauri.app/http:/v1/api/js/globalShortcut#unregister>
pub async fn unregister(shortcut: &str) -> crate::Result<()> {
    raw::unregister(shortcut).await?;
    Ok(())
}

/// Ref: <https://v1.tauri.app/v1/api/js/globalShortcut#unregisterall>
pub async fn unregister_all() -> crate::Result<()> {
    raw::unregisterAll().await?;
    Ok(())
}

/// A shortcut handler stream and unregister all when it is dropped
pub struct ShorcutHandler {
    _handler: Closure<dyn FnMut(String)>,
    rx: UnboundedReceiver<String>,
    shortcuts: Vec<String>,
}

impl ShorcutHandler {
    /// Create a shortcuts handler stream
    pub async fn new(shortcuts: Vec<String>) -> crate::Result<Self> {
        let (tx, rx) = unbounded::<String>();
        let handler = Closure::new(move |key| {
            let _ = tx.unbounded_send(key);
        });
        register_all(&shortcuts, &handler).await?;
        Ok(Self {
            _handler: handler,
            rx,
            shortcuts,
        })
    }
}

impl Drop for ShorcutHandler {
    fn drop(&mut self) {
        let shortcuts = self.shortcuts.clone();
        spawn_local(async move {
            for key in shortcuts {
                if let Err(_err) = unregister(&key).await {
                    #[cfg(feature = "log")]
                    log::error!("{_err}");
                }
            }
        });
    }
}

impl Stream for ShorcutHandler {
    type Item = String;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}
