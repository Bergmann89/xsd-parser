pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub union_: UnionType,
}
#[derive(Debug, Clone)]
pub enum UnionType {
    I32(i32),
    String(String),
}
