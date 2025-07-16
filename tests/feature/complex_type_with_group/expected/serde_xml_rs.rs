use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "#content")]
    pub content: Vec<FooTypeContent>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FooTypeContent {
    #[serde(rename = "tns:Bar")]
    Bar(String),
    #[serde(rename = "tns:Baz")]
    Baz(i32),
    #[serde(rename = "tns:Fizz")]
    Fizz(String),
    #[serde(rename = "tns:Buzz")]
    Buzz(i32),
}
