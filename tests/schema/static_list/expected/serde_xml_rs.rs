use serde::{Deserialize, Serialize};
pub type Array = ArrayType;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayType {
    #[serde(rename = "Item")]
    pub item: [IntType; 5usize],
}
pub type IntType = i32;
