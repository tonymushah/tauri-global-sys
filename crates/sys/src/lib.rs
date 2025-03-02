/// The equivalent of the [`window.__TAURI__.app`](https://v1.tauri.app/v1/api/js/app) module
pub mod app;
pub mod error;
/// The equivalent of the [`window.__TAURI__.tauri`](https://v1.tauri.app/v1/api/js/tauri) module
pub mod tauri;

pub use error::Error;
