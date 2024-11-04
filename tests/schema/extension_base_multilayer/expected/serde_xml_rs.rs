use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "Messages")]
    pub messages: FooTypeMessages,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooTypeMessages {
    #[serde(rename = "aa")]
    pub aa: IntType,
    #[serde(rename = "bb")]
    pub bb: StringType,
    #[serde(rename = "a")]
    pub a: StringType,
}
pub type IntType = i32;
pub type StringType = String;
