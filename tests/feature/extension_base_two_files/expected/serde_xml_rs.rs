use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "tns:a")]
    pub a: f32,
    #[serde(rename = "tns:b")]
    pub b: BarType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BarType {
    #[serde(rename = "other:b")]
    pub b: i32,
    #[serde(rename = "other:c")]
    pub c: String,
}
