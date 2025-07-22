use serde::{Deserialize, Serialize};
pub type Array = ArrayType;
#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayType {
    #[serde(default, rename = "Item")]
    pub item: Vec<i32>,
}
