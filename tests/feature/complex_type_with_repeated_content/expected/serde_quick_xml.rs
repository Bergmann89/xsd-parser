use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(default, rename = "$value")]
    pub content: Vec<FooTypeContent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooTypeContent {
    #[serde(rename = "a")]
    pub a: i32,
    #[serde(rename = "b")]
    pub b: String,
    #[serde(default, rename = "c")]
    pub c: Option<String>,
}
