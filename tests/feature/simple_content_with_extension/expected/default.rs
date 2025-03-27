pub type SupplierId = SupplierIdType;
#[derive(Debug, Clone)]
pub struct SupplierIdType {
    pub type_: Option<String>,
    pub content: String,
}
pub type UnitName = UnitNameType;
#[derive(Debug, Clone)]
pub struct UnitNameType {
    pub lang: Option<String>,
    pub content: UnitNameContent1Type,
}
#[derive(Debug, Clone)]
pub enum UnitNameContent1Type {
    Unit1,
    Unit2,
    Unit3,
}
