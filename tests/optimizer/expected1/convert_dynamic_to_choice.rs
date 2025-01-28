pub struct Abstract {
    pub content: AbstractContent,
}
pub enum AbstractContent {
    First(FirstType),
    Second(SecondType),
}
pub struct FirstType {
    pub a: String,
}
pub enum SecondType {
    Var1,
    Var2,
}
