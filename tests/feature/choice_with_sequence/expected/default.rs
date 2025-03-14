pub type Foo = FooType;
#[derive(Debug, Clone)]
pub enum FooType {
    Content3(FooContent3Type),
    Content6(FooContent6Type),
}
#[derive(Debug, Clone)]
pub struct FooContent3Type {
    pub element_1: i32,
    pub element_2: String,
}
#[derive(Debug, Clone)]
pub struct FooContent6Type {
    pub element_3: i32,
    pub element_4: String,
}
