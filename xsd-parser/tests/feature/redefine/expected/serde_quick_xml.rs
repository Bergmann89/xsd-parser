use serde::{Deserialize, Serialize};
pub type Foo = EnumType;
#[derive(Debug, Deserialize, Serialize)]
pub enum EnumType {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
}
