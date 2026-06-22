use xsd_parser_types::AsAny;
pub struct Animal(pub Box<dyn AnimalTrait>);
pub trait AnimalTrait: AsAny {}
impl Animal {
    pub fn new<T: AnimalTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
pub struct AnimalDyn {
    pub id: i32,
}
impl AnimalTrait for AnimalDyn {}
pub struct DogType {
    pub id: i32,
    pub name: String,
}
impl AnimalTrait for DogType {}
pub struct LabradorType {
    pub id: i32,
    pub name: String,
    pub color: String,
}
impl AnimalTrait for LabradorType {}
