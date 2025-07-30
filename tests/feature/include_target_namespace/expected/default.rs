pub type EnumType = EnumType;
#[derive(Debug)]
pub struct EnumType {
    pub refname: Option<EnumTypeRefnameType>,
    pub content: EnumListType,
}
#[derive(Debug)]
pub enum EnumListType {
    _01,
    _02,
}
#[derive(Debug)]
pub enum EnumTypeRefnameType {
    EnumType,
}
