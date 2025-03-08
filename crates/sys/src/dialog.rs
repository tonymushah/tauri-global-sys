//! Native system dialogs for opening and saving files.
//! This module only works when ['build.withGlobalTauri'](https://tauri.app/v1/api/config/#buildconfig.withglobaltauri) in `tauri.conf.json` is set to true.
//! The APIs must be added to tauri.allowlist.dialog in tauri.conf.json:
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "dialog": {
//!        "all": true, // enable all dialog APIs
//!        "ask": true, // enable dialog ask API
//!        "confirm": true, // enable dialog confirm API
//!        "message": true, // enable dialog message API
//!        "open": true, // enable file open API
//!        "save": true // enable file save API
//!      }
//!    }
//!  }
//! }
//! ```
//! __It is recommended to allowlist only the APIs you use for optimal bundle size and security.__

use serde::{Deserialize, Serialize};

pub mod raw;

#[derive(
    Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(rename_all = "camelCase")]
pub enum DialogType {
    #[default]
    Info,
    Warning,
    Error,
}

fn option_string_data(data: &Option<String>) -> bool {
    match data {
        Some(str) => str.is_empty(),
        None => true,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum ConfirmDialogOptions {
    Title(String),
    /// Ref: https://v1.tauri.app/v1/api/js/dialog/#confirmdialogoptions
    #[serde(rename_all = "camelCase")]
    Options {
        /// The label of the cancel button.
        #[serde(skip_serializing_if = "option_string_data")]
        cancel_label: Option<String>,
        /// The label of the confirm button.
        #[serde(skip_serializing_if = "option_string_data")]
        ok_label: Option<String>,
        /// The title of the dialog. Defaults to the app name.
        #[serde(skip_serializing_if = "option_string_data")]
        title: Option<String>,
        /// The type of the dialog. Defaults to info.
        _type: Option<DialogType>,
    },
}

impl From<String> for ConfirmDialogOptions {
    fn from(value: String) -> Self {
        Self::Title(value)
    }
}

impl Default for ConfirmDialogOptions {
    fn default() -> Self {
        ConfirmDialogOptions::Options {
            cancel_label: None,
            ok_label: None,
            title: None,
            _type: None,
        }
    }
}

/// Extension filters for the file dialog.
///
///  Ref: https://v1.tauri.app/v1/api/js/dialog/#dialogfilter
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct DialogFilter {
    pub extensions: Vec<String>,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum MessageDialogOptions {
    Title(String),
    /// Ref: https://v1.tauri.app/v1/api/js/dialog/#messagedialogoptions
    #[serde(rename_all = "camelCase")]
    Options {
        /// The label of the confirm button.
        #[serde(skip_serializing_if = "option_string_data")]
        ok_label: Option<String>,
        /// The title of the dialog. Defaults to the app name.
        #[serde(skip_serializing_if = "option_string_data")]
        title: Option<String>,
        /// The type of the dialog. Defaults to info.
        _type: Option<DialogType>,
    },
}

impl From<String> for MessageDialogOptions {
    fn from(value: String) -> Self {
        Self::Title(value)
    }
}

impl From<ConfirmDialogOptions> for MessageDialogOptions {
    fn from(value: ConfirmDialogOptions) -> Self {
        match value {
            ConfirmDialogOptions::Title(t) => Self::Title(t),
            ConfirmDialogOptions::Options {
                cancel_label: _,
                ok_label,
                title,
                _type,
            } => Self::Options {
                ok_label,
                title,
                _type,
            },
        }
    }
}

impl From<MessageDialogOptions> for ConfirmDialogOptions {
    fn from(value: MessageDialogOptions) -> Self {
        match value {
            MessageDialogOptions::Title(t) => Self::Title(t),
            MessageDialogOptions::Options {
                ok_label,
                title,
                _type,
            } => Self::Options {
                cancel_label: None,
                ok_label,
                title,
                _type,
            },
        }
    }
}

/// Options for the open dialog.
///
/// Ref: https://v1.tauri.app/v1/api/js/dialog/#opendialogoptions
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenDialogOptions {
    /// Initial directory or file path.
    #[serde(skip_serializing_if = "option_string_data")]
    pub default_path: Option<String>,
    /// Whether the dialog is a directory selection or not.
    pub directory: Option<bool>,
    /// The filters of the dialog.
    pub filters: Option<Vec<DialogFilter>>,
    /// Whether the dialog allows multiple selection or not.
    pub multiple: Option<bool>,
    /// If [`Self::directory`] is true, indicates that it will be read recursively later. Defines whether subdirectories will be allowed on the scope or not.
    pub recursive: Option<bool>,
    /// The title of the dialog window.
    #[serde(skip_serializing_if = "option_string_data")]
    pub title: Option<String>,
}

/// Options for the save dialog.
///
/// Ref: https://v1.tauri.app/v1/api/js/dialog/#savedialogoptions
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "camelCase")]
pub struct SaveDialogOptions {
    /// Initial directory or file path. If it's a directory path, the dialog interface will change to that folder. If it's not an existing directory, the file name will be set to the dialog's file name input and the dialog will be set to the parent folder.
    #[serde(skip_serializing_if = "option_string_data")]
    pub default_path: Option<String>,
    /// The filters of the dialog.
    pub filters: Option<Vec<DialogFilter>>,
    /// The title of the dialog window.
    #[serde(skip_serializing_if = "option_string_data")]
    pub title: Option<String>,
}

/// Shows a question dialog with `Yes` and `No` buttons.
///
/// Ref: https://v1.tauri.app/v1/api/js/dialog/#ask
pub async fn ask(message: &str, options: Option<ConfirmDialogOptions>) -> crate::Result<bool> {
    let res = raw::ask(message, serde_wasm_bindgen::to_value(&options)?).await?;
    Ok(serde_wasm_bindgen::from_value(res)?)
}

/// Shows a question dialog with Ok and Cancel buttons.
///
/// Ref: https://v1.tauri.app/v1/api/js/dialog/#confirm
pub async fn confirm(message: &str, options: Option<ConfirmDialogOptions>) -> crate::Result<bool> {
    let res = raw::confirm(message, serde_wasm_bindgen::to_value(&options)?).await?;
    Ok(serde_wasm_bindgen::from_value(res)?)
}

/// Shows a message dialog with Ok button.
///
/// Ref: https://v1.tauri.app/v1/api/js/dialog/#message
pub async fn message(message: &str, options: Option<MessageDialogOptions>) -> crate::Result<()> {
    raw::message(message, serde_wasm_bindgen::to_value(&options)?).await?;
    Ok(())
}

/// The [`open`] function return type.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase", untagged)]
pub enum OpenReturnType {
    Single(String),
    Multiple(Vec<String>),
}

impl OpenReturnType {
    /// Put all possible values into a [`Vec<String>`]
    pub fn to_vec(self) -> Vec<String> {
        match self {
            OpenReturnType::Single(path) => vec![path],
            OpenReturnType::Multiple(items) => items,
        }
    }
}

/// Open a file/directory selection dialog.
///
/// The selected paths are added to the filesystem and asset protocol allowlist scopes.
/// When security is more important than the easy of use of this API, prefer writing a dedicated command instead.
///
/// Note that the allowlist scope change is not persisted,
/// so the values are cleared when the application is restarted.
/// You can save it to the filesystem using [`tauri-plugin-persisted-scope`](https://github.com/tauri-apps/plugins-workspace/tree/v1/plugins/persisted-scope).
///
pub async fn open(options: Option<OpenDialogOptions>) -> crate::Result<Option<OpenReturnType>> {
    let res = raw::open(serde_wasm_bindgen::to_value(&options)?).await?;
    Ok(serde_wasm_bindgen::from_value(res)?)
}

/// Open a file/directory save dialog.
///
/// The selected path is added to the filesystem and asset protocol allowlist scopes.
/// When security is more important than the easy of use of this API, prefer writing a dedicated command instead.
///
/// Note that the allowlist scope change is not persisted, so the values are cleared when the application is restarted.
/// You can save it to the filesystem using [`tauri-plugin-persisted-scope`](https://github.com/tauri-apps/plugins-workspace/tree/v1/plugins/persisted-scope).
///
pub async fn save(options: Option<SaveDialogOptions>) -> crate::Result<Option<String>> {
    let res = raw::save(serde_wasm_bindgen::to_value(&options)?).await?;
    Ok(serde_wasm_bindgen::from_value(res)?)
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::dialog::{ConfirmDialogOptions, DialogType};

    #[test]
    fn dialog_type_ser() {
        assert_eq!(
            Value::String(String::from("info")),
            serde_json::to_value(DialogType::Info).unwrap()
        );
    }
    #[test]
    fn confirm_dialog_set() {
        assert_eq!(
            Value::String(String::from("info")),
            serde_json::to_value(ConfirmDialogOptions::Title("info".into())).unwrap()
        );
    }
}
