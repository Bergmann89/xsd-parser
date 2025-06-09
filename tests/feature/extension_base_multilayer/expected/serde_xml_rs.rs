use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "tns:Messages")]
    pub messages: FooTypeMessagesType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FooTypeMessagesType {
    #[serde(rename = "tns:aa")]
    pub aa: i32,
    #[serde(rename = "tns:bb")]
    pub bb: String,
    #[serde(rename = "tns:a")]
    pub a: String,
}
