#[derive(Debug, Clone)]
pub struct MyAllType {
    pub content: [MyAllTypeContent; 2usize],
}
#[derive(Debug, Clone)]
pub enum MyAllTypeContent {
    A(i32),
    B(String),
}
#[derive(Debug, Clone)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Clone)]
pub enum MyChoiceTypeContent {
    A(i32),
    B(String),
}
#[derive(Debug, Clone)]
pub struct MySequenceType {
    pub content: [MySequenceTypeContent; 2usize],
}
#[derive(Debug, Clone)]
pub enum MySequenceTypeContent {
    A(i32),
    B(String),
}
