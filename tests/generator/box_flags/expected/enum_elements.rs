#[derive(Debug, Clone)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Clone)]
pub enum MyChoiceTypeContent {
    A(Box<IntType>),
    B(Box<StringType>),
    C(Box<MySequenceType>),
}
pub type IntType = i32;
pub type StringType = String;
#[derive(Debug, Clone)]
pub struct MySequenceType {
    pub a: IntType,
    pub b: StringType,
    pub c: Box<MyChoiceType>,
}
