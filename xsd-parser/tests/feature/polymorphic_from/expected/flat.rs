#[derive(Debug)]
pub enum BaseType {
    ForeignLeaf(ForeignLeafType),
    Leaf(LeafType),
    Node(NodeType),
}
impl From<ForeignLeafType> for BaseType {
    fn from(value: ForeignLeafType) -> Self {
        Self::ForeignLeaf(value)
    }
}
impl From<LeafType> for BaseType {
    fn from(value: LeafType) -> Self {
        Self::Leaf(value)
    }
}
impl From<NodeType> for BaseType {
    fn from(value: NodeType) -> Self {
        Self::Node(value)
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
    pub child: Option<Box<BaseType>>,
}
pub type UnsignedIntType = u32;
pub type IntType = i32;
pub type StringType = String;
#[derive(Debug)]
pub enum MiddleType {
    Leaf(LeafType),
}
impl From<LeafType> for MiddleType {
    fn from(value: LeafType) -> Self {
        Self::Leaf(value)
    }
}
#[derive(Debug)]
pub enum EmptyType {}
#[derive(Debug)]
pub enum OrdinaryType {
    Text(StringType),
    Number(IntType),
}
#[derive(Debug)]
pub enum Head {
    First(String),
    Second(String),
    AliasFirst(String),
    AliasSecond(String),
    Number(i32),
}
impl From<i32> for Head {
    fn from(value: i32) -> Self {
        Self::Number(value)
    }
}
#[derive(Debug)]
pub enum MixedHead {
    Plain(String),
    Native(String),
}
#[derive(Debug)]
pub enum Unique {
    Boolean(bool),
}
impl From<bool> for Unique {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
