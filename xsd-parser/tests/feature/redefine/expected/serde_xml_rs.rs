use core::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
pub type Foo = EnumType;
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub enum EnumTypeValue {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
}
