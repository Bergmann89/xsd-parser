use xsd_parser::{
    config::{OptimizerFlags, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .without_optimizer_flags(OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE_SIMPLE)
        .with_generate([(IdentType::Element, "tns:OperatingMode")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/multi_pattern_alternatives/schema.xsd",
        "tests/feature/multi_pattern_alternatives/expected/default.rs",
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
        "tests/feature/multi_pattern_alternatives/schema.xsd",
        "tests/feature/multi_pattern_alternatives/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_hybrid() {
    use quick_xml::{OperatingMode, OperatingModeType};

    assert!(OperatingModeType::validate_str("hybrid/auto").is_ok());
    assert!(OperatingModeType::validate_str("heat pump only").is_ok());
    assert!(OperatingModeType::validate_str("electric heater only").is_ok());

    let obj = crate::utils::quick_xml_read_test::<OperatingMode, _>(
        "tests/feature/multi_pattern_alternatives/example/hybrid.xml",
    );

    assert_eq!(&*obj, "hybrid/auto");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_heat_pump() {
    use quick_xml::OperatingMode;

    let obj = crate::utils::quick_xml_read_test::<OperatingMode, _>(
        "tests/feature/multi_pattern_alternatives/example/heat-pump.xml",
    );

    assert_eq!(&*obj, "heat pump only");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_electric() {
    use quick_xml::OperatingMode;

    let obj = crate::utils::quick_xml_read_test::<OperatingMode, _>(
        "tests/feature/multi_pattern_alternatives/example/electric.xml",
    );

    assert_eq!(&*obj, "electric heater only");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_invalid() {
    use quick_xml::OperatingMode;
    use xsd_parser_types::quick_xml::{ErrorKind, ValidateError};

    let err = crate::utils::quick_xml_read_test_result::<OperatingMode, _>(
        "tests/feature/multi_pattern_alternatives/example/invalid.xml",
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
        "tests/feature/multi_pattern_alternatives/schema.xsd",
        "tests/feature/multi_pattern_alternatives/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn validate_serde_xml_rs_patterns() {
    use serde_xml_rs::OperatingModeType;

    assert!(OperatingModeType::validate_str("hybrid/auto").is_ok());
    assert!(OperatingModeType::validate_str("heat pump only").is_ok());
    assert!(OperatingModeType::validate_str("electric heater only").is_ok());
    assert!(OperatingModeType::validate_str("invalid value").is_err());
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
        "tests/feature/multi_pattern_alternatives/schema.xsd",
        "tests/feature/multi_pattern_alternatives/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn validate_serde_quick_xml_patterns() {
    use serde_quick_xml::OperatingModeType;

    assert!(OperatingModeType::validate_str("hybrid/auto").is_ok());
    assert!(OperatingModeType::validate_str("heat pump only").is_ok());
    assert!(OperatingModeType::validate_str("electric heater only").is_ok());
    assert!(OperatingModeType::validate_str("invalid value").is_err());
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
