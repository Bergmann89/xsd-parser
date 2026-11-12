#[derive(Debug)]
pub struct BaseType {
    pub content: BaseTypeContent,
}
#[derive(Debug)]
pub enum BaseTypeContent {
    ForeignLeaf(ForeignLeafType),
    Leaf(LeafType),
    Node(NodeType),
}
impl From<ForeignLeafType> for BaseTypeContent {
    fn from(value: ForeignLeafType) -> Self {
        Self::ForeignLeaf(value)
    }
}
impl From<LeafType> for BaseTypeContent {
    fn from(value: LeafType) -> Self {
        Self::Leaf(value)
    }
}
impl From<NodeType> for BaseTypeContent {
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
    pub content: NodeTypeContent,
}
#[derive(Debug)]
pub struct NodeTypeContent {
    pub child: Option<Box<BaseType>>,
}
pub type UnsignedIntType = u32;
pub type IntType = i32;
pub type StringType = String;
#[derive(Debug)]
pub struct MiddleType {
    pub content: MiddleTypeContent,
}
#[derive(Debug)]
pub enum MiddleTypeContent {
    Leaf(LeafType),
}
impl From<LeafType> for MiddleTypeContent {
    fn from(value: LeafType) -> Self {
        Self::Leaf(value)
    }
}
#[derive(Debug)]
pub struct EmptyType {
    pub content: EmptyTypeContent,
}
#[derive(Debug)]
pub enum EmptyTypeContent {}
#[derive(Debug)]
pub struct OrdinaryType {
    pub content: OrdinaryTypeContent,
}
#[derive(Debug)]
pub enum OrdinaryTypeContent {
    Text(StringType),
    Number(IntType),
}
#[derive(Debug)]
pub struct Head {
    pub content: HeadContent,
}
#[derive(Debug)]
pub enum HeadContent {
    First(String),
    Second(String),
    AliasFirst(String),
    AliasSecond(String),
    Number(i32),
}
impl From<i32> for HeadContent {
    fn from(value: i32) -> Self {
        Self::Number(value)
    }
}
#[derive(Debug)]
pub struct MixedHead {
    pub content: MixedHeadContent,
}
#[derive(Debug)]
pub enum MixedHeadContent {
    Plain(String),
    Native(String),
}
#[derive(Debug)]
pub struct Unique {
    pub content: UniqueContent,
}
#[derive(Debug)]
pub enum UniqueContent {
    Boolean(bool),
}
impl From<bool> for UniqueContent {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
