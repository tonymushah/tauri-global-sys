/// The equivalent of the [`window.__TAURI__.app`](https://v1.tauri.app/v1/api/js/app) module
pub mod app;
/// The equivalent of the [`window.__TAURI__.cli`](https://v1.tauri.app/v1/api/js/cli) module
pub mod cli;
/// The equivalent of the [`window.__TAURI__.clipboard`](https://v1.tauri.app/v1/api/js/clipboard) module
pub mod clipboard;
pub mod error;
/// The equivalent of the [`window.__TAURI__.event`](https://v1.tauri.app/v1/api/js/event) module
pub mod event;
/// The equivalent of the [`window.__TAURI__.tauri`](https://v1.tauri.app/v1/api/js/tauri) module
pub mod tauri;

pub use error::Error;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
