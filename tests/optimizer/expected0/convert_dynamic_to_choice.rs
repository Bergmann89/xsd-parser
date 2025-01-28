pub struct Abstract(pub Box<dyn AbstractTrait>);
pub trait AbstractTrait: xsd_parser::AsAny {}
pub struct FirstType {
    pub a: String,
}
impl AbstractTrait for FirstType {}
impl xsd_parser::AsAny for FirstType {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}
pub enum SecondType {
    Var1,
    Var2,
}
impl AbstractTrait for SecondType {}
impl xsd_parser::AsAny for SecondType {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}
