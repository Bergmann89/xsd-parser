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
    pub a: i32,
    pub b: String,
    pub c: Box<MyChoiceType>,
}
