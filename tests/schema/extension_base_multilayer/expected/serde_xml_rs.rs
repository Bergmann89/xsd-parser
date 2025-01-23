use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "Messages")]
    pub messages: FooTypeMessagesType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooTypeMessagesType {
    #[serde(rename = "aa")]
    pub aa: i32,
    #[serde(rename = "bb")]
    pub bb: String,
    #[serde(rename = "a")]
    pub a: String,
}
