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
pub struct AnimalType(pub Box<dyn AnimalTypeTrait>);
pub trait AnimalTypeTrait: Debug + AsAny {}
impl AnimalTrait for AnimalType {}
impl AnimalType {
    pub fn new<T: AnimalTypeTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Debug)]
pub struct DogType {
    pub id: i32,
    pub name: String,
}
impl AnimalTypeTrait for DogType {}
impl AnimalTrait for DogType {}
#[derive(Debug)]
pub struct LabradorType {
    pub id: i32,
    pub name: String,
    pub color: String,
}
impl AnimalTypeTrait for LabradorType {}
impl AnimalTrait for LabradorType {}
#[derive(Debug)]
pub struct AnimalDyn {
    pub id: i32,
}
impl AnimalTypeTrait for AnimalDyn {}
