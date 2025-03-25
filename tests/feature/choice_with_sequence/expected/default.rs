pub type Foo = FooType;
#[derive(Debug, Clone)]
pub enum FooType {
    Content2(FooContent2Type),
    Content3(FooContent3Type),
}
#[derive(Debug, Clone)]
pub struct FooContent2Type {
    pub element_1: i32,
    pub element_2: String,
}
#[derive(Debug, Clone)]
pub struct FooContent3Type {
    pub element_3: i32,
    pub element_4: String,
}
