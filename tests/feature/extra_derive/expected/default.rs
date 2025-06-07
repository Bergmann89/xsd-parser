#[derive(Debug, Default, Eq, PartialEq, Hash)]
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
