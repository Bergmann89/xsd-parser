#[derive(Debug)]
pub enum BaseType {
    ForeignLeaf(ForeignLeafType),
    Leaf(LeafType),
    Node(NodeType),
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
#[derive(Debug)]
pub enum MixedHead {
    Plain(String),
    Native(String),
}
#[derive(Debug)]
pub enum Unique {
    Boolean(bool),
}
