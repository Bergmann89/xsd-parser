use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use xsd_parser_types::quick_xml::ValidateError;
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(rename = "Enum")]
    pub enum_: EnumType,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum EnumType {
    #[serde(rename = "OFF")]
    Off,
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "AUTO")]
    Auto,
}
impl EnumType {
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[(&str, Regex); 1usize]> =
            LazyLock::new(|| [("[A-Z]+", Regex::new("^(?:[A-Z]+)$").unwrap())]);
        for (pattern, regex) in PATTERNS.iter() {
            if !regex.is_match(s) {
                return Err(ValidateError::Pattern(pattern));
            }
        }
        Ok(())
    }
}
