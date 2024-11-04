pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content: FooTypeContent,
}
#[derive(Debug, Clone)]
pub enum FooTypeContent {
    Bar(BarType),
    Baz(BazType),
}
pub type BarType = String;
pub type BazType = i32;
