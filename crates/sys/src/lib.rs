/// The equivalent of the [`window.__TAURI__.app`](https://v1.tauri.app/v1/api/js/app) module.
///
pub mod app;
/// The equivalent of the [`window.__TAURI__.cli`](https://v1.tauri.app/v1/api/js/cli) module.
///
pub mod cli;
/// The equivalent of the [`window.__TAURI__.clipboard`](https://v1.tauri.app/v1/api/js/clipboard) module.
///
pub mod clipboard;
/// The equivalent of the [`window.__TAURI__.dialog`](https://v1.tauri.app/v1/api/js/dialog) module.
///
pub mod dialog;
pub mod error;
/// The equivalent of the [`window.__TAURI__.event`](https://v1.tauri.app/v1/api/js/event) module.
///
pub mod event;
/// The equivalent of the [`window.__TAURI__.fs`](https://v1.tauri.app/v1/api/js/fs) module.
///
pub mod fs;
/// the equivalent of the [`window.__TAURI__.globalShortcut`](https://v1.tauri.app/) module.
///
pub mod global_shortcut;
/// The equivalent of the [`window.__TAURI.http`](http://v1.tauri.app/v1/api/js/http) module.
///
pub mod http;
/// The equivalent of the [`window.__TAURI__.notification`](https://v1.tauri.app/v1/api/js/notification) module.
///
pub mod notification;
/// The equivalent of the [`window.__TAURI__.tauri`](https://v1.tauri.app/v1/api/js/tauri) module.
///
pub mod tauri;

pub use error::Error;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
