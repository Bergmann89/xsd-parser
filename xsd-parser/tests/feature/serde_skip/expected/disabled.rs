use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageType {
    #[serde(default, rename = "@note")]
    pub note: Option<String>,
    #[serde(default = "MessageType::default_category", rename = "@category")]
    pub category: String,
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    #[serde(default, rename = "Item")]
    pub item: Vec<String>,
    #[serde(default, rename = "Required")]
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
    #[serde(default, rename = "$value")]
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
pub mod xml_rs_v7 {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct MessageType {
        #[serde(default, rename = "note")]
        pub note: Option<String>,
        #[serde(default = "MessageType::default_category", rename = "category")]
        pub category: String,
        #[serde(default, rename = "Description")]
        pub description: Option<String>,
        #[serde(default, rename = "Item")]
        pub item: Vec<String>,
        #[serde(default, rename = "Required")]
        pub required: Vec<String>,
    }
    impl MessageType {
        #[must_use]
        pub fn default_category() -> String {
            String::from("example")
        }
    }
}
pub mod xml_rs_v8 {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct MessageType {
        #[serde(default, rename = "@note")]
        pub note: Option<String>,
        #[serde(default = "MessageType::default_category", rename = "@category")]
        pub category: String,
        #[serde(default, rename = "Description")]
        pub description: Option<String>,
        #[serde(default, rename = "Item")]
        pub item: Vec<String>,
        #[serde(default, rename = "Required")]
        pub required: Vec<String>,
    }
    impl MessageType {
        #[must_use]
        pub fn default_category() -> String {
            String::from("example")
        }
    }
}
