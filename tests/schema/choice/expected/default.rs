pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content: FooTypeContent,
}
#[derive(Debug, Clone)]
pub enum FooTypeContent {
    Bar(String),
    Baz(i32),
}
