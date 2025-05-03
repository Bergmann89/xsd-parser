pub enum MyUnionType {
    String(String),
    MyEnum(MyEnumType),
    I32(i32),
    MyBaseUnion1(UnsignedIntList),
}
pub enum MyEnumType {
    Var1,
    Var2,
}
#[derive(Default)]
pub struct UnsignedIntList(pub Vec<UnsignedIntType>);
pub type UnsignedIntType = u32;
