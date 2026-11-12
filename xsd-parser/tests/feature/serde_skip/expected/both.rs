use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageType {
    #[serde(
        default,
        skip_serializing_if = "::core::option::Option::is_none",
        rename = "@note"
    )]
    pub note: Option<String>,
    #[serde(default = "MessageType::default_category", rename = "@category")]
    pub category: String,
    #[serde(
        default,
        skip_serializing_if = "::core::option::Option::is_none",
        rename = "Description"
    )]
    pub description: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty",
        rename = "Item"
    )]
    pub item: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty",
        rename = "Required"
    )]
    pub required: Vec<String>,
}
impl MessageType {
    #[must_use]
    pub fn default_category() -> String {
        String::from("example")
    }
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ContentType {
    #[serde(
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty",
        rename = "$value"
    )]
    pub content: Vec<ContentTypeContent>,
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ContentTypeContent {
    #[serde(rename = "Flag")]
    Flag(bool),
    #[serde(rename = "Text")]
    Text(String),
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum PayloadType {
    #[serde(rename = "Values")]
    Values(Vec<String>),
    #[serde(rename = "Text")]
    Text(String),
}
