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
pub struct CatType {
    pub id: i32,
    pub breed: String,
}
impl AnimalTrait for CatType {}
#[derive(Debug)]
pub struct DogType(pub Box<dyn DogTypeTrait>);
pub trait DogTypeTrait: Debug + AsAny {}
impl AnimalTrait for DogType {}
impl DogType {
    pub fn new<T: DogTypeTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct LabradorType {
    pub id: i32,
    pub name: String,
    pub color: String,
}
impl DogTypeTrait for LabradorType {}
#[derive(Debug)]
pub struct BeagleType {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
impl DogTypeTrait for BeagleType {}
