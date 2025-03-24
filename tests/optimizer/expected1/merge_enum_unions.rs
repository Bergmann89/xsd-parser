pub enum MyUnionType {
    String(String),
    Var1,
    Var2,
    I32(i32),
    UnsignedIntOpt(UnsignedIntOpt),
}
#[derive(Default)]
pub struct UnsignedIntOpt(pub Vec<UnsignedIntType>);
pub type UnsignedIntType = u32;
