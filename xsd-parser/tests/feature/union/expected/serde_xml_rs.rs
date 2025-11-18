use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(rename = "@union")]
    pub union_: UnionType,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum UnionType {
    I32(i32),
    String(String),
}
