use xsd_parser::{config::InterpreterFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_interpreter_flags(InterpreterFlags::WITH_XS_ANY_SIMPLE_TYPE)
        .with_generate([(IdentType::Element, "tns:Foo")])
}
/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/any_simple_type/schema-default.xsd",
        "tests/feature/any_simple_type/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml_default */

#[test]
fn generate_quick_xml_default() {
    generate_test(
        "tests/feature/any_simple_type/schema-default.xsd",
        "tests/feature/any_simple_type/expected/quick_xml_default.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_default() {
    use quick_xml_default::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/any_simple_type/example/default.xml",
    );

    assert_eq!(2, obj.value.len());
    assert_eq!(obj.value[0].type_.as_deref(), Some("xs:string"));
    assert_eq!(obj.value[0].content, "Test");
    assert_eq!(obj.value[1].type_.as_deref(), Some("xs:integer"));
    assert_eq!(obj.value[1].content, "12345678");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_default() {
    use quick_xml_default::{AnySimpleType, Foo};

    let obj = Foo {
        value: vec![
            AnySimpleType {
                type_: Some(String::from("xs:string")),
                content: String::from("Test"),
            },
            AnySimpleType {
                type_: Some(String::from("xs:integer")),
                content: String::from("12345678"),
            },
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/any_simple_type/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_default {
    #![allow(unused_imports)]

    include!("expected/quick_xml_default.rs");
}

/* quick_xml_custom_type */

#[test]
fn generate_quick_xml_custom_type() {
    generate_test(
        "tests/feature/any_simple_type/schema-custom-type.xsd",
        "tests/feature/any_simple_type/expected/quick_xml_custom_type.rs",
        config()
            .with_quick_xml()
            .with_xs_any_simple_type("xsd_parser_types::xml::AnySimpleType"),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_custom_type() {
    use quick_xml_custom_type::Foo;
    use xsd_parser_types::xml::{AnySimpleType, Integer};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/any_simple_type/example/custom.xml",
    );

    assert_eq!(2, obj.value.len());
    assert!(matches!(&obj.value[0], AnySimpleType::String(x) if x == "Test"));
    assert!(matches!(&obj.value[1], AnySimpleType::Integer(x) if *x == Integer::from(12345678)));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_custom_type() {
    use quick_xml_custom_type::Foo;
    use xsd_parser_types::xml::AnySimpleType;

    let obj = Foo {
        value: vec![
            AnySimpleType::String(String::from("Test")),
            AnySimpleType::Integer(12345678.into()),
        ],
        test_attrib: Some(AnySimpleType::Integer(87654321.into())),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/any_simple_type/example/custom.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_custom_type {
    #![allow(unused_imports)]

    include!("expected/quick_xml_custom_type.rs");
}
