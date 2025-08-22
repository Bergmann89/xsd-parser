use xsd_parser::xml::{Mixed, Text};
#[derive(Debug)]
pub struct MyChoiceType {
    pub text_before: Option<Text>,
    pub content: MyChoiceTypeContent,
}
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    Once(Mixed<i32>),
    Optional(Option<Mixed<i32>>),
    OnceSpecify(Mixed<i32>),
    TwiceOrMore(Vec<Mixed<i32>>),
}
#[derive(Debug)]
pub struct MySequenceType {
    pub text_before: Option<Text>,
    pub content: MySequenceTypeContent,
}
#[derive(Debug)]
pub struct MySequenceTypeContent {
    pub once: Mixed<i32>,
    pub optional: Option<Mixed<i32>>,
    pub once_specify: Mixed<i32>,
    pub twice_or_more: Vec<Mixed<i32>>,
}
