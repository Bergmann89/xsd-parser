use core::fmt::Debug;
use xsd_parser_types::AsAny;
pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub animal: Vec<Animal>,
}
#[derive(Debug)]
pub struct Animal(pub Box<dyn AnimalTrait>);
pub trait AnimalTrait: Debug + AsAny {}
impl Animal {
    pub fn new<T: AnimalTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct AnimalType {
    pub id: i32,
}
impl AnimalTrait for AnimalType {}
#[derive(Debug)]
pub struct DogType {
    pub id: i32,
    pub name: String,
}
impl AnimalTrait for DogType {}
#[derive(Debug)]
pub struct LabradorType {
    pub id: i32,
    pub name: String,
    pub color: String,
}
impl AnimalTrait for LabradorType {}
