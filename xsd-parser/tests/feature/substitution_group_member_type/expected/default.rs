use core::fmt::Debug;
use xsd_parser_types::AsAny;
#[derive(Debug)]
pub struct Head(pub Box<dyn HeadTrait>);
pub trait HeadTrait: Debug + AsAny {}
impl Head {
    pub fn new<T: HeadTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct BaseType {
    pub name: String,
}
impl Chain1Trait for BaseType {}
impl ChainHeadTrait for BaseType {}
impl HeadTrait for BaseType {}
pub type Member = BaseType;
pub type Chain2 = BaseType;
#[derive(Debug)]
pub struct Chain1(pub Box<dyn Chain1Trait>);
pub trait Chain1Trait: Debug + AsAny {}
impl Chain1 {
    pub fn new<T: Chain1Trait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct ChainHead(pub Box<dyn ChainHeadTrait>);
pub trait ChainHeadTrait: Debug + AsAny {}
impl ChainHead {
    pub fn new<T: ChainHeadTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct UntypedHead(pub Box<dyn UntypedHeadTrait>);
pub trait UntypedHeadTrait: Debug + AsAny {}
impl UntypedHead {
    pub fn new<T: UntypedHeadTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct AnyType;
impl UntypedHeadTrait for AnyType {}
pub type UntypedMember = AnyType;
