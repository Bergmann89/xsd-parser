use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use xsd_parser::quick_xml::ValidateError;
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "Enum")]
    pub enum_: EnumType,
}
#[derive(Debug, Serialize, Deserialize)]
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
        static PATTERNS: LazyLock<[Regex; 1usize]> =
            LazyLock::new(|| [Regex::new("[A-Z]+").unwrap()]);
        for pattern in PATTERNS.iter() {
            if !pattern.is_match(s) {
                return Err(ValidateError::Pattern(pattern.as_str()));
            }
        }
        Ok(())
    }
}
