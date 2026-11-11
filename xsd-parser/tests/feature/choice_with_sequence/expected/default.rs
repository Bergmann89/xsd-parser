pub type Foo = FooType;
#[derive(Debug)]
pub enum FooType {
    Content2(FooContent2Type),
    Content3(FooContent3Type),
}
#[derive(Debug)]
pub struct FooContent2Type {
    pub element1: i32,
    pub element2: String,
}
#[derive(Debug)]
pub struct FooContent3Type {
    pub element3: i32,
    pub element4: String,
}
