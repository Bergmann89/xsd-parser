pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub union_: UnionType,
}
#[derive(Debug, Clone)]
pub enum UnionType {
    Int(IntType),
    String(StringType),
}
pub type IntType = i32;
pub type StringType = String;
