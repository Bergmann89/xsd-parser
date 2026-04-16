use core::ops::Deref;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use xsd_parser_types::{
    quick_xml::{fraction_digits, ValidateError},
    xml::{Base64String, HexString},
};
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
    #[serde(rename = "HexType")]
    HexType(HexType),
    #[serde(rename = "Base64Type")]
    Base64Type(Base64Type),
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
        if !PATTERNS.iter().any(|(_, regex)| regex.is_match(s)) {
            return Err(ValidateError::Pattern(PATTERNS[0usize].0));
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
#[derive(Debug, Deserialize, Serialize)]
pub struct HexType(pub HexString);
impl HexType {
    pub fn new(inner: HexString) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> HexString {
        self.0
    }
    pub fn validate_value(value: &HexString) -> Result<(), ValidateError> {
        if value.len() < 16usize {
            return Err(ValidateError::MinLength(16usize));
        }
        if value.len() > 16usize {
            return Err(ValidateError::MaxLength(16usize));
        }
        Ok(())
    }
}
impl From<HexType> for HexString {
    fn from(value: HexType) -> HexString {
        value.0
    }
}
impl TryFrom<HexString> for HexType {
    type Error = ValidateError;
    fn try_from(value: HexString) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for HexType {
    type Target = HexString;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Base64Type(pub Base64String);
impl Base64Type {
    pub fn new(inner: Base64String) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> Base64String {
        self.0
    }
    pub fn validate_value(value: &Base64String) -> Result<(), ValidateError> {
        if value.len() < 16usize {
            return Err(ValidateError::MinLength(16usize));
        }
        if value.len() > 16usize {
            return Err(ValidateError::MaxLength(16usize));
        }
        Ok(())
    }
}
impl From<Base64Type> for Base64String {
    fn from(value: Base64Type) -> Base64String {
        value.0
    }
}
impl TryFrom<Base64String> for Base64Type {
    type Error = ValidateError;
    fn try_from(value: Base64String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for Base64Type {
    type Target = Base64String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
