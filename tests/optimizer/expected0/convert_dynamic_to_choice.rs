use xsd_parser::AsAny;
pub struct Abstract(pub Box<dyn AbstractTrait>);
pub trait AbstractTrait: AsAny {}
pub struct FirstType {
    pub a: String,
}
impl AbstractTrait for FirstType {}
pub enum SecondType {
    Var1,
    Var2,
}
impl AbstractTrait for SecondType {}
