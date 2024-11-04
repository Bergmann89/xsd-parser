pub enum MyUnionType {
    String(StringType),
    MyEnum(MyEnumType),
    MyBaseUnion(MyBaseUnionType),
}
pub type StringType = String;
pub enum MyEnumType {
    Var1,
    Var2,
}
pub enum MyBaseUnionType {
    Int(IntType),
    Unnamed1(UnsignedIntList),
}
pub type IntType = i32;
#[derive(Default)]
pub struct UnsignedIntList(pub Vec<UnsignedIntType>);
pub type UnsignedIntType = u32;
