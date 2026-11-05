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
    pub content: Vec<base::FlowContentType>,
}
#[derive(Debug)]
pub struct RootCommentType {
    pub author: Option<String>,
    pub content: Vec<base::FlowContentType>,
}
#[derive(Debug)]
pub struct HomeType {
    pub owner: Option<String>,
    pub content: Vec<base::ContactType>,
}
#[derive(Debug)]
pub struct OfficeType {
    pub company: Option<String>,
    pub content: Vec<base::ContactType>,
}
#[derive(Debug)]
pub struct ShopType {
    pub name: Option<String>,
    pub content: Vec<base::AddressContentType>,
}
#[derive(Debug)]
pub struct DepotType {
    pub size: Option<i32>,
    pub content: Vec<base::AddressContentType>,
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
    #[derive(Debug)]
    pub struct FlowContentType {
        pub content: FlowContentTypeContent,
    }
    #[derive(Debug)]
    pub enum FlowContentTypeContent {
        Para(String),
        Break(String),
    }
    #[derive(Debug)]
    pub struct ContactType {
        pub content: ContactTypeContent,
    }
    #[derive(Debug)]
    pub enum ContactTypeContent {
        Phone(String),
        Email(String),
    }
    #[derive(Debug)]
    pub struct AddressContentType {
        pub content: AddressContentTypeContent,
    }
    #[derive(Debug)]
    pub enum AddressContentTypeContent {
        Street(String),
        PostBox(String),
    }
}
