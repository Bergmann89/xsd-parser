use serde::{Deserialize, Serialize};
pub type Packet = PacketType;
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PacketType {
    #[serde(default, rename = "tns:Sample")]
    pub sample: Vec<String>,
    #[serde(default, rename = "tns:Small")]
    pub small: Vec<u32>,
    #[serde(default, rename = "tns:Threshold")]
    pub threshold: Vec<u32>,
    #[serde(rename = "tns:Choice")]
    pub choice: ChoiceType,
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ChoiceType {
    #[serde(rename = "tns:Number")]
    Number(Vec<u32>),
    #[serde(rename = "tns:Text")]
    Text(String),
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct NamedArrayType {
    #[serde(default, rename = "tns:Sample")]
    pub sample: Vec<String>,
    #[serde(default, rename = "tns:Threshold")]
    pub threshold: Vec<u32>,
}
