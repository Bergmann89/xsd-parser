use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyAllType {
    #[serde(default, rename = "x")]
    pub x: Option<BooleanType>,
    #[serde(rename = "a")]
    pub a: IntType,
    #[serde(rename = "b")]
    pub b: StringType,
}
pub type IntType = i32;
pub type StringType = String;
pub type BooleanType = bool;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyChoiceType {
    #[serde(default, rename = "x")]
    pub x: Option<BooleanType>,
    #[serde(rename = "$value")]
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MyChoiceTypeContent {
    #[serde(rename = "a")]
    A(IntType),
    #[serde(rename = "b")]
    B(StringType),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MySequenceType {
    #[serde(default, rename = "x")]
    pub x: Option<BooleanType>,
    #[serde(rename = "a")]
    pub a: IntType,
    #[serde(rename = "b")]
    pub b: StringType,
}
