pub mod xs {
    use super::*;
    pub type IntType = i32;
}
pub mod tns {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct MyChoiceType {
        pub content: Vec<MyChoiceTypeContent>,
    }
    #[derive(Debug, Clone)]
    pub enum MyChoiceTypeContent {
        Once(xs::IntType),
        Optional(xs::IntType),
        OnceSpecify(xs::IntType),
        TwiceOrMore(xs::IntType),
    }
}
