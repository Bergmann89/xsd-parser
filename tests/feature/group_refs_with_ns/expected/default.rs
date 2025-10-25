pub type Outer = OuterType;
#[derive(Debug)]
pub enum OuterType {
    Inner(OuterInnerType),
    BarInner(OuterBarInnerType),
    BazInner(OuterBazInnerType),
}
#[derive(Debug)]
pub struct OuterInnerType {
    pub value: String,
}
#[derive(Debug)]
pub struct OuterBarInnerType {
    pub value: String,
}
#[derive(Debug)]
pub struct OuterBazInnerType {
    pub value: String,
}
