use core::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageType {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "#text")]
    pub content: StateTypeValue,
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StateType {
    #[serde(rename = "#text")]
    pub value: StateTypeValue,
}
impl From<StateTypeValue> for StateType {
    fn from(value: StateTypeValue) -> Self {
        Self { value }
    }
}
impl From<StateType> for StateTypeValue {
    fn from(value: StateType) -> Self {
        value.value
    }
}
impl Deref for StateType {
    type Target = StateTypeValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl DerefMut for StateType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum StateTypeValue {
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "DONE")]
    Done,
}
