///This is an example type to show how choices are working
#[derive(Debug)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
///This is an example type to show how choices are working
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    ///Exactly one integer.
    Once(i32),
    ///An optional integer.
    Optional(Option<i32>),
    ///Exactly one integer, but this was set explicitly.
    OnceSpecify(i32),
    ///Two or more integers.
    TwiceOrMore(Vec<i32>),
}
///This is an example type to show how sequences are working
#[derive(Debug)]
pub struct MySequenceType {
    ///Exactly one integer.
    pub once: i32,
    ///An optional integer.
    pub optional: Option<i32>,
    ///Exactly one integer, but this was set explicitly.
    pub once_specify: i32,
    ///Two or more integers.
    pub twice_or_more: Vec<i32>,
}
