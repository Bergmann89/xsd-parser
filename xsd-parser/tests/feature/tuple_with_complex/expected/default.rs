use core::fmt::Debug;
use xsd_parser_types::AsAny;
#[derive(Debug)]
pub struct Foo(pub FooType);
#[derive(Debug)]
pub struct FooType {
    pub name: String,
    pub value: i32,
}
#[derive(Debug)]
pub struct Holder(pub HolderType);
#[derive(Debug)]
pub struct HolderType {
    pub head: Head,
}
#[derive(Debug)]
pub struct Head(pub Box<dyn HeadTrait>);
pub trait HeadTrait: Debug + AsAny {}
impl Head {
    pub fn new<T: HeadTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct MemberType {
    pub info: String,
}
impl HeadTrait for MemberType {}
