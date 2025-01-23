pub struct Abstract(Box<dyn core::any::Any>);
pub struct FirstType {
    pub a: String,
}
pub enum SecondType {
    Var1,
    Var2,
}
