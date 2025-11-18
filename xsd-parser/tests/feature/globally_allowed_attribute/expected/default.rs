pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub content: FooTypeContent,
}
#[derive(Debug)]
pub enum FooTypeContent {
    Once(i32),
    Optional(Option<i32>),
    OnceSpecify(i32),
    TwiceOrMore(Vec<i32>),
}
