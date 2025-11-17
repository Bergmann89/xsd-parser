use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(rename = "#content")]
    pub content: Vec<FooTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum FooTypeContent {
    #[serde(rename = "tns:Element1")]
    Element1(i32),
    #[serde(rename = "tns:Element2")]
    Element2(String),
    #[serde(rename = "tns:Element3")]
    Element3(i32),
    #[serde(rename = "tns:Element4")]
    Element4(String),
}
