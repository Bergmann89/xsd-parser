#[doc = "This is an example type to show how choices are working"]
#[derive(Debug)]
pub struct MyChoiceType {
    pub content: MyChoiceTypeContent,
}
#[doc = "This is an example type to show how choices are working"]
#[derive(Debug)]
pub enum MyChoiceTypeContent {
    #[doc = "Exactly one integer."]
    Once(i32),
    #[doc = "An optional integer."]
    Optional(Option<i32>),
    #[doc = "Exactly one integer, but this was set explicitly."]
    OnceSpecify(i32),
    #[doc = "Two or more integers."]
    TwiceOrMore(Vec<i32>),
}
#[doc = "This is an example type to show how sequences are working"]
#[derive(Debug)]
pub struct MySequenceType {
    pub content: MySequenceTypeContent,
}
#[doc = "This is an example type to show how sequences are working"]
#[derive(Debug)]
pub struct MySequenceTypeContent {
    #[doc = "Exactly one integer."]
    pub once: i32,
    #[doc = "An optional integer."]
    pub optional: Option<i32>,
    #[doc = "Exactly one integer, but this was set explicitly."]
    pub once_specify: i32,
    #[doc = "Two or more integers."]
    pub twice_or_more: Vec<i32>,
}
