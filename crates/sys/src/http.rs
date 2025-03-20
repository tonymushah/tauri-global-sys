//! Access the Tauri HTTP client written in Rust.
//!
//! This module is only accessible with `window.__TAURI__.http` when [`build.withGlobalTauri`](https://v1.tauri.app/v1/api/config/#buildconfig.withglobaltauri) in `tauri.conf.json` is set to `true`.
//!
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "http": {
//!        "all": true, // enable all http APIs
//!        "request": true // enable HTTP request API
//!      }
//!    }
//!  }
//! }
//! ```
//!
//! ## Security
//!
//! This API has a scope configuration that forces you to restrict the URLs and paths that can be accessed using glob patterns.
//!
//! For instance, this scope configuration only allows making HTTP requests to the GitHub API for the `tauri-apps` organization:
//!
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "http": {
//!        "scope": ["https://api.github.com/repos/tauri-apps/*"]
//!      }
//!    }
//!  }
//! }
//! ```
//!
//! Trying to execute any API with a URL not configured on the scope results in a promise rejection due to denied access.

pub mod raw;

use enum_all_variants::AllVariants;
use enum_repr::EnumRepr;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[EnumRepr(type = "u8", implicit = true)]
#[derive(
    Debug,
    Clone,
    Copy,
    AllVariants,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Deserialize_repr,
    Serialize_repr,
    Hash,
)]
pub enum ResponseType {
    JSON = 1,
    Text,
    Binary,
}

#[derive(
    Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AllVariants,
)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpVerb {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Connect,
    Trace,
}
