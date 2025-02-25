use serde::{Deserialize, Serialize};
pub type FooOption = FooOptionType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooOptionType {
    #[serde(rename = "$value")]
    pub content: Option<FooOptionTypeContent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FooOptionTypeContent {
    #[serde(rename = "Bar")]
    Bar(String),
    #[serde(rename = "Baz")]
    Baz(i32),
}
pub type FooList = FooListType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooListType {
    #[serde(rename = "$value")]
    pub content: Vec<FooListTypeContent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FooListTypeContent {
    #[serde(rename = "Bar")]
    Bar(String),
    #[serde(rename = "Baz")]
    Baz(i32),
}
