use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "Once")]
    pub once: IntType,
    #[serde(default, rename = "Optional")]
    pub optional: Option<IntType>,
    #[serde(rename = "OnceSpecify")]
    pub once_specify: IntType,
    #[serde(default, rename = "TwiceOrMore")]
    pub twice_or_more: Vec<IntType>,
}
pub type IntType = i32;
