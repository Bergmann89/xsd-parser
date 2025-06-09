use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct MyAllType {
    #[serde(default, rename = "@tns:x")]
    pub x: Option<bool>,
    #[serde(rename = "tns:a")]
    pub a: i32,
    #[serde(rename = "tns:b")]
    pub b: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MyChoiceType {
    #[serde(default, rename = "@tns:x")]
    pub x: Option<bool>,
    #[serde(rename = "#content")]
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum MyChoiceTypeContent {
    #[serde(rename = "tns:a")]
    A(i32),
    #[serde(rename = "tns:b")]
    B(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MySequenceType {
    #[serde(default, rename = "@tns:x")]
    pub x: Option<bool>,
    #[serde(rename = "tns:a")]
    pub a: i32,
    #[serde(rename = "tns:b")]
    pub b: String,
}
