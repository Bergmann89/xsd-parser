use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(default, rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "$text")]
    pub content: EnumType,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum EnumType {
    #[serde(rename = "OFF")]
    Off,
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "AUTO")]
    Auto,
}
