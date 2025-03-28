use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct MyAllType {
    #[serde(default, rename = "@x")]
    pub x: Option<bool>,
    #[serde(rename = "a")]
    pub a: i32,
    #[serde(rename = "b")]
    pub b: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MyChoiceType {
    #[serde(default, rename = "@x")]
    pub x: Option<bool>,
    #[serde(rename = "$value")]
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum MyChoiceTypeContent {
    #[serde(rename = "a")]
    A(i32),
    #[serde(rename = "b")]
    B(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MySequenceType {
    #[serde(default, rename = "@x")]
    pub x: Option<bool>,
    #[serde(rename = "a")]
    pub a: i32,
    #[serde(rename = "b")]
    pub b: String,
}
