#[derive(Debug, Clone)]
pub struct MyType(pub i32);
#[derive(Debug, Clone, Default)]
pub struct MyListType(pub Vec<IntType>);
#[derive(Debug, Clone)]
pub struct IntType(pub i32);
