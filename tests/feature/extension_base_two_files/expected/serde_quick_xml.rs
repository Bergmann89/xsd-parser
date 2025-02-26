use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "a")]
    pub a: f32,
    #[serde(rename = "b")]
    pub b: BarType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarType {
    #[serde(rename = "b")]
    pub b: i32,
    #[serde(rename = "c")]
    pub c: String,
}
