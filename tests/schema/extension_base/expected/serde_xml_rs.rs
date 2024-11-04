use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "b")]
    pub b: IntType,
    #[serde(rename = "c")]
    pub c: StringType,
    #[serde(rename = "a")]
    pub a: FloatType,
}
pub type IntType = i32;
pub type StringType = String;
pub type FloatType = f32;
