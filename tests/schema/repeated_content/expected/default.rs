pub type FooOption = FooOptionType;
#[derive(Debug, Clone)]
pub struct FooOptionType {
    pub content: Option<FooOptionTypeContent>,
}
#[derive(Debug, Clone)]
pub enum FooOptionTypeContent {
    Bar(String),
    Baz(i32),
}
pub type FooList = FooListType;
#[derive(Debug, Clone)]
pub struct FooListType {
    pub content: Vec<FooListTypeContent>,
}
#[derive(Debug, Clone)]
pub enum FooListTypeContent {
    Bar(String),
    Baz(i32),
}
