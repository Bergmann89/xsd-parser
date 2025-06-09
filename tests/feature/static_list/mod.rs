use xsd_parser::{config::SerdeXmlRsVersion, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "tns:Array")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Array")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove)
            .with_generate([(IdentType::Element, "tns:Array")]),
    );
}

#[test]
fn generate_serde_xml_rs_v7() {
    generate_test(
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/serde_xml_rs_v7.rs",
        Config::test_default()
            .with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow)
            .with_generate([(IdentType::Element, "tns:Array")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_quick_xml()
            .with_generate([(IdentType::Element, "tns:Array")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Array;

    let obj = crate::utils::quick_xml_read_test::<Array, _>(
        "tests/feature/static_list/example/default.xml",
    );

    assert_eq!(obj.item, [111, 222, 333, 444, 555]);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::Array;

    let obj = Array {
        item: [111, 222, 333, 444, 555],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Array",
        "tests/feature/static_list/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Array;

    let obj = crate::utils::serde_xml_rs_read_test::<Array, _>(
        "tests/feature/static_list/example/default.xml",
    );

    assert_eq!(obj.item, [111, 222, 333, 444, 555]);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs_v7() {
    use serde_xml_rs_v7::Array;

    let obj = crate::utils::serde_xml_rs_v7_read_test::<Array, _>(
        "tests/feature/static_list/example/default.xml",
    );

    assert_eq!(obj.item, [111, 222, 333, 444, 555]);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Array;

    let obj = crate::utils::serde_quick_xml_read_test::<Array, _>(
        "tests/feature/static_list/example/default.xml",
    );

    assert_eq!(obj.item, [111, 222, 333, 444, 555]);
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
mod serde_xml_rs_v7 {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs_v7.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
