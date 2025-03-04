pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content: Vec<FooTypeContent>,
}
#[derive(Debug, Clone)]
pub struct FooTypeContent {
    pub a: i32,
    pub b: String,
    pub c: Option<String>,
}
