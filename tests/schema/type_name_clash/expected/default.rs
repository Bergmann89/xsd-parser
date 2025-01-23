pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub bar: FooTypeBarType,
}
#[derive(Debug, Clone)]
pub struct FooTypeBarType {
    pub a: Option<String>,
    pub b: Option<String>,
}
