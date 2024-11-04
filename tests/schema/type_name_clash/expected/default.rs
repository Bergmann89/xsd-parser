pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub bar: FooTypeBar,
}
#[derive(Debug, Clone)]
pub struct FooTypeBar {
    pub a: Option<StringType>,
    pub b: Option<StringType>,
}
pub type StringType = String;
