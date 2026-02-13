use core::ops::Deref;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use xsd_parser_types::quick_xml::{fraction_digits, ValidateError};
pub type Root = RootType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RootType {
    #[serde(default, rename = "#content")]
    pub content: Vec<RootTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum RootTypeContent {
    #[serde(rename = "NegativeDecimal")]
    NegativeDecimal(NegativeDecimalType),
    #[serde(rename = "PositiveDecimal")]
    PositiveDecimal(PositiveDecimalType),
    #[serde(rename = "RestrictedString")]
    RestrictedString(RestrictedStringType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NegativeDecimalType(pub f64);
impl NegativeDecimalType {
    pub fn new(inner: f64) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> f64 {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        fraction_digits(s, 2usize)?;
        Ok(())
    }
    pub fn validate_value(value: &f64) -> Result<(), ValidateError> {
        if *value < -999999999.99f64 {
            return Err(ValidateError::LessThan("-999999999.99"));
        }
        if *value >= 0f64 {
            return Err(ValidateError::GraterEqualThan("0"));
        }
        Ok(())
    }
}
impl From<NegativeDecimalType> for f64 {
    fn from(value: NegativeDecimalType) -> f64 {
        value.0
    }
}
impl TryFrom<f64> for NegativeDecimalType {
    type Error = ValidateError;
    fn try_from(value: f64) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for NegativeDecimalType {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PositiveDecimalType(pub f64);
impl PositiveDecimalType {
    pub fn new(inner: f64) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> f64 {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        fraction_digits(s, 2usize)?;
        Ok(())
    }
    pub fn validate_value(value: &f64) -> Result<(), ValidateError> {
        if *value <= 0f64 {
            return Err(ValidateError::LessEqualThan("0"));
        }
        if *value > 999999999.99f64 {
            return Err(ValidateError::GraterThan("999999999.99"));
        }
        Ok(())
    }
}
impl From<PositiveDecimalType> for f64 {
    fn from(value: PositiveDecimalType) -> f64 {
        value.0
    }
}
impl TryFrom<f64> for PositiveDecimalType {
    type Error = ValidateError;
    fn try_from(value: f64) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for PositiveDecimalType {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RestrictedStringType(pub String);
impl RestrictedStringType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_str(s: &str) -> Result<(), ValidateError> {
        static PATTERNS: LazyLock<[(&str, Regex); 1usize]> = LazyLock::new(|| {
            [(
                "[A-Z][a-z]{4,9}",
                Regex::new("^(?:[A-Z][a-z]{4,9})$").unwrap(),
            )]
        });
        for (pattern, regex) in PATTERNS.iter() {
            if !regex.is_match(s) {
                return Err(ValidateError::Pattern(pattern));
            }
        }
        Ok(())
    }
    pub fn validate_value(value: &String) -> Result<(), ValidateError> {
        if value.len() < 5usize {
            return Err(ValidateError::MinLength(5usize));
        }
        if value.len() > 10usize {
            return Err(ValidateError::MaxLength(10usize));
        }
        Ok(())
    }
}
impl From<RestrictedStringType> for String {
    fn from(value: RestrictedStringType) -> String {
        value.0
    }
}
impl TryFrom<String> for RestrictedStringType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for RestrictedStringType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
