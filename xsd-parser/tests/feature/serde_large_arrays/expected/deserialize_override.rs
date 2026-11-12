use serde::{Deserialize, Serialize};
pub type Packet = PacketType;
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PacketType {
    #[serde(with = "crate::feature::serde_large_arrays::overrides::codec")]
    #[serde(rename = "Sample")]
    pub sample: [String; 64usize],
    #[serde(rename = "Small")]
    pub small: [u32; 32usize],
    #[serde(
        deserialize_with = "crate::feature::serde_large_arrays::overrides::codec::deserialize"
    )]
    #[serde(
        serialize_with = "::serde_with::As::<[::serde_with::Same; 33]>::serialize",
        rename = "Threshold"
    )]
    pub threshold: [u32; 33usize],
    #[serde(rename = "Choice")]
    pub choice: ChoiceType,
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ChoiceType {
    #[serde(rename = "Number")]
    Number(#[serde(with = "::serde_with::As::<[::serde_with::Same; 64]>")] [u32; 64usize]),
    #[serde(rename = "Text")]
    Text(String),
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct NamedArrayType {
    #[serde(with = "crate::feature::serde_large_arrays::overrides::codec")]
    #[serde(rename = "Sample")]
    pub sample: [String; 64usize],
    #[serde(
        deserialize_with = "crate::feature::serde_large_arrays::overrides::codec::deserialize"
    )]
    #[serde(
        serialize_with = "::serde_with::As::<[::serde_with::Same; 33]>::serialize",
        rename = "Threshold"
    )]
    pub threshold: [u32; 33usize],
}
