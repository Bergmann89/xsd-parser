use xsd_parser::xml::{Mixed, Text};
pub struct MixedChoiceListType {
    pub text_before: Option<Text>,
    pub content: Vec<MixedChoiceListTypeContent>,
}
pub enum MixedChoiceListTypeContent {
    Fuu(Mixed<i32>),
    Bar(Mixed<String>),
}
pub struct MixedSequenceType {
    pub text_before: Option<Text>,
    pub fuu: Mixed<i32>,
    pub bar: Mixed<String>,
}
