use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(default, rename = "#content")]
    pub content: Vec<FooTypeContent>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FooTypeContent {
    #[serde(rename = "tns:a")]
    pub a: i32,
    #[serde(default, rename = "tns:b")]
    pub b: Option<String>,
    #[serde(default, rename = "tns:c")]
    pub c: Option<String>,
}
