use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FooType {
    #[serde(rename = "Min")]
    pub min: i32,
    #[serde(rename = "Max")]
    pub max: i32,
    #[serde(rename = "Value")]
    pub value: EnumType,
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum EnumType {
    #[serde(rename = "OFF")]
    Off,
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "AUTO")]
    #[default]
    Auto,
}
