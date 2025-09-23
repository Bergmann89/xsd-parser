use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub enum FooType {
    #[serde(rename = "tns:Once")]
    Once(i32),
    #[serde(rename = "tns:Optional")]
    Optional(Option<i32>),
    #[serde(rename = "tns:OnceSpecify")]
    OnceSpecify(i32),
    #[serde(rename = "tns:TwiceOrMore")]
    TwiceOrMore(Vec<i32>),
}
