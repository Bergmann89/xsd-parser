use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub enum BaseType {
    #[serde(rename = "ForeignLeaf")]
    ForeignLeaf(ForeignLeafType),
    #[serde(rename = "Leaf")]
    Leaf(LeafType),
    #[serde(rename = "Node")]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct ForeignLeafType {
    #[serde(rename = "@id")]
    pub id: UnsignedIntType,
    #[serde(rename = "@value")]
    pub value: IntType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LeafType {
    #[serde(rename = "@id")]
    pub id: UnsignedIntType,
    #[serde(rename = "@label")]
    pub label: StringType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NodeType {
    #[serde(rename = "@id")]
    pub id: UnsignedIntType,
    #[serde(default, rename = "child")]
    pub child: Option<Box<BaseType>>,
}
pub type UnsignedIntType = u32;
pub type IntType = i32;
pub type StringType = String;
#[derive(Debug, Deserialize, Serialize)]
pub enum MiddleType {
    #[serde(rename = "Leaf")]
    Leaf(LeafType),
}
impl From<LeafType> for MiddleType {
    fn from(value: LeafType) -> Self {
        Self::Leaf(value)
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub enum EmptyType {}
#[derive(Debug, Deserialize, Serialize)]
pub enum OrdinaryType {
    #[serde(rename = "text")]
    Text(StringType),
    #[serde(rename = "number")]
    Number(IntType),
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Head {
    #[serde(rename = "First")]
    First(String),
    #[serde(rename = "Second")]
    Second(String),
    #[serde(rename = "AliasFirst")]
    AliasFirst(String),
    #[serde(rename = "AliasSecond")]
    AliasSecond(String),
    #[serde(rename = "Number")]
    Number(i32),
}
impl From<i32> for Head {
    fn from(value: i32) -> Self {
        Self::Number(value)
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub enum MixedHead {
    #[serde(rename = "Plain")]
    Plain(String),
    #[serde(rename = "Native")]
    Native(String),
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Unique {
    #[serde(rename = "Boolean")]
    Boolean(bool),
}
impl From<bool> for Unique {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
