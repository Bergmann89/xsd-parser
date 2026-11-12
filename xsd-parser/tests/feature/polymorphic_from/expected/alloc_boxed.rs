#[derive(Debug)]
pub enum BaseType {
    ForeignLeaf(::alloc::boxed::Box<ForeignLeafType>),
    Leaf(::alloc::boxed::Box<LeafType>),
    Node(::alloc::boxed::Box<NodeType>),
}
impl ::core::convert::From<ForeignLeafType> for BaseType {
    fn from(value: ForeignLeafType) -> Self {
        Self::ForeignLeaf(::alloc::boxed::Box::new(value))
    }
}
impl ::core::convert::From<LeafType> for BaseType {
    fn from(value: LeafType) -> Self {
        Self::Leaf(::alloc::boxed::Box::new(value))
    }
}
impl ::core::convert::From<NodeType> for BaseType {
    fn from(value: NodeType) -> Self {
        Self::Node(::alloc::boxed::Box::new(value))
    }
}
#[derive(Debug)]
pub struct ForeignLeafType {
    pub id: UnsignedIntType,
    pub value: IntType,
}
#[derive(Debug)]
pub struct LeafType {
    pub id: UnsignedIntType,
    pub label: StringType,
}
#[derive(Debug)]
pub struct NodeType {
    pub id: UnsignedIntType,
    pub child: ::core::option::Option<::alloc::boxed::Box<BaseType>>,
}
pub type UnsignedIntType = ::core::primitive::u32;
pub type IntType = ::core::primitive::i32;
pub type StringType = ::std::string::String;
#[derive(Debug)]
pub enum MiddleType {
    Leaf(::alloc::boxed::Box<LeafType>),
}
impl ::core::convert::From<LeafType> for MiddleType {
    fn from(value: LeafType) -> Self {
        Self::Leaf(::alloc::boxed::Box::new(value))
    }
}
#[derive(Debug)]
pub enum EmptyType {}
#[derive(Debug)]
pub enum OrdinaryType {
    Text(::alloc::boxed::Box<StringType>),
    Number(::alloc::boxed::Box<IntType>),
}
#[derive(Debug)]
pub enum Head {
    First(::alloc::boxed::Box<::std::string::String>),
    Second(::alloc::boxed::Box<::std::string::String>),
    AliasFirst(::alloc::boxed::Box<::std::string::String>),
    AliasSecond(::alloc::boxed::Box<::std::string::String>),
    Number(::alloc::boxed::Box<::core::primitive::i32>),
}
impl ::core::convert::From<::core::primitive::i32> for Head {
    fn from(value: ::core::primitive::i32) -> Self {
        Self::Number(::alloc::boxed::Box::new(value))
    }
}
#[derive(Debug)]
pub enum MixedHead {
    Plain(::alloc::boxed::Box<::std::string::String>),
    Native(::alloc::boxed::Box<::std::string::String>),
}
#[derive(Debug)]
pub enum Unique {
    Boolean(::alloc::boxed::Box<::core::primitive::bool>),
}
impl ::core::convert::From<::core::primitive::bool> for Unique {
    fn from(value: ::core::primitive::bool) -> Self {
        Self::Boolean(::alloc::boxed::Box::new(value))
    }
}
