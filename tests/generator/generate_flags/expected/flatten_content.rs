#[derive(Debug, Clone)]
pub struct MyChoiceType {
    pub content: Vec<MyChoiceTypeContent>,
}
#[derive(Debug, Clone)]
pub enum MyChoiceTypeContent {
    Once(i32),
    Optional(i32),
    OnceSpecify(i32),
    TwiceOrMore(i32),
}
