#[derive(Debug, Clone)]
pub struct MyAllType {
    pub a: IntType,
    pub b: StringType,
}
pub type IntType = i32;
pub type StringType = String;
#[derive(Debug, Clone)]
pub struct MyChoiceType {
    pub a: Option<IntType>,
    pub b: Option<StringType>,
}
#[derive(Debug, Clone)]
pub struct MySequenceType {
    pub a: IntType,
    pub b: StringType,
}
