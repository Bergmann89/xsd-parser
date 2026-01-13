#[derive(Debug)]
pub struct OuterType {
    pub bar_inner: OuterBarInnerType,
    pub baz_inner: OuterBazInnerType,
}
#[derive(Debug)]
pub struct OuterBarInnerType {
    pub a: String,
}
#[derive(Debug)]
pub struct OuterBazInnerType {
    pub b: String,
}
