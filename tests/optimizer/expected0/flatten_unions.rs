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
    MyBaseUnion1(MyBaseUnion1Type),
}
#[derive(Default)]
pub struct MyBaseUnion1Type(pub Vec<UnsignedIntType>);
pub type UnsignedIntType = u32;
