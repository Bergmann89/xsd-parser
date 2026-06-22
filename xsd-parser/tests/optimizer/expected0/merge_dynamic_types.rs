use xsd_parser_types::AsAny;
pub struct Animal(pub Box<dyn AnimalTrait>);
pub trait AnimalTrait: AsAny {}
impl Animal {
    pub fn new<T: AnimalTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
pub struct AnimalType(pub Box<dyn AnimalTypeTrait>);
pub trait AnimalTypeTrait: AsAny {}
impl AnimalTrait for AnimalType {}
impl AnimalType {
    pub fn new<T: AnimalTypeTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
pub struct DogType {
    pub id: i32,
    pub name: String,
}
impl AnimalTypeTrait for DogType {}
impl AnimalTrait for DogType {}
pub struct LabradorType {
    pub id: i32,
    pub name: String,
    pub color: String,
}
impl AnimalTypeTrait for LabradorType {}
impl AnimalTrait for LabradorType {}
pub struct AnimalDyn {
    pub id: i32,
}
impl AnimalTypeTrait for AnimalDyn {}
