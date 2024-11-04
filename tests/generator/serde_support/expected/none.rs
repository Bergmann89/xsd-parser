#[derive(Debug, Clone)]
pub struct MyAllType {
    pub x: Option<BooleanType>,
    pub a: IntType,
    pub b: StringType,
}
pub type IntType = i32;
pub type StringType = String;
pub type BooleanType = bool;
#[derive(Debug, Clone)]
pub struct MyChoiceType {
    pub x: Option<BooleanType>,
    pub content: MyChoiceTypeContent,
}
#[derive(Debug, Clone)]
pub enum MyChoiceTypeContent {
    A(IntType),
    B(StringType),
}
#[derive(Debug, Clone)]
pub struct MySequenceType {
    pub x: Option<BooleanType>,
    pub a: IntType,
    pub b: StringType,
}
