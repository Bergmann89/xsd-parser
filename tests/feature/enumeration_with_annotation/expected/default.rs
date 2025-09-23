pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub enum_: EnumType,
}
#[derive(Debug)]
pub enum EnumType {
    ///Something is explicitly turned off.
    Off,
    ///Something is explicitly turned on.
    On,
    ///Something is handled automatically.
    Auto,
}
