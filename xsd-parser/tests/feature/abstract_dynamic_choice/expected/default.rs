#[derive(Debug)]
pub struct PayloadType {
    pub content: PayloadTypeContent,
}
#[derive(Debug)]
pub enum PayloadTypeContent {
    Leaf(LeafType),
}
#[derive(Debug)]
pub struct LeafType {
    pub id: u32,
    pub label: String,
}
#[derive(Debug)]
pub struct MiddleType {
    pub content: MiddleTypeContent,
}
#[derive(Debug)]
pub enum MiddleTypeContent {
    Leaf(LeafType),
}
#[derive(Debug)]
pub struct EmptyType {
    pub content: EmptyTypeContent,
}
#[derive(Debug)]
pub enum EmptyTypeContent {}
#[derive(Debug)]
pub struct Head {
    pub content: HeadContent,
}
#[derive(Debug)]
pub enum HeadContent {
    Member(String),
}
#[derive(Debug)]
pub struct AbstractMember {
    pub content: AbstractMemberContent,
}
#[derive(Debug)]
pub enum AbstractMemberContent {
    Member(String),
}
#[derive(Debug)]
pub struct EmptyHead {
    pub content: EmptyHeadContent,
}
#[derive(Debug)]
pub enum EmptyHeadContent {}
#[derive(Debug)]
pub struct EmptyAbstractMember {
    pub content: EmptyAbstractMemberContent,
}
#[derive(Debug)]
pub enum EmptyAbstractMemberContent {}
pub type Message = MessageType;
#[derive(Debug)]
pub struct MessageType {
    pub content: MessageTypeContent,
}
#[derive(Debug)]
pub struct MessageTypeContent {
    pub payload: PayloadType,
    pub head: Head,
}
