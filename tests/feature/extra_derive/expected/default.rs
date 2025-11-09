#[derive(Debug, Default, Eq, Hash, PartialEq)]
pub struct FooType {
    pub a: Option<String>,
}
pub struct BarType {
    pub b: Option<String>,
}
#[derive(Debug)]
pub struct BazType {
    pub c: Option<String>,
}
