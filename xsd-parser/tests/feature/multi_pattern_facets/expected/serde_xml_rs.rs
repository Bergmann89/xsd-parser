use core::ops::Deref;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use xsd_parser_types::quick_xml::ValidateError;
pub type Voltage = VoltageType;
#[derive(Debug, Deserialize, Serialize)]
pub struct VoltageType(pub String);
impl VoltageType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[(&str, Regex); 3usize]> = LazyLock::new(|| {
            [
                ("\\d+V", Regex::new("^(?:\\d+V)$").unwrap()),
                ("[0-9]+V", Regex::new("^(?:[0-9]+V)$").unwrap()),
                ("[0-9]{1,4}V", Regex::new("^(?:[0-9]{1,4}V)$").unwrap()),
            ]
        });
        if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
            return Err(ValidateError::Pattern(PATTERNS[0usize].0));
        }
        Ok(())
    }
}
impl From<VoltageType> for String {
    fn from(value: VoltageType) -> String {
        value.0
    }
}
impl TryFrom<String> for VoltageType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for VoltageType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
