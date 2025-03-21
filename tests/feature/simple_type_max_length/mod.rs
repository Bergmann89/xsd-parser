use xsd_parser::{generator::SerdeSupport, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/simple_type_max_length/schema.xsd",
        "tests/feature/simple_type_max_length/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "tns:Name")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/simple_type_max_length/schema.xsd",
        "tests/feature/simple_type_max_length/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Name")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/simple_type_max_length/schema.xsd",
        "tests/feature/simple_type_max_length/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::SerdeXmlRs)
            .with_generate([(IdentType::Element, "tns:Name")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/simple_type_max_length/schema.xsd",
        "tests/feature/simple_type_max_length/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::QuickXml)
            .with_generate([(IdentType::Element, "tns:Name")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Name;

    let obj = crate::utils::quick_xml_read_test::<Name, _>(
        "tests/feature/simple_type_max_length/example/default.xml",
    );

    assert_eq!(obj, "string");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Name;

    let obj = crate::utils::serde_xml_rs_read_test::<Name, _>(
        "tests/feature/simple_type_max_length/example/default.xml",
    );

    assert_eq!(obj, "string");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Name;

    let obj = crate::utils::serde_quick_xml_read_test::<Name, _>(
        "tests/feature/simple_type_max_length/example/default.xml",
    );

    assert_eq!(obj, "string");
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
