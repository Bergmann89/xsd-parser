use core::fmt::Debug;
use xsd_parser_types::AsAny;
pub type Root = RootType;
#[derive(Debug)]
pub struct RootType {
    pub head: Vec<Head>,
}
#[derive(Debug)]
pub struct Head(pub Box<dyn HeadTrait>);
pub trait HeadTrait: Debug + AsAny {}
impl Head {
    pub fn new<T: HeadTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct HeadType {
    pub id: Option<String>,
}
impl HeadTrait for HeadType {}
