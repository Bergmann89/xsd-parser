#[derive(Debug, Clone)]
pub struct MyAllType {
    pub a: i32,
    pub b: String,
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
    pub a: i32,
    pub b: String,
}
