pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub once: IntType,
    pub optional: Option<IntType>,
    pub once_specify: IntType,
    pub twice_or_more: Vec<IntType>,
}
pub type IntType = i32;
