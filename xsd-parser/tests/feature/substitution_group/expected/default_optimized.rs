pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub animal: Vec<Animal>,
}
#[derive(Debug)]
pub enum Animal {
    Animal(AnimalType),
    Dog(DogType),
    Labrador(LabradorType),
}
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
