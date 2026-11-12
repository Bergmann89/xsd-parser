use xsd_parser::{
    config::{Generate, GeneratorFlags, Schema},
    generate, Config, Error, GeneratorError, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Option")])
}

#[test]
fn shadowed_build_in_is_an_error() {
    let err = generate(config().with_schema(Schema::File(
        "tests/feature/shadowed_build_in/schema.xsd".into(),
    )))
    .unwrap_err();

    let Error::GeneratorError(GeneratorError::ShadowedBuildIn { name, type_ }) = err else {
        panic!("unexpected error: {err:?}");
    };

    assert_eq!(name, "Option");
    assert_eq!(type_, "{http://example.com}Element(Option)");
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/shadowed_build_in/schema.xsd",
        "tests/feature/shadowed_build_in/expected/quick_xml.rs",
        config()
            .with_quick_xml()
            .with_generator_flags(GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");

    #[test]
    fn read_quick_xml() {
        let obj = crate::utils::quick_xml_read_test::<Option, _>(
            "tests/feature/shadowed_build_in/example/default.xml",
        );

        assert_eq!(obj.value.as_deref(), Some("text"));
    }
}

#[test]
fn empty_type_postfix_shadows_string() {
    let mut config = Config::test_default()
        .with_generator_flags(GeneratorFlags::USE_MODULES)
        .with_schema(Schema::File(
            "tests/feature/shadowed_build_in/schema.xsd".into(),
        ));
    config.generator.generate = Generate::Named;
    config.generator.type_postfix.type_ = String::new();

    let err = generate(config).unwrap_err();

    let Error::GeneratorError(GeneratorError::ShadowedBuildIn { name, type_ }) = err else {
        panic!("unexpected error: {err:?}");
    };

    assert_eq!(name, "String");
    assert_eq!(type_, "{http://www.w3.org/2001/XMLSchema}Type(string)");
}
