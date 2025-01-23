#[derive(Debug, Clone)]
pub enum MyChoiceType {
    Once(i32),
    Optional(i32),
    OnceSpecify(i32),
    TwiceOrMore(i32),
}
