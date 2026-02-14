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
        .with_generator_flags(GeneratorFlags::all())
        .with_schema(Schema::File(
            "tests/feature/override_/schema/schema1.xsd".into(),
        ))
        .with_schema(Schema::File(
            "tests/feature/override_/schema/schema2.xsd".into(),
        ))
        .with_generate([
            (IdentType::Element, "tns:Persons"),
            (IdentType::Element, "tns:AdvancedPersons"),
        ])
}

/* default */

#[test]
fn generate_default() {
    generate_test_no_input("tests/feature/override_/expected/default.rs", config());
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml */
/*
TODO
#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/all/schema.xsd",
        "tests/feature/all/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/all/example/default.xml");

    check_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    let obj = test_obj!(quick_xml);

    crate::utils::quick_xml_write_test(&obj, "tns:Foo", "tests/feature/all/example/serialize.xml");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
*/
