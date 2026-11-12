use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AttributesType {
    #[serde(default = "AttributesType::default_values", rename = "@values")]
    pub values: ValuesType,
}
impl AttributesType {
    #[must_use]
    pub fn default_values() -> ValuesType {
        vec![String::from("fallback"), String::from("other")]
    }
}
pub type ValuesType = Vec<StringType>;
pub type StringType = String;
