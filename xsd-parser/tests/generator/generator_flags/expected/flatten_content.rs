#[derive(Debug)]
pub enum MyChoiceType {
    Once(i32),
    Optional(Option<i32>),
    OnceSpecify(i32),
    TwiceOrMore(Vec<i32>),
}
#[derive(Debug)]
pub struct MySequenceType {
    pub once: i32,
    pub optional: Option<i32>,
    pub once_specify: i32,
    pub twice_or_more: Vec<i32>,
}
