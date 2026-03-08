use xsd_parser::{
    config::{OptimizerFlags, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .without_optimizer_flags(OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE_SIMPLE)
        .with_generate([(IdentType::Element, "tns:Voltage")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/multi_pattern_facets/schema.xsd",
        "tests/feature/multi_pattern_facets/expected/default.rs",
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
        "tests/feature/multi_pattern_facets/schema.xsd",
        "tests/feature/multi_pattern_facets/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Voltage;

    let obj = crate::utils::quick_xml_read_test::<Voltage, _>(
        "tests/feature/multi_pattern_facets/example/default.xml",
    );

    assert_eq!(*obj, "120V");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_invalid() {
    use quick_xml::Voltage;
    use xsd_parser_types::quick_xml::{ErrorKind, ValidateError};

    let err = crate::utils::quick_xml_read_test_result::<Voltage, _>(
        "tests/feature/multi_pattern_facets/example/invalid.xml",
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

/* serde_xml_rs */

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/multi_pattern_facets/schema.xsd",
        "tests/feature/multi_pattern_facets/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Voltage;

    let obj = crate::utils::serde_xml_rs_read_test::<Voltage, _>(
        "tests/feature/multi_pattern_facets/example/default.xml",
    );

    assert_eq!(*obj, "120V");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

/* serde_quick_xml */

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/multi_pattern_facets/schema.xsd",
        "tests/feature/multi_pattern_facets/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Voltage;

    let obj = crate::utils::serde_quick_xml_read_test::<Voltage, _>(
        "tests/feature/multi_pattern_facets/example/default.xml",
    );

    assert_eq!(*obj, "120V");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
