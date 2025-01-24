#[derive(Debug)]
pub struct Base(pub Box<dyn BaseTrait>);
pub trait BaseTrait: core::fmt::Debug + xsd_parser::AsAny {}
#[derive(Debug)]
pub struct IntermediateType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
}
impl BaseTrait for IntermediateType {}
impl IntermediateTrait for IntermediateType {}
impl xsd_parser::AsAny for IntermediateType {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}
#[derive(Debug)]
pub struct FinalType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
    pub final_value: Option<i32>,
}
impl BaseTrait for FinalType {}
impl IntermediateTrait for FinalType {}
impl xsd_parser::AsAny for FinalType {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}
#[derive(Debug)]
pub struct Intermediate(pub Box<dyn IntermediateTrait>);
pub trait IntermediateTrait: BaseTrait {}
