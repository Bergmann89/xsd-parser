pub mod tns {
    pub mod base {
        #[derive(Debug)]
        pub struct PersonType {
            pub name: ::std::string::String,
            pub gender: GenderType,
        }
        #[derive(Debug)]
        pub enum GenderType {
            Male,
            Female,
        }
    }
    pub mod schema1 {
        pub type Persons = PersonsType;
        #[derive(Debug)]
        pub struct PersonsType {
            pub person: ::std::vec::Vec<super::base::PersonType>,
        }
    }
    pub mod schema2 {
        pub type AdvancedPersons = AdvancedPersonsType;
        #[derive(Debug)]
        pub struct AdvancedPersonsType {
            pub person: ::std::vec::Vec<PersonType>,
        }
        #[derive(Debug)]
        pub struct PersonType {
            pub name: ::std::string::String,
            pub gender: super::base::GenderType,
            pub last_name: ::std::string::String,
            pub age: ::core::primitive::i32,
        }
    }
}
