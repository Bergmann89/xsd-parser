use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(default, rename = "@value")]
    pub value: Option<String>,
    #[serde(default, rename = "@anotherValue")]
    pub another_value: Option<String>,
    #[serde(rename = "$text")]
    pub content: EnumType,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EnumType {
    #[serde(rename = "OFF")]
    Off,
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "AUTO")]
    Auto,
}
