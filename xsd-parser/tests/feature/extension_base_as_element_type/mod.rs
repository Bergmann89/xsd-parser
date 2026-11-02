use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([
        (IdentType::Element, "tns:SomeDetails"),
        (IdentType::Element, "tns:Other"),
    ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/extension_base_as_element_type/schema.xsd",
        "tests/feature/extension_base_as_element_type/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
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
        "tests/feature/extension_base_as_element_type/schema.xsd",
        "tests/feature/extension_base_as_element_type/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::SomeDetails;

    let obj = crate::utils::quick_xml_read_test::<SomeDetails, _>(
        "tests/feature/extension_base_as_element_type/example/default.xml",
    );

    assert_eq!(obj.create_date.as_deref(), Some("2026-01-01T00:00:00"));
    assert_eq!(
        obj.first_info.unwrap().create_date.as_deref(),
        Some("2026-01-02T00:00:00")
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_other() {
    use quick_xml::Other;

    let obj = crate::utils::quick_xml_read_test::<Other, _>(
        "tests/feature/extension_base_as_element_type/example/other.xml",
    );

    assert_eq!(obj.alt.as_deref(), Some("x"));
    assert_eq!(obj.content, "y");
}
