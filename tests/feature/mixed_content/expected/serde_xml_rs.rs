use serde::{Deserialize, Serialize};
use xsd_parser::xml::Text;
pub type MixedChoiceList = MixedChoiceListType;
#[derive(Debug, Serialize, Deserialize)]
pub struct MixedChoiceListType {
    #[serde(default, rename = "#content")]
    pub content: Vec<MixedChoiceListTypeContent>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum MixedChoiceListTypeContent {
    #[serde(rename = "tns:Fuu")]
    Fuu(i32),
    #[serde(rename = "tns:Bar")]
    Bar(String),
    #[serde(rename = "#text")]
    Text(Text),
}
