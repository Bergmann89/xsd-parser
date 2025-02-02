pub enum MyUnionType {
    String(String),
    MyEnum(MyEnumType),
    MyBaseUnion(MyBaseUnionType),
}
pub enum MyEnumType {
    Var1,
    Var2,
}
pub enum MyBaseUnionType {
    I32(i32),
    Unnamed2(UnsignedIntList),
}
#[derive(Default)]
pub struct UnsignedIntList(pub Vec<UnsignedIntType>);
pub type UnsignedIntType = u32;
