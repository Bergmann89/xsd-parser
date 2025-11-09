use serde::{Deserialize, Serialize};
pub type Array = ArrayType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayType {
    #[serde(default, rename = "Item")]
    pub item: Vec<i32>,
}
