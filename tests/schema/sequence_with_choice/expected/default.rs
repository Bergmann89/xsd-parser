pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content_3: FooContent3Type,
    pub content_6: FooContent6Type,
}
#[derive(Debug, Clone)]
pub struct FooContent3Type {
    pub content: FooContent3TypeContent,
}
#[derive(Debug, Clone)]
pub enum FooContent3TypeContent {
    Element1(i32),
    Element2(String),
}
#[derive(Debug, Clone)]
pub struct FooContent6Type {
    pub content: FooContent6TypeContent,
}
#[derive(Debug, Clone)]
pub enum FooContent6TypeContent {
    Element3(i32),
    Element4(String),
}
