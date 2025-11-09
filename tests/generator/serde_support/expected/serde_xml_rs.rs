use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct MyAllType {
    #[serde(default, rename = "@x")]
    pub x: Option<bool>,
    #[serde(rename = "a")]
    pub a: i32,
    #[serde(rename = "b")]
    pub b: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MyChoiceType {
    #[serde(default, rename = "@x")]
    pub x: Option<bool>,
    #[serde(rename = "#content")]
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MyChoiceTypeContent {
    #[serde(rename = "a")]
    A(i32),
    #[serde(rename = "b")]
    B(String),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MySequenceType {
    #[serde(default, rename = "@x")]
    pub x: Option<bool>,
    #[serde(rename = "a")]
    pub a: i32,
    #[serde(rename = "b")]
    pub b: String,
}
