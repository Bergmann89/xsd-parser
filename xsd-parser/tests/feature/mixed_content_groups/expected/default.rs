use xsd_parser_types::xml::{Mixed, Text};
pub type NormalElement = NormalType;
#[derive(Debug)]
pub struct NormalType {
    pub group: NormalGroupType,
    pub baz: String,
}
#[derive(Debug)]
pub struct NormalGroupType {
    pub fuu: i32,
    pub bar: String,
}
pub type MixedElement = MixedType;
#[derive(Debug)]
pub struct MixedType {
    pub text_before: Option<Text>,
    pub group: MixedGroupType,
    pub baz: Mixed<String>,
}
#[derive(Debug)]
pub struct MixedGroupType {
    pub fuu: Mixed<i32>,
    pub bar: Mixed<String>,
}
