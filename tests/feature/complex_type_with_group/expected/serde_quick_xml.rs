use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "$value")]
    pub content: Vec<FooTypeContent>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FooTypeContent {
    #[serde(rename = "Bar")]
    Bar(String),
    #[serde(rename = "Baz")]
    Baz(i32),
    #[serde(rename = "Fizz")]
    Fizz(String),
    #[serde(rename = "Buzz")]
    Buzz(i32),
}
