use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "a")]
    pub a: FloatType,
    #[serde(rename = "b")]
    pub b: BarType,
}
pub type FloatType = f32;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarType {
    #[serde(rename = "b")]
    pub b: IntType,
    #[serde(rename = "c")]
    pub c: StringType,
}
pub type IntType = i32;
pub type StringType = String;
