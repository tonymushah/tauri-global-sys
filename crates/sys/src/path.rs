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
