pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content_2: FooContent2Type,
    pub content_3: FooContent3Type,
}
#[derive(Debug, Clone)]
pub enum FooContent2Type {
    Element1(i32),
    Element2(String),
}
#[derive(Debug, Clone)]
pub enum FooContent3Type {
    Element3(i32),
    Element4(String),
}
