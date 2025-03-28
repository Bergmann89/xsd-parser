pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub enum_: EnumType,
}
#[derive(Debug)]
pub enum EnumType {
    Off,
    On,
    Auto,
}
