pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub text: StringType,
}
pub type StringType = String;
