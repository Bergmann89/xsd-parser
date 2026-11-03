use xsd_parser::{config::OptimizerFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .without_optimizer_flags(OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE_SIMPLE)
        .with_generate([
            (IdentType::Element, "tns:Value"),
            (IdentType::Element, "tns:Inline"),
            (IdentType::Element, "tns:Extended"),
            (IdentType::Element, "tns:Restricted"),
            (IdentType::Type, "tns:BaseType"),
        ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/multi_pattern_derivation/schema.xsd",
        "tests/feature/multi_pattern_derivation/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/multi_pattern_derivation/schema.xsd",
        "tests/feature/multi_pattern_derivation/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn derived_patterns_are_anded() {
    use quick_xml::{BaseType, DerivedType, InlineType};

    for value in ["abc", "123"] {
        assert!(DerivedType::validate_str(value).is_ok(), "{value}");
    }
    for value in ["xyz", "789", "ABC", "!!"] {
        assert!(DerivedType::validate_str(value).is_err(), "{value}");
    }

    assert!(BaseType::validate_str("xyz").is_ok());
    assert!(BaseType::validate_str("ABC").is_err());

    assert!(InlineType::validate_str("abc").is_ok());
    assert!(InlineType::validate_str("xyz").is_err());
    assert!(InlineType::validate_str("123").is_err());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn simple_content_restriction_patterns_are_anded() {
    use quick_xml::RestrictedContent1Type;

    assert!(RestrictedContent1Type::validate_str("abc").is_ok());
    assert!(RestrictedContent1Type::validate_str("xyz").is_err());
    assert!(RestrictedContent1Type::validate_str("123").is_err());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_valid() {
    use quick_xml::Value;

    let obj = crate::utils::quick_xml_read_test::<Value, _>(
        "tests/feature/multi_pattern_derivation/example/valid.xml",
    );

    assert_eq!(&*obj, "abc");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_invalid() {
    use quick_xml::Value;
    use xsd_parser_types::quick_xml::{ErrorKind, ValidateError};

    let err = crate::utils::quick_xml_read_test_result::<Value, _>(
        "tests/feature/multi_pattern_derivation/example/invalid.xml",
    )
    .unwrap_err();

    assert!(matches!(
        err.kind,
        ErrorKind::InvalidValue(_, ValidateError::Pattern(_))
    ));
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
