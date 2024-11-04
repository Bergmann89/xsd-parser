use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "Bar")]
    pub bar: FooTypeBar,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooTypeBar {
    #[serde(default, rename = "a")]
    pub a: Option<StringType>,
    #[serde(default, rename = "b")]
    pub b: Option<StringType>,
}
pub type StringType = String;
