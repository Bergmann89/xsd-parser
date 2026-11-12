#[derive(Debug)]
pub enum BaseType {
    Base(BaseDyn),
    Derived(DerivedType),
}
impl From<BaseDyn> for BaseType {
    fn from(value: BaseDyn) -> Self {
        Self::Base(value)
    }
}
impl From<DerivedType> for BaseType {
    fn from(value: DerivedType) -> Self {
        Self::Derived(value)
    }
}
#[derive(Debug)]
pub struct BaseDyn {
    pub id: UnsignedIntType,
}
#[derive(Debug)]
pub struct DerivedType {
    pub id: UnsignedIntType,
    pub label: StringType,
}
pub type UnsignedIntType = u32;
pub type StringType = String;
