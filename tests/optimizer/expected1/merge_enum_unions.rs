pub enum MyUnionType {
    String(String),
    Var1,
    Var2,
    I32(i32),
    MyBaseUnion1(MyBaseUnion1Type),
}
#[derive(Default)]
pub struct MyBaseUnion1Type(pub Vec<UnsignedIntType>);
pub type UnsignedIntType = u32;
