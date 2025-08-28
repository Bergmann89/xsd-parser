use xsd_parser::xml::Nillable;
#[derive(Debug)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    Once(i32),
    Optional(Option<i32>),
    OnceSpecify(i32),
    TwiceOrMore(Vec<i32>),
}
#[derive(Debug)]
pub struct MySequenceType {
    pub content: MySequenceTypeContent,
}
#[derive(Debug)]
pub struct MySequenceTypeContent {
    pub once: Nillable<i32>,
    pub optional: Option<Nillable<i32>>,
    pub once_specify: i32,
    pub twice_or_more: Vec<i32>,
}
