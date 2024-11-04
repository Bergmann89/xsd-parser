pub struct Abstract(Box<dyn core::any::Any>);
pub struct First {
    pub a: StringType,
}
pub enum Second {
    Var1,
    Var2,
}
pub type StringType = String;
