use core::ops::{Deref, DerefMut};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use xsd_parser::quick_xml::ValidateError;
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "tns:Enum")]
    pub enum_: EnumType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EnumType {
    #[serde(rename = "#text")]
    pub value: EnumTypeValue,
}
impl From<EnumTypeValue> for EnumType {
    fn from(value: EnumTypeValue) -> Self {
        Self { value }
    }
}
impl From<EnumType> for EnumTypeValue {
    fn from(value: EnumType) -> Self {
        value.value
    }
}
impl Deref for EnumType {
    type Target = EnumTypeValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl DerefMut for EnumType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EnumTypeValue {
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
