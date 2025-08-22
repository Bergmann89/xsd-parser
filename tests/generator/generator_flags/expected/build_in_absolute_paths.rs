#[derive(Debug)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    Once(::core::primitive::i32),
    Optional(Option<::core::primitive::i32>),
    OnceSpecify(::core::primitive::i32),
    TwiceOrMore(Vec<::core::primitive::i32>),
}
#[derive(Debug)]
pub struct MySequenceType {
    pub content: MySequenceTypeContent,
}
#[derive(Debug)]
pub struct MySequenceTypeContent {
    pub once: ::core::primitive::i32,
    pub optional: Option<::core::primitive::i32>,
    pub once_specify: ::core::primitive::i32,
    pub twice_or_more: Vec<::core::primitive::i32>,
}
