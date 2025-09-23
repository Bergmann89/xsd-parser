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

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($module:ident, $obj:expr) => {{
        use $module::RootTypeContent;

        let obj = $obj;

        let mut it = obj.content.into_iter();
        assert!(matches!(it.next(), Some(RootTypeContent::NegativeDecimal(x)) if x.eq(&-1234.56_f64)));
        assert!(matches!(it.next(), Some(RootTypeContent::PositiveDecimal(x)) if x.eq(&0.01_f64)));
        assert!(matches!(it.next(), Some(RootTypeContent::RestrictedString(x)) if *x == "Abcdef"));
        assert!(it.next().is_none());
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use $module::{RootType, RootTypeContent};

        RootType {
            content: vec![
                RootTypeContent::NegativeDecimal((-1234.56_f64).try_into().unwrap()),
                RootTypeContent::PositiveDecimal(0.011_f64.try_into().unwrap()),
                RootTypeContent::RestrictedString(String::from("Abcdef").try_into().unwrap()),
            ],
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/facets/schema.xsd",
        "tests/feature/facets/expected/default.rs",
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
        "tests/feature/facets/schema.xsd",
        "tests/feature/facets/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::RootType;

    let obj = crate::utils::quick_xml_read_test::<RootType, _>(
        "tests/feature/facets/example/default.xml",
    );

    check_obj!(quick_xml, obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    let obj = test_obj!(quick_xml);

    crate::utils::quick_xml_write_test(&obj, "Values", "tests/feature/facets/example/default.xml");
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
        "tests/feature/facets/schema.xsd",
        "tests/feature/facets/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::RootType;

    let obj = crate::utils::serde_xml_rs_read_test::<RootType, _>(
        "tests/feature/facets/example/default.xml",
    );

    check_obj!(serde_xml_rs, obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_serde_xml_rs() {
    let obj = test_obj!(serde_xml_rs);

    crate::utils::serde_xml_rs_write_test(&obj, "tests/feature/facets/example/default.xml");
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
        "tests/feature/facets/schema.xsd",
        "tests/feature/facets/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
