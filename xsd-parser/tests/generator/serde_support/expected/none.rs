#[derive(Debug)]
pub struct MyAllType {
    pub x: Option<bool>,
    pub a: i32,
    pub b: String,
}
#[derive(Debug)]
pub struct MyChoiceType {
    pub x: Option<bool>,
    pub content: MyChoiceTypeContent,
}
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    A(i32),
    B(String),
}
#[derive(Debug)]
pub struct MySequenceType {
    pub x: Option<bool>,
    pub a: i32,
    pub b: String,
}
