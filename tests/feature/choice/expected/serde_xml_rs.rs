use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(rename = "#content")]
    pub content: FooTypeContent,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum FooTypeContent {
    #[serde(rename = "tns:Bar")]
    Bar(String),
    #[serde(rename = "tns:Baz")]
    Baz(i32),
}
