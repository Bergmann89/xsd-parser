pub enum MyUnionType {
    String(String),
    Var1,
    Var2,
    I32(i32),
    UnsignedIntList(UnsignedIntList),
}
#[derive(Default)]
pub struct UnsignedIntList(pub Vec<UnsignedIntType>);
pub type UnsignedIntType = u32;
