use xsd_parser::xml::Nillable;
pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub base: Vec<Base>,
}
#[derive(Debug)]
pub enum Base {
    Intermediate(IntermediateDyn),
    Final(FinalDyn),
}
pub type IntermediateDyn = Nillable<IntermediateType>;
pub type FinalDyn = Nillable<FinalType>;
#[derive(Debug)]
pub struct IntermediateType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
}
#[derive(Debug)]
pub struct FinalType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
    pub final_value: Option<i32>,
}
