pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub refname: Option<RootRefnameType>,
    pub content: EnumListType,
}
#[derive(Debug)]
pub enum EnumListType {
    _01,
    _02,
}
#[derive(Debug)]
pub enum RootRefnameType {
    EnumValue,
}
