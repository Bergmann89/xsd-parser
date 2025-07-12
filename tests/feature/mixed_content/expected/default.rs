use xsd_parser::xml::{Mixed, Text};
pub type MixedAll = MixedAllType;
#[derive(Debug)]
pub struct MixedAllType {
    pub text_before: Option<Text>,
    pub fuu: Mixed<i32>,
    pub bar: Mixed<String>,
}
pub type MixedChoice = MixedChoiceType;
#[derive(Debug)]
pub struct MixedChoiceType {
    pub text_before: Option<Text>,
    pub content: MixedChoiceTypeContent,
}
#[derive(Debug)]
pub enum MixedChoiceTypeContent {
    Fuu(Mixed<i32>),
    Bar(Mixed<String>),
}
pub type MixedChoiceList = MixedChoiceListType;
#[derive(Debug)]
pub struct MixedChoiceListType {
    pub text_before: Option<Text>,
    pub content: Vec<MixedChoiceListTypeContent>,
}
#[derive(Debug)]
pub enum MixedChoiceListTypeContent {
    Fuu(Mixed<i32>),
    Bar(Mixed<String>),
}
pub type MixedSequence = MixedSequenceType;
#[derive(Debug)]
pub struct MixedSequenceType {
    pub text_before: Option<Text>,
    pub fuu: Mixed<i32>,
    pub bar: Mixed<String>,
}
