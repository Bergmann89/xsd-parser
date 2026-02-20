use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Schema},
    Config, IdentType,
};

use crate::utils::{generate_test_no_input, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_parser_flags(ParserFlags::all())
        .with_interpreter_flags(InterpreterFlags::all())
        .with_optimizer_flags(OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES)
        .with_generator_flags(GeneratorFlags::all() - GeneratorFlags::ADVANCED_ENUMS)
        .with_schema(Schema::File(
            "tests/feature/redefine/schema/schema1.xsd".into(),
        ))
        .with_schema(Schema::File(
            "tests/feature/redefine/schema/schema2.xsd".into(),
        ))
        .with_generate([
            (IdentType::Element, "tns:Persons"),
            (IdentType::Element, "tns:AdvancedPersons"),
        ])
}

/* default */

#[test]
fn generate_default() {
    generate_test_no_input("tests/feature/redefine/expected/default.rs", config());
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test_no_input(
        "tests/feature/redefine/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_persons() {
    use quick_xml::tns::{base::GenderType, schema_1::Persons};

    let obj = crate::utils::quick_xml_read_test::<Persons, _>(
        "tests/feature/redefine/example/Persons.xml",
    );

    let mut persons = obj.person.into_iter();

    let person = persons.next().unwrap();
    assert_eq!(person.name, "Rusty");
    assert!(matches!(person.gender, GenderType::Male));

    let person = persons.next().unwrap();
    assert_eq!(person.name, "Rustiene");
    assert!(matches!(person.gender, GenderType::Female));

    assert!(persons.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_advanced_persons() {
    use quick_xml::tns::{base::GenderType, schema_2::AdvancedPersons};

    let obj = crate::utils::quick_xml_read_test::<AdvancedPersons, _>(
        "tests/feature/redefine/example/AdvancedPersons.xml",
    );

    let mut persons = obj.person.into_iter();

    let person = persons.next().unwrap();
    assert_eq!(person.name, "Rusty");
    assert_eq!(person.last_name, "Crab");
    assert_eq!(person.age, 24);
    assert!(matches!(person.gender, GenderType::Male));

    let person = persons.next().unwrap();
    assert_eq!(person.name, "Rustiene");
    assert_eq!(person.last_name, "Crab");
    assert_eq!(person.age, 22);
    assert!(matches!(person.gender, GenderType::Female));

    assert!(persons.next().is_none());
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
