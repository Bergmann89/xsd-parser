#[derive(Debug, Clone)]
pub struct MyAllType {
    pub content: [MyAllTypeContent; 2usize],
}
#[derive(Debug, Clone)]
pub enum MyAllTypeContent {
    A(IntType),
    B(StringType),
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
    pub content: [MySequenceTypeContent; 2usize],
}
#[derive(Debug, Clone)]
pub enum MySequenceTypeContent {
    A(IntType),
    B(StringType),
}
