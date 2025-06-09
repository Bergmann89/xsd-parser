use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "tns:Once")]
    pub once: i32,
    #[serde(default, rename = "tns:Optional")]
    pub optional: Option<i32>,
    #[serde(rename = "tns:OnceSpecify")]
    pub once_specify: i32,
    #[serde(default, rename = "tns:TwiceOrMore")]
    pub twice_or_more: Vec<i32>,
}
