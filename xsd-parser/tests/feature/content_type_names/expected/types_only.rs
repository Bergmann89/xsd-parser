pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub note: RootNoteType,
    pub comment: RootCommentType,
    pub home: HomeType,
    pub office: OfficeType,
    pub shop: ShopType,
    pub depot: DepotType,
    pub location: base::AddressType,
    pub ul: UlType,
    pub ol: OlType,
}
#[derive(Debug)]
pub struct RootNoteType {
    pub lang: Option<String>,
    pub content: Vec<RootNoteTypeContent>,
}
#[derive(Debug)]
pub enum RootNoteTypeContent {
    Para(String),
    Break(String),
}
#[derive(Debug)]
pub struct RootCommentType {
    pub author: Option<String>,
    pub content: Vec<RootCommentTypeContent>,
}
#[derive(Debug)]
pub enum RootCommentTypeContent {
    Para(String),
    Break(String),
}
#[derive(Debug)]
pub struct HomeType {
    pub owner: Option<String>,
    pub content: Vec<HomeTypeContent>,
}
#[derive(Debug)]
pub enum HomeTypeContent {
    Phone(String),
    Email(String),
}
#[derive(Debug)]
pub struct OfficeType {
    pub company: Option<String>,
    pub content: Vec<OfficeTypeContent>,
}
#[derive(Debug)]
pub enum OfficeTypeContent {
    Phone(String),
    Email(String),
}
#[derive(Debug)]
pub struct ShopType {
    pub name: Option<String>,
    pub content: Vec<ShopTypeContent>,
}
#[derive(Debug)]
pub enum ShopTypeContent {
    Street(String),
    PostBox(String),
}
#[derive(Debug)]
pub struct DepotType {
    pub size: Option<i32>,
    pub content: Vec<DepotTypeContent>,
}
#[derive(Debug)]
pub enum DepotTypeContent {
    Street(String),
    PostBox(String),
}
#[derive(Debug)]
pub struct UlType {
    pub compact: Option<bool>,
    pub content: Vec<UlTypeContent>,
}
#[derive(Debug)]
pub enum UlTypeContent {
    Item(String),
    Text(String),
}
#[derive(Debug)]
pub struct OlType {
    pub start: Option<i32>,
    pub content: Vec<OlTypeContent>,
}
#[derive(Debug)]
pub enum OlTypeContent {
    Item(String),
    Text(String),
}
pub mod base {
    #[derive(Debug)]
    pub struct AddressType {
        pub id: Option<String>,
    }
}
