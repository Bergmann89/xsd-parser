pub type Foo = FooType;
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FooType {
    pub min: i32,
    pub max: i32,
    pub value: EnumType,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum EnumType {
    Off,
    On,
    #[default]
    Auto,
}
