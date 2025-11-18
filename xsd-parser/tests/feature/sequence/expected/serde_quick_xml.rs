use serde::{Deserialize, Serialize};
pub type Foo = FooType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FooType {
    #[serde(rename = "Min")]
    pub min: i32,
    #[serde(rename = "Max")]
    pub max: i32,
}
