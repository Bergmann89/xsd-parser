#[derive(Debug, Clone)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Clone)]
pub enum MyChoiceTypeContent {
    A(i32),
    B(String),
    C(Box<MySequenceType>),
}
#[derive(Debug, Clone)]
pub struct MySequenceType {
    pub a: Box<i32>,
    pub b: Box<String>,
    pub c: Box<MyChoiceType>,
}
