pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content_2: FooContent2Type,
    pub content_5: FooContent5Type,
}
#[derive(Debug, Clone)]
pub struct FooContent2Type {
    pub content: FooContent2TypeContent,
}
#[derive(Debug, Clone)]
pub enum FooContent2TypeContent {
    Element1(i32),
    Element2(String),
}
#[derive(Debug, Clone)]
pub struct FooContent5Type {
    pub content: FooContent5TypeContent,
}
#[derive(Debug, Clone)]
pub enum FooContent5TypeContent {
    Element3(i32),
    Element4(String),
}
