pub type SupplierId = SupplierIdType;
#[derive(Debug)]
pub struct SupplierIdType {
    pub type_: Option<String>,
    pub content: String,
}
pub type UnitName = UnitNameType;
#[derive(Debug)]
pub struct UnitNameType {
    pub lang: Option<String>,
    pub content: UnitNameContent1Type,
}
#[derive(Debug)]
pub enum UnitNameContent1Type {
    Unit1,
    Unit2,
    Unit3,
}
