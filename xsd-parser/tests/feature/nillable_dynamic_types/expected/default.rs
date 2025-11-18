use core::fmt::Debug;
use xsd_parser_types::{xml::Nillable, AsAny};
pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub base: Vec<Base>,
}
#[derive(Debug)]
pub struct Base(pub Box<dyn BaseTrait>);
pub trait BaseTrait: Debug + AsAny {}
pub type IntermediateDyn = Nillable<IntermediateType>;
impl BaseTrait for IntermediateDyn {}
impl IntermediateTrait for IntermediateDyn {}
pub type FinalDyn = Nillable<FinalType>;
impl BaseTrait for FinalDyn {}
impl IntermediateTrait for FinalDyn {}
#[derive(Debug)]
pub struct IntermediateType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
}
#[derive(Debug)]
pub struct Intermediate(pub Box<dyn IntermediateTrait>);
pub trait IntermediateTrait: BaseTrait {}
#[derive(Debug)]
pub struct FinalType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
    pub final_value: Option<i32>,
}
