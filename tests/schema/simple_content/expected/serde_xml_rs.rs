use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(default, rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "$value")]
    pub content: EnumType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnumType {
    #[serde(rename = "OFF")]
    Off,
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "AUTO")]
    Auto,
}
