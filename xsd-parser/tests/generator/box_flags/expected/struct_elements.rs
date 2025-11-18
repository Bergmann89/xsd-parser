#[derive(Debug)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    A(i32),
    B(String),
    C(MySequenceType),
}
#[derive(Debug)]
pub struct MySequenceType {
    pub a: Box<i32>,
    pub b: Box<String>,
    pub c: Box<MyChoiceType>,
}
