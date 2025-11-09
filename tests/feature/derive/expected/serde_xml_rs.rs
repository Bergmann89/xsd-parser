use core::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FooType {
    #[serde(rename = "tns:Min")]
    pub min: i32,
    #[serde(rename = "tns:Max")]
    pub max: i32,
    #[serde(rename = "tns:Value")]
    pub value: EnumType,
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum EnumTypeValue {
    #[serde(rename = "OFF")]
    Off,
    #[serde(rename = "ON")]
    On,
    #[default]
    #[serde(rename = "AUTO")]
    Auto,
}
