pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub b: IntType,
    pub c: StringType,
    pub a: FloatType,
}
pub type IntType = i32;
pub type StringType = String;
pub type FloatType = f32;
