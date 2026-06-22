use xsd_parser_types::xml::Nillable;
pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub animal: Vec<Animal>,
}
#[derive(Debug)]
pub enum Animal {
    Animal(AnimalDyn),
    Dog(DogDyn),
    Labrador(LabradorDyn),
}
pub type AnimalDyn = Nillable<AnimalType>;
pub type DogDyn = Nillable<DogType>;
pub type LabradorDyn = Nillable<LabradorType>;
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
