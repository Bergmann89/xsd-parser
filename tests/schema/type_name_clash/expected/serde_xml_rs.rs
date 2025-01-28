use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "Bar")]
    pub bar: FooTypeBarType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooTypeBarType {
    #[serde(default, rename = "a")]
    pub a: Option<String>,
    #[serde(default, rename = "b")]
    pub b: Option<String>,
}
