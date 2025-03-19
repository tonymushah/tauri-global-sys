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
pub mod raw;
