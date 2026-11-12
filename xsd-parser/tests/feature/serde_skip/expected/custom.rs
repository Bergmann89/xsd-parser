use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageType {
    #[serde(default = "crate::feature::serde_skip::default_note")]
    #[serde(rename = "@note")]
    pub note: Option<String>,
    #[serde(default = "MessageType::default_category", rename = "@category")]
    pub category: String,
    #[serde(skip_serializing)]
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    #[serde(
        skip_serializing_if = "crate::feature::serde_skip::keep_empty",
        default = "crate::feature::serde_skip::default_items"
    )]
    #[serde(rename = "Item")]
    pub item: Vec<String>,
    #[serde(default = "crate::feature::serde_skip::default_items")]
    #[serde(rename = "Required")]
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
