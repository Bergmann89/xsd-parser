pub enum MyUnionType {
    String(String),
    MyEnum(MyEnumType),
    I32(i32),
    Unnamed2(UnsignedIntOpt),
}
pub enum MyEnumType {
    Var1,
    Var2,
}
#[derive(Default)]
pub struct UnsignedIntOpt(pub Vec<UnsignedIntType>);
pub type UnsignedIntType = u32;
