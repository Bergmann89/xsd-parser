pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub animal: Vec<Animal>,
}
#[derive(Debug)]
pub enum Animal {
    Cat(CatType),
    Dog(DogType),
}
#[derive(Debug)]
pub struct CatType {
    pub id: i32,
    pub breed: String,
}
#[derive(Debug)]
pub enum DogType {
    Labrador(LabradorType),
    Beagle(BeagleType),
}
#[derive(Debug)]
pub struct LabradorType {
    pub id: i32,
    pub name: String,
    pub color: String,
}
#[derive(Debug)]
pub struct BeagleType {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
