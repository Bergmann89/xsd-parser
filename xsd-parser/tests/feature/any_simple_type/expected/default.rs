pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub value: Vec<AnySimpleType>,
}
#[derive(Debug)]
pub struct AnySimpleType {
    pub type_: Option<String>,
    pub content: String,
}
