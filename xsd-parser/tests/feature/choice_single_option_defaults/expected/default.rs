pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub content: FooTypeContent,
}
#[derive(Debug)]
pub enum FooTypeContent {
    Content2(Vec<FooContent2Type>),
}
#[derive(Debug)]
pub struct FooContent2Type {
    pub info: String,
}
pub type Bar = BarType;
#[derive(Debug)]
pub struct BarType {
    pub content: BarTypeContent,
}
#[derive(Debug)]
pub enum BarTypeContent {
    Content4(Vec<BarContent4Type>),
}
#[derive(Debug)]
pub struct BarContent4Type {
    pub info: String,
}
