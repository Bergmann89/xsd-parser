pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub union_: UnionType,
}
#[derive(Debug)]
pub enum UnionType {
    I32(i32),
    String(String),
}
