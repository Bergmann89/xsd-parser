#[derive(Debug, Clone)]
pub struct MyChoiceType {
    pub content: Vec<MyChoiceTypeContent>,
}
#[derive(Debug, Clone)]
pub enum MyChoiceTypeContent {
    Once(IntType),
    Optional(IntType),
    OnceSpecify(IntType),
    TwiceOrMore(IntType),
}
pub type IntType = i32;
