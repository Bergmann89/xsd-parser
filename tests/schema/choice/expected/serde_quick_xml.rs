use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "$value")]
    pub content: FooTypeContent,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FooTypeContent {
    #[serde(rename = "Bar")]
    Bar(BarType),
    #[serde(rename = "Baz")]
    Baz(BazType),
}
pub type BarType = String;
pub type BazType = i32;
