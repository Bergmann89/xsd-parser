use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "Min")]
    pub min: IntType,
    #[serde(rename = "Max")]
    pub max: IntType,
}
pub type IntType = i32;
