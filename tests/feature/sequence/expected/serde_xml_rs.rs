use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(rename = "tns:Min")]
    pub min: i32,
    #[serde(rename = "tns:Max")]
    pub max: i32,
}
