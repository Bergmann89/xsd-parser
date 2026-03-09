use core::ops::Deref;
use regex::Regex;
use std::sync::LazyLock;
use xsd_parser_types::quick_xml::ValidateError;
pub type OperatingMode = OperatingModeType;
#[derive(Debug)]
pub struct OperatingModeType(pub String);
impl OperatingModeType {
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
                ("hybrid/auto", Regex::new("^(?:hybrid/auto)$").unwrap()),
                (
                    "heat pump only",
                    Regex::new("^(?:heat pump only)$").unwrap(),
                ),
                (
                    "electric heater only",
                    Regex::new("^(?:electric heater only)$").unwrap(),
                ),
            ]
        });
        if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
            return Err(ValidateError::Pattern(PATTERNS[0usize].0));
        }
        Ok(())
    }
}
impl From<OperatingModeType> for String {
    fn from(value: OperatingModeType) -> String {
        value.0
    }
}
impl TryFrom<String> for OperatingModeType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for OperatingModeType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
