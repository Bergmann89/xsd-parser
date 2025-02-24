use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "Once")]
    pub once: i32,
    #[serde(default, rename = "Optional")]
    pub optional: Option<i32>,
    #[serde(rename = "OnceSpecify")]
    pub once_specify: i32,
    #[serde(default, rename = "TwiceOrMore")]
    pub twice_or_more: Vec<i32>,
}
