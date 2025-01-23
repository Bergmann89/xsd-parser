pub struct Base(Box<dyn core::any::Any>);
pub struct Intermediate(Box<dyn core::any::Any>);
pub struct FinalType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
    pub final_value: Option<i32>,
}
pub struct IntermediateType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
}
