#[derive(Debug, Clone)]
pub enum MyChoiceType {
    Once(IntType),
    Optional(IntType),
    OnceSpecify(IntType),
    TwiceOrMore(IntType),
}
pub type IntType = i32;
