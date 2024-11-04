#[derive(Debug, Clone)]
pub struct MyAllType {
    pub a: IntType,
    pub b: StringType,
}
pub type IntType = i32;
pub type StringType = String;
#[derive(Debug, Clone)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Clone)]
pub enum MyChoiceTypeContent {
    A(IntType),
    B(StringType),
}
#[derive(Debug, Clone)]
pub struct MySequenceType {
    pub a: IntType,
    pub b: StringType,
}
