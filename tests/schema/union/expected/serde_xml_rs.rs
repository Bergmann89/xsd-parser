use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "union")]
    pub union_: UnionType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnionType {
    Int(IntType),
    String(StringType),
}
pub type IntType = i32;
pub type StringType = String;
