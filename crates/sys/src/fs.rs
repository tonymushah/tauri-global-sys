//! Access the file system.
//!
//! This module is only accessible when [`build.withGlobalTauri`](https://tauri.app/v1/api/config/#buildconfig.withglobaltauri) in tauri.conf.json is set to true.
//!
//! The APIs must be added to tauri.allowlist.fs in tauri.conf.json:
//!
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "fs": {
//!        "all": true, // enable all FS APIs
//!        "readFile": true,
//!        "writeFile": true,
//!        "readDir": true,
//!        "copyFile": true,
//!        "createDir": true,
//!        "removeDir": true,
//!       "removeFile": true,
//!        "renameFile": true,
//!        "exists": true
//!      }
//!    }
//!  }
//! }
//! ```
//!
//!
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.
//! Security
//!
//! This module prevents path traversal, not allowing absolute paths or parent dir components (i.e. "/usr/path/to/file" or "../path/to/file" paths are not allowed). Paths accessed with this API must be relative to one of the base directories so if you need access to arbitrary filesystem paths, you must write such logic on the core layer instead.
//!
//! The API has a scope configuration that forces you to restrict the paths that can be accessed using glob patterns.
//!
//! The scope configuration is an array of glob patterns describing folder paths that are allowed. For instance, this scope configuration only allows accessing files on the databases folder of the $APPDATA directory:
//!
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "fs": {
//!        "scope": ["$APPDATA/databases/*"]
//!      }
//!    }
//!  }
//! }
//! ```
//! Notice the use of the $APPDATA variable. The value is injected at runtime, resolving to the app data directory. The available variables are: $APPCONFIG, $APPDATA, $APPLOCALDATA, $APPCACHE, $APPLOG, $AUDIO, $CACHE, $CONFIG, $DATA, $LOCALDATA, $DESKTOP, $DOCUMENT, $DOWNLOAD, $EXE, $FONT, $HOME, $PICTURE, $PUBLIC, $RUNTIME, $TEMPLATE, $VIDEO, $RESOURCE, $APP, $LOG, $TEMP.
//!
//! Trying to execute any API with a URL not configured on the scope results in a promise rejection due to denied access.
//!
//! Note that this scope applies to all APIs on this module.
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u8)]
pub enum BaseDirectory {
    Audio = 1,
    Cache,
    Config,
    Data,
    LocalData,
    Desktop,
    Document,
    Download,
    Executable,
    Font,
    Home,
    Picture,
    Public,
    Runtime,
    Template,
    Video,
    Resource,
    App,
    Log,
    Temp,
    AppConfig,
    AppData,
    AppLocalData,
    AppCache,
    AppLog,
}

/// Struct interface to [`FileEntry`](https://v1.tauri.app/v1/api/js/fs#fileentry)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    /// Children of this entry if it's a directory; [`None`] otherwise
    pub children: Option<Vec<Self>>,
    /// Name of the directory/file can be [`None`] if the path terminates with ..
    pub name: Option<String>,
    pub path: String,
}

/// Struct interface to [`FsBinaryFileOption`](https://v1.tauri.app/v1/api/js/fs#fsbinaryfileoption)
///
/// Options object used to write a binary data to a file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FsBinaryFileOption {
    pub contents: Vec<u8>,
    pub path: String,
}

/// Struct interface to [`FsDirOptions`](https://v1.tauri.app/v1/api/js/fs#fsdiroptions)
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FsDirOptions {
    pub dir: Option<BaseDirectory>,
    pub recursive: Option<bool>,
}

/// Struct interface to [`FsOptions`](https://v1.tauri.app/v1/api/js/fs#fsoptions)
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FsOptions {
    /// Whether the content should overwrite the content of the file or append to it.
    ///
    /// Since Tauri 1.5.0
    pub append: Option<bool>,
    pub dir: Option<BaseDirectory>,
}

/// Struct interface to [`FsTextFileOption`](https://v1.tauri.app/v1/api/js/fs#fstextfileoption)
///
/// Options object used to write a UTF-8 string to a file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FsTextFileOption {
    /// The UTF-8 string to write to the file.
    pub contents: String,
    /// Path to the file to write.
    pub path: String,
}

#[cfg(test)]
mod tests {
    use serde_json::{to_value, Value};

    use crate::fs::BaseDirectory;
    #[test]
    fn base_dir_ser() {
        assert_eq!(
            to_value(BaseDirectory::Audio).unwrap(),
            Value::Number((BaseDirectory::Audio as u8).into())
        )
    }
}
