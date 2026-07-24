pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub bar: Option<FooBarType>,
}
#[derive(Debug)]
pub struct FooBarType {
    pub baz: String,
    pub content: bool,
}
