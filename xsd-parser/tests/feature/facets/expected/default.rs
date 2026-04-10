use core::ops::Deref;
use regex::Regex;
use std::sync::LazyLock;
use xsd_parser_types::quick_xml::{fraction_digits, ValidateError};
pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub content: Vec<RootTypeContent>,
}
#[derive(Debug)]
pub enum RootTypeContent {
    NegativeDecimal(NegativeDecimalType),
    PositiveDecimal(PositiveDecimalType),
    RestrictedString(RestrictedStringType),
    HexType(HexType),
    Base64Type(Base64Type),
}
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct HexType(pub String);
impl HexType {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_value(value: &String) -> Result<(), ValidateError> {
        fn binary_value_length(value: &str) -> usize {
            value.bytes().count() / 2usize
        }
        if binary_value_length(value) < 16usize {
            return Err(ValidateError::MinLength(16usize));
        }
        if binary_value_length(value) > 16usize {
            return Err(ValidateError::MaxLength(16usize));
        }
        Ok(())
    }
}
impl From<HexType> for String {
    fn from(value: HexType) -> String {
        value.0
    }
}
impl TryFrom<String> for HexType {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for HexType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug)]
pub struct Base64Type(pub String);
impl Base64Type {
    pub fn new(inner: String) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn validate_value(value: &String) -> Result<(), ValidateError> {
        fn binary_value_length(value: &str) -> usize {
            let bytes = value.as_bytes();
            let padding = match bytes {
                [.., b'=', b'='] => 2,
                [.., b'='] => 1,
                _ => 0,
            };
            (bytes.len() / 4usize) * 3usize - padding
        }
        if binary_value_length(value) < 16usize {
            return Err(ValidateError::MinLength(16usize));
        }
        if binary_value_length(value) > 16usize {
            return Err(ValidateError::MaxLength(16usize));
        }
        Ok(())
    }
}
impl From<Base64Type> for String {
    fn from(value: Base64Type) -> String {
        value.0
    }
}
impl TryFrom<String> for Base64Type {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for Base64Type {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
