use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(rename = "#content")]
    pub content: FooTypeContent,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum FooTypeContent {
    #[serde(rename = "tns:Once")]
    Once(i32),
    #[serde(rename = "tns:Optional")]
    Optional(Option<i32>),
    #[serde(rename = "tns:OnceSpecify")]
    OnceSpecify(i32),
    #[serde(rename = "tns:TwiceOrMore")]
    TwiceOrMore(Vec<i32>),
}
