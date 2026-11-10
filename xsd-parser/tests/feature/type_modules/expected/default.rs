pub use self::type_animal_dyn::AnimalDyn;
pub use self::type_animal_type::{AnimalType, AnimalTypeTrait};
pub use self::type_dog_type::DogType;
pub use self::type_labrador_type::LabradorType;
pub use self::type_list::List;
pub use self::type_list_type::ListType;
pub mod type_animal_dyn {
    #[derive(Debug)]
    pub struct AnimalDyn {
        pub id: i32,
    }
    impl super::type_animal_type::AnimalTypeTrait for AnimalDyn {}
}
pub mod type_animal_type {
    use core::fmt::Debug;
    use xsd_parser_types::AsAny;
    #[derive(Debug)]
    pub struct AnimalType(pub Box<dyn AnimalTypeTrait>);
    pub trait AnimalTypeTrait: Debug + AsAny {}
    impl AnimalType {
        pub fn new<T: AnimalTypeTrait + 'static>(value: T) -> Self {
            Self(Box::new(value))
        }
    }
}
pub mod type_dog_type {
    #[derive(Debug)]
    pub struct DogType {
        pub id: i32,
        pub name: String,
    }
    impl super::type_animal_type::AnimalTypeTrait for DogType {}
}
pub mod type_labrador_type {
    #[derive(Debug)]
    pub struct LabradorType {
        pub id: i32,
        pub name: String,
        pub color: String,
    }
    impl super::type_animal_type::AnimalTypeTrait for LabradorType {}
}
pub mod type_list {
    pub type List = super::type_list_type::ListType;
}
pub mod type_list_type {
    #[derive(Debug)]
    pub struct ListType {
        pub animal: Vec<super::type_animal_type::AnimalType>,
    }
}
