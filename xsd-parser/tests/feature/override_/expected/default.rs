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
    pub mod schema_1 {
        pub type Persons = PersonsType;
        #[derive(Debug)]
        pub struct PersonsType {
            pub person: ::std::vec::Vec<super::base::PersonType>,
        }
    }
    pub mod schema_2 {
        pub type AdvancedPersons = AdvancedPersonsType;
        #[derive(Debug)]
        pub struct AdvancedPersonsType {
            pub person: ::std::vec::Vec<PersonType>,
        }
        #[derive(Debug)]
        pub struct PersonType {
            pub name: ::std::string::String,
            pub last_name: ::std::string::String,
            pub age: ::core::primitive::i32,
            pub gender: super::base::GenderType,
        }
    }
}
