pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content_2: FooContent2Type,
    pub content_3: FooContent3Type,
}
#[derive(Debug, Clone)]
pub struct FooContent2Type {
    pub content: FooContent2TypeContent,
}
#[derive(Debug, Clone)]
pub enum FooContent2TypeContent {
    Element1(IntType),
    Element2(StringType),
}
#[derive(Debug, Clone)]
pub struct FooContent3Type {
    pub content: FooContent3TypeContent,
}
#[derive(Debug, Clone)]
pub enum FooContent3TypeContent {
    Element3(IntType),
    Element4(StringType),
}
pub type IntType = i32;
pub type StringType = String;
