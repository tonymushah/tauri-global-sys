//! Send toast notifications (brief auto-expiring OS window element) to your user.
//!
//! Can also be used with the Notification Web API.
//!
//! This module only work with `window.__TAURI__.notification` when [`build.withGlobalTauri`](https://v1.tauri.app/v1/api/config/#buildconfig.withglobaltauri) in `tauri.conf.json` is set to `true`.
//!
//! The APIs must be added to [`tauri.allowlist.notification`](https://v1.tauri.app/v1/api/config/#allowlistconfig.notification) in `tauri.conf.json`:
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "notification": {
//!        "all": true // enable all notification APIs
//!      }
//!    }
//!  }
//! }
//! ```
//!
//! Tony Mushah Note: the [`send_notification`] function doesn't work really well on (Fedora 41) Linux. *Probably a Webkit issue...*

use enum_all_variants::AllVariants;
use serde::{Deserialize, Serialize};

pub mod raw;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Options {
    /// Optional notification body.
    ///
    /// Ref: <https://v1.tauri.app/v1/api/js/notification/#body>
    pub body: Option<String>,
    /// Optional notification icon.
    ///
    /// ### Platform-specific
    /// - **Windows**: The app must be installed for this to have any effect.
    ///
    /// Ref: <https://v1.tauri.app/v1/api/js/notification/#icon>
    pub icon: Option<String>,
    /// Optional notification sound.
    ///
    /// ### Platform-specific
    ///
    /// Each OS has a different sound name so you will need to conditionally specify an appropriate sound based on the OS in use, 'default' represents the default system sound.
    /// For a list of sounds see:
    ///
    /// - **Linux**: can be one of the sounds listed in <https://0pointer.de/public/sound-naming-spec.html>
    /// - **Windows**: can be one of the sounds listed in <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-audio> but without the prefix, for example, if `ms-winsoundevent:Notification.Default` you would use `Default` and if `ms-winsoundevent:Notification.Looping.Alarm2`, you would use `Alarm2`. Windows 7 is not supported, if a sound is provided, it will play the default sound, otherwise it will be silent.
    /// - **macOS**: you can specify the name of the sound you'd like to play when the notification is shown. Any of the default sounds (under System Preferences > Sound) can be used, in addition to custom sound files. Be sure that the sound file is copied under the app bundle (e.g., `YourApp.app/Contents/Resources`), or one of the following locations:
    ///     - `~/Library/Sounds`
    ///     - `/Library/Sounds`
    ///     - `/Network/Library/Sounds`
    ///     - `/System/Library/Sounds`
    ///       \
    ///       See the [NSSound](https://developer.apple.com/documentation/appkit/nssound) docs for more information.\
    ///
    /// Since: Tauri 1.5.0
    ///
    /// Ref: <https://v1.tauri.app/v1/api/js/notification/#sound>
    pub sound: Option<String>,
    /// Notification title.
    ///
    /// Ref: <https://v1.tauri.app/v1/api/js/notification/#title>
    pub title: String,
}

impl<S: AsRef<str>> From<S> for Options {
    fn from(value: S) -> Self {
        Self {
            body: None,
            icon: None,
            sound: None,
            title: String::from(value.as_ref()),
        }
    }
}

/// Ref: <http://v1.tauri.app/v1/api/js/notification#permission>
#[derive(
    Debug,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    AllVariants,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    Granted,
    Denied,
    #[default]
    Default,
}

/// Checks if the permission to send notifications is granted.
///
/// ```rs
/// use tauri_global_sys::notification::is_permission_granted;
///
/// let permission_granted = is_permission_granted().await?;
/// ```
///
/// Ref: <http://v1.tauri.app/v1/api/js/notification#ispermissiongranted>
pub async fn is_permission_granted() -> crate::Result<bool> {
    Ok(raw::isPermissionGranted().await?.value_of())
}

/// Requests the permission to send notifications.
///
/// ```rs
/// use tauri_global_sys::notification::{is_permission_granted, request_permission, Permission};
///
/// let mut permission_granted = is_permission_granted().await?;
/// if !permission_granted {
///     let permission = request_permission().await?;
///     permission_granted = permission == Permission::Granted;
/// }
/// ```
///
/// Ref: <http://v1.tauri.app/v1/api/js/notification#requestpermission>
pub async fn request_permission() -> crate::Result<Permission> {
    Ok(serde_wasm_bindgen::from_value(
        raw::requestPermission().await?,
    )?)
}

/// Sends a notification to the user.
///
/// ```rs
/// use tauri_global_sys::notification::{is_permission_granted, request_permission, Permission, send_notification, Options};
///
/// let mut permission_granted = is_permission_granted().await?;
/// if !permission_granted {
///     let permission = request_permission().await?;
///     permission_granted = permission == Permission::Granted;
/// }
/// if permission_granted {
///     send_notification("Tauri is awesome!".into())?;
///     send_notification(Options {
///         title: "TAURI".into(),
///         body: Some("Tauri is awesome!".into()),
///         sound: None,
///         icon: None
///     })?;
/// }
/// ```
///
/// Ref: <http://v1.tauri.app/v1/api/js/notification#sendnotification>
pub fn send_notification(options: Options) -> crate::Result<()> {
    raw::sendNotification(serde_wasm_bindgen::to_value(&options)?)?;
    Ok(())
}
