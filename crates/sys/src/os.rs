//! Provides operating system-related utility methods and properties.
//!
//! This module only work with `window.__TAURI__.os` when [`build.withGlobalTauri`](https://v1.tauri.app/v1/api/config/#buildconfig.withglobaltauri) in `tauri.conf.json` is set to `true`.
//!
//! The APIs must be added to [`tauri.allowlist.os`](https://v1.tauri.app/v1/api/config/#allowlistconfig.os) in `tauri.conf.json`:
//! ```json
//! {
//!  "tauri": {
//!    "allowlist": {
//!      "os": {
//!        "all": true, // enable all Os APIs
//!      }
//!    }
//!  }
//! }
//! ```
//!
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.
//!

use enum_all_variants::AllVariants;
use serde::{Deserialize, Serialize};
pub mod raw;

/// Possible values of he operating system CPU architecture for which the tauri app was compiled
///
/// Ref: <http://v1.tauri.app/v1/api/js/os#arch>
#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, AllVariants, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    X86,
    X86_64,
    Arm,
    Aarch64,
    Mips,
    Mips64,
    Powerpc,
    Powerpc64,
    Riscv64,
    S390x,
    Sparc64,
}

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, AllVariants, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum OsType {
    Linux,
    Darwin,
    #[serde(rename = "Windows_NT")]
    WindowsNt,
}

/// Possible values of the string identifying the operating system platform.
/// The value is set at compile time.
#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, AllVariants, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Linux,
    Darwin,
    IOS,
    FreeBSD,
    Dragonfly,
    NetBSD,
    OpenBSD,
    Solaris,
    Android,
    Win32,
}

/// The operating system-specific end-of-line marker.
///
/// - `\n` on POSIX
/// - `\r\n` on Windows
///
pub fn eol() -> String {
    raw::EOL.with(|e| e.clone())
}

/// Returns the operating system CPU architecture for which the tauri app was compiled.
///
/// Ref: <http://v1.tauri.app/v1/api/js/os#arch-1>
pub async fn arch() -> crate::Result<Arch> {
    Ok(serde_wasm_bindgen::from_value(raw::arch().await?)?)
}

/// Returns a String with a `BCP-47` language tag inside. If the locale couldn’t be obtained, null is returned instead.
///
/// Since: 1.4.0
///
/// Ref: <http://v1.tauri.app/v1/api/js/os#locale>
pub async fn locale() -> crate::Result<Option<String>> {
    Ok(serde_wasm_bindgen::from_value(raw::locale().await?)?)
}

/// Returns a string identifying the operating system platform.
/// The value is set at compile time.
///
/// Ref: <http://v1.tauri.app/v1/api/js/os#platform-1>
pub async fn platform() -> crate::Result<Platform> {
    Ok(serde_wasm_bindgen::from_value(raw::platform().await?)?)
}

/// Returns the operating system's default directory for temporary files as a string.
///
/// Ref: <http://v1.tauri.app/v1/api/js/os#tempdir>
pub async fn tempdir() -> crate::Result<String> {
    raw::tempdir()
        .await?
        .as_string()
        .ok_or(crate::Error::JsStringToString)
}

/// Returns [`OsType::Linux`] on Linux, [`OsType::Darwin`] on macOS, and [`OsType::WindowsNt`] on Windows.
///
/// Ref: <http://v1.tauri.app/v1/api/js/os#type>
pub async fn type_() -> crate::Result<OsType> {
    Ok(serde_wasm_bindgen::from_value(raw::type_().await?)?)
}

/// Returns a string identifying the kernel version.
///
/// Ref: <http://v1.tauri.app/v1/api/js/os#version>
pub async fn version() -> crate::Result<String> {
    raw::version()
        .await?
        .as_string()
        .ok_or(crate::Error::JsStringToString)
}

#[cfg(test)]
mod test {
    use serde::de::{value::StringDeserializer, DeserializeOwned, IntoDeserializer};

    use crate::os::{Arch, OsType, Platform};

    fn string_deser<T: DeserializeOwned, S: AsRef<str>>(string: S) -> T {
        T::deserialize::<StringDeserializer<serde_json::Error>>(
            String::from(string.as_ref()).into_deserializer(),
        )
        .unwrap()
    }

    #[test]
    fn arch() {
        assert_eq!(Arch::X86, string_deser("x86"));
        assert_eq!(Arch::X86_64, string_deser("x86_64"));
        assert_eq!(Arch::Arm, string_deser("arm"));
        assert_eq!(Arch::Aarch64, string_deser("aarch64"));
        assert_eq!(Arch::Mips, string_deser("mips"));
        assert_eq!(Arch::Mips64, string_deser("mips64"));
        assert_eq!(Arch::Powerpc, string_deser("powerpc"));
        assert_eq!(Arch::Powerpc64, string_deser("powerpc64"));
        assert_eq!(Arch::Riscv64, string_deser("riscv64"));
        assert_eq!(Arch::S390x, string_deser("s390x"));
        assert_eq!(Arch::Sparc64, string_deser("sparc64"));
    }

    #[test]
    fn os_type() {
        assert_eq!(OsType::Linux, string_deser("Linux"));
        assert_eq!(OsType::Darwin, string_deser("Darwin"));
        assert_eq!(OsType::WindowsNt, string_deser("Windows_NT"));
    }

    #[test]
    fn platform() {
        assert_eq!(Platform::Linux, string_deser("linux"));
        assert_eq!(Platform::Darwin, string_deser("darwin"));
        assert_eq!(Platform::IOS, string_deser("ios"));
        assert_eq!(Platform::FreeBSD, string_deser("freebsd"));
        assert_eq!(Platform::Dragonfly, string_deser("dragonfly"));
        assert_eq!(Platform::NetBSD, string_deser("netbsd"));
        assert_eq!(Platform::OpenBSD, string_deser("openbsd"));
        assert_eq!(Platform::Solaris, string_deser("solaris"));
        assert_eq!(Platform::Android, string_deser("android"));
        assert_eq!(Platform::Win32, string_deser("win32"));
    }
}
