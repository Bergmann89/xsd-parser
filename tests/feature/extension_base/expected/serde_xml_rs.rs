use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "tns:b")]
    pub b: i32,
    #[serde(rename = "tns:c")]
    pub c: String,
    #[serde(rename = "tns:a")]
    pub a: f32,
}
