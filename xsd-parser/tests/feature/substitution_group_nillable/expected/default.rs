use core::fmt::Debug;
use xsd_parser_types::{xml::Nillable, AsAny};
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
pub type AnimalDyn = Nillable<AnimalType>;
impl AnimalTrait for AnimalDyn {}
pub type DogDyn = Nillable<DogType>;
impl AnimalTrait for DogDyn {}
pub type LabradorDyn = Nillable<LabradorType>;
impl AnimalTrait for LabradorDyn {}
#[derive(Debug)]
pub struct AnimalType {
    pub id: i32,
}
#[derive(Debug)]
pub struct DogType {
    pub id: i32,
    pub name: String,
}
#[derive(Debug)]
pub struct LabradorType {
    pub id: i32,
    pub name: String,
    pub color: String,
}
