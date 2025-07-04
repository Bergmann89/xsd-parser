use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    #[serde(rename = "@tns:union")]
    pub union_: UnionType,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UnionType {
    I32(i32),
    String(String),
}
