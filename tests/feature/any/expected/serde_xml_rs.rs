use serde::{Deserialize, Serialize};
use xsd_parser::xml::{AnyAttributes, AnyElement};
pub type Foo = FooType;
#[derive(Debug, Serialize, Deserialize)]
pub struct FooType {
    pub any_attributes: AnyAttributes,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Choice")]
    pub choice: ChoiceType,
    pub any_elements: Vec<AnyElement>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChoiceType {
    pub any_attributes: AnyAttributes,
    #[serde(rename = "$value")]
    pub content: ChoiceTypeContent,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ChoiceTypeContent {
    #[serde(rename = "Name")]
    Name(String),
    AnyElement(AnyElement),
}
