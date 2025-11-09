use serde::{Deserialize, Serialize};
use xsd_parser::xml::Text;
pub type MixedChoiceList = MixedChoiceListType;
#[derive(Debug, Deserialize, Serialize)]
pub struct MixedChoiceListType {
    #[serde(default, rename = "$value")]
    pub content: Vec<MixedChoiceListTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MixedChoiceListTypeContent {
    #[serde(rename = "Fuu")]
    Fuu(i32),
    #[serde(rename = "Bar")]
    Bar(String),
    #[serde(rename = "$text")]
    Text(Text),
}
