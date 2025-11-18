use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(rename = "Messages")]
    pub messages: FooTypeMessagesType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FooTypeMessagesType {
    #[serde(rename = "aa")]
    pub aa: i32,
    #[serde(rename = "bb")]
    pub bb: String,
    #[serde(rename = "a")]
    pub a: String,
}
