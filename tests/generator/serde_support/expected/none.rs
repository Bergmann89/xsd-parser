#[derive(Debug, Clone)]
pub struct MyAllType {
    pub x: Option<bool>,
    pub a: i32,
    pub b: String,
}
#[derive(Debug, Clone)]
pub struct MyChoiceType {
    pub x: Option<bool>,
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Clone)]
pub enum MyChoiceTypeContent {
    A(i32),
    B(String),
}
#[derive(Debug, Clone)]
pub struct MySequenceType {
    pub x: Option<bool>,
    pub a: i32,
    pub b: String,
}
