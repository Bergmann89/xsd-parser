#[derive(Debug, Clone)]
pub enum MyChoiceType {
    Once(i32),
    Optional(Option<i32>),
    OnceSpecify(i32),
    TwiceOrMore(Vec<i32>),
}
