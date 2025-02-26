use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "@union")]
    pub union_: UnionType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnionType {
    I32(i32),
    String(String),
}
