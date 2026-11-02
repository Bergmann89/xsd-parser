use std::path::Path;

use xsd_parser::{
    config::{GeneratorFlags, Schema},
    generate, Config, Error, GeneratorError, IdentType,
};

use crate::utils::ConfigEx;

#[test]
fn cross_namespace_collision_is_an_error() {
    let err = generate(config("schema.xsd")).unwrap_err();

    let Error::GeneratorError(GeneratorError::DuplicateTypeName {
        path,
        first,
        second,
    }) = err
    else {
        panic!("unexpected error: {err:?}");
    };

    assert_eq!(path, "KindType");
    assert_eq!(first, "{http://example.com}Type(kind)");
    assert_eq!(second, "{http://example.com/other}Type(kind)");
}

#[test]
fn use_modules_avoids_cross_namespace_collision() {
    let config = config("schema.xsd").with_generator_flags(GeneratorFlags::USE_MODULES);

    generate(config).unwrap();
}

#[test]
fn postfix_collision_with_type_is_an_error() {
    let err = generate(config("postfix_type.xsd")).unwrap_err();

    let Error::GeneratorError(GeneratorError::DuplicateTypeName {
        path,
        first,
        second,
    }) = err
    else {
        panic!("unexpected error: {err:?}");
    };

    assert_eq!(path, "FooType");
    assert_eq!(first, "{http://example.com}Type(Foo)");
    assert_eq!(second, "{http://example.com}Type(FooType)");
}

#[test]
fn postfix_collision_with_element_is_an_error() {
    let err = generate(config("postfix_element.xsd")).unwrap_err();

    let Error::GeneratorError(GeneratorError::DuplicateTypeName {
        path,
        first,
        second,
    }) = err
    else {
        panic!("unexpected error: {err:?}");
    };

    assert_eq!(path, "BarType");
    assert_eq!(first, "{http://example.com}Type(Bar)");
    assert_eq!(second, "{http://example.com}ElementType(Bar)");
}

#[test]
fn content_type_collision_is_an_error() {
    let mut config =
        config("content.xsd").without_generator_flags(GeneratorFlags::FLATTEN_STRUCT_CONTENT);
    config.generator.type_postfix.type_ = String::new();

    let err = generate(config).unwrap_err();

    let Error::GeneratorError(GeneratorError::DuplicateTypeName {
        path,
        first,
        second,
    }) = err
    else {
        panic!("unexpected error: {err:?}");
    };

    assert_eq!(path, "FooContent");
    assert_eq!(first, "{http://example.com}Type(FooContent)");
    assert_eq!(second, "{http://example.com}Type(Foo)");
}

#[test]
fn collision_with_fixed_type_is_an_error() {
    let mut config = config("postfix_type.xsd").with_generator_flags(GeneratorFlags::USE_MODULES);
    config
        .generator
        .types
        .push((IdentType::Type, "tns:FooType").into());

    let err = generate(config).unwrap_err();

    let Error::GeneratorError(GeneratorError::DuplicateTypeName {
        path,
        first,
        second,
    }) = err
    else {
        panic!("unexpected error: {err:?}");
    };

    assert_eq!(path, "tns::FooType");
    assert_eq!(first, "{http://example.com}Type(FooType)");
    assert_eq!(second, "{http://example.com}Type(Foo)");
}

fn config(file: &str) -> Config {
    let mut config = Config::test_default().with_generate([(IdentType::Element, "tns:Root")]);
    let xsd = Path::new("tests/feature/duplicate_type_name")
        .join(file)
        .canonicalize()
        .unwrap();
    config.parser.schemas.push(Schema::File(xsd));

    config
}
