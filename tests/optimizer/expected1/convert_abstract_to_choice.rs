pub struct Abstract {
    pub content: AbstractContent,
}
pub enum AbstractContent {
    First(First),
    Second(Second),
}
pub struct First {
    pub a: StringType,
}
pub enum Second {
    Var1,
    Var2,
}
pub type StringType = String;
