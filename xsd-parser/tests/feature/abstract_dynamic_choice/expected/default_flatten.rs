#[derive(Debug)]
pub enum PayloadType {
    Leaf(LeafType),
}
#[derive(Debug)]
pub struct LeafType {
    pub id: u32,
    pub label: String,
}
#[derive(Debug)]
pub enum MiddleType {
    Leaf(LeafType),
}
#[derive(Debug)]
pub enum EmptyType {}
#[derive(Debug)]
pub enum Head {
    Member(String),
}
#[derive(Debug)]
pub enum AbstractMember {
    Member(String),
}
#[derive(Debug)]
pub enum EmptyHead {}
#[derive(Debug)]
pub enum EmptyAbstractMember {}
pub type Message = MessageType;
#[derive(Debug)]
pub struct MessageType {
    pub payload: PayloadType,
    pub head: Head,
}
