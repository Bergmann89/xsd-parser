pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub enum_: EnumType,
}
#[derive(Debug, Clone)]
pub enum EnumType {
    Off,
    On,
    Auto,
}
