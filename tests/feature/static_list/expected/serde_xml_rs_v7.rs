use serde::{Deserialize, Serialize};
pub type Array = ArrayType;
#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayType {
    #[serde(rename = "Item")]
    pub item: [i32; 5usize],
}
