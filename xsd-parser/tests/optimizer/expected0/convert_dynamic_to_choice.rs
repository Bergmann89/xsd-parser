use xsd_parser_types::AsAny;
pub struct Abstract(pub Box<dyn AbstractTrait>);
pub trait AbstractTrait: AsAny {}
impl Abstract {
    pub fn new<T: AbstractTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
pub struct FirstType {
    pub a: String,
}
impl AbstractTrait for FirstType {}
pub enum SecondType {
    Var1,
    Var2,
}
impl AbstractTrait for SecondType {}
