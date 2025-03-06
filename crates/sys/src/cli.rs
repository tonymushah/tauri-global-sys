use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod raw;

/// Possible value(s) for [`ArgMatch`]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum ArgMatchValue {
    String(String),
    Boolean(bool),
    Strings(Vec<String>),
}

/// Rust equivalent struct for [`ArgMatch`](https://v1.tauri.app/v1/api/js/cli#argmatch)
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ArgMatch {
    pub occurences: usize,
    pub value: Option<ArgMatchValue>,
}

/// Rust equivalent struct for [`CliMatches`](https://v1.tauri.app/v1/api/js/cli#climatches)
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CliMatches {
    pub args: HashMap<String, ArgMatch>,
    pub subcommand: Option<Box<SubcommandMatch>>,
}

/// Rust equivalent struct for [`SubcommandMatch`](https://v1.tauri.app/v1/api/js/cli#subcommandmatch)
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SubcommandMatch {
    pub name: String,
    pub matches: Box<CliMatches>,
}

/// Parse the arguments provided to the current process and get the matches using the configuration defined [tauri.cli](https://tauri.app/v1/api/config/#tauriconfig.cli) in tauri.conf.json
///
/// Since: 1.0.0
pub async fn get_matches() -> crate::Result<CliMatches> {
    let maybe_data = raw::getMatches().await?;
    Ok(serde_wasm_bindgen::from_value(maybe_data)?)
}

#[cfg(test)]
mod tests {
    use serde_json::{from_value, Value};

    use crate::cli::ArgMatchValue;
    #[test]
    fn arg_value_parse() {
        assert_eq!(
            ArgMatchValue::Boolean(true),
            from_value(Value::Bool(true)).unwrap()
        );
        let test_str = String::from("Tony");
        assert_eq!(
            ArgMatchValue::String(test_str.clone()),
            from_value(Value::String(test_str.clone())).unwrap()
        );
        assert_eq!(
            ArgMatchValue::Strings(vec![test_str.clone()]),
            from_value(Value::Array(vec![test_str.clone().into()])).unwrap()
        );
    }
}
