pub mod tns {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct MyChoiceType {
        pub content: MyChoiceTypeContent,
    }
    #[derive(Debug, Clone)]
    pub enum MyChoiceTypeContent {
        Once(i32),
        Optional(Option<i32>),
        OnceSpecify(i32),
        TwiceOrMore(Vec<i32>),
    }
    #[derive(Debug, Clone)]
    pub struct MySequenceType {
        pub content: MySequenceTypeContent,
    }
    #[derive(Debug, Clone)]
    pub struct MySequenceTypeContent {
        pub once: i32,
        pub optional: Option<i32>,
        pub once_specify: i32,
        pub twice_or_more: Vec<i32>,
    }
}
