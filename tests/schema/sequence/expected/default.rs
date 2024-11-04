pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub min: IntType,
    pub max: IntType,
}
pub type IntType = i32;
