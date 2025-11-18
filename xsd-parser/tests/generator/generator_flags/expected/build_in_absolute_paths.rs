#[derive(Debug)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    Once(::core::primitive::i32),
    Optional(::core::option::Option<::core::primitive::i32>),
    OnceSpecify(::core::primitive::i32),
    TwiceOrMore(::std::vec::Vec<::core::primitive::i32>),
}
#[derive(Debug)]
pub struct MySequenceType {
    pub content: MySequenceTypeContent,
}
#[derive(Debug)]
pub struct MySequenceTypeContent {
    pub once: ::core::primitive::i32,
    pub optional: ::core::option::Option<::core::primitive::i32>,
    pub once_specify: ::core::primitive::i32,
    pub twice_or_more: ::std::vec::Vec<::core::primitive::i32>,
}
