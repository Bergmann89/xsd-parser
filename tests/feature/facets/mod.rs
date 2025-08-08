use xsd_parser::{
    config::{OptimizerFlags, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .without_optimizer_flags(OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE_SIMPLE)
        .with_generate([(IdentType::Element, "tns:Root")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/facets/schema.xsd",
        "tests/feature/facets/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/facets/schema.xsd",
        "tests/feature/facets/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/facets/schema.xsd",
        "tests/feature/facets/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/facets/schema.xsd",
        "tests/feature/facets/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{RootType, RootTypeContent};

    let obj = crate::utils::quick_xml_read_test::<RootType, _>(
        "tests/feature/facets/example/default.xml",
    );

    let mut it = obj.content.into_iter();
    assert!(matches!(it.next(), Some(RootTypeContent::NegativeDecimal(x)) if x.eq(&-1234.56_f64)));
    assert!(matches!(it.next(), Some(RootTypeContent::PositiveDecimal(x)) if x.eq(&0.01_f64)));
    assert!(matches!(it.next(), Some(RootTypeContent::RestrictedString(x)) if *x == "Abcdef"));
    assert!(it.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{RootType, RootTypeContent};

    let obj = RootType {
        content: vec![
            RootTypeContent::NegativeDecimal((-1234.56_f64).try_into().unwrap()),
            RootTypeContent::PositiveDecimal(0.011_f64.try_into().unwrap()),
            RootTypeContent::RestrictedString(String::from("Abcdef").try_into().unwrap()),
        ],
    };

    crate::utils::quick_xml_write_test(&obj, "Values", "tests/feature/facets/example/default.xml");
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
