use xsd_parser::{config::SerdeXmlRsVersion, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Foo")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/xsd_string/schema.xsd",
        "tests/feature/xsd_string/expected/default.rs",
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
        "tests/feature/xsd_string/schema.xsd",
        "tests/feature/xsd_string/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj =
        crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/xsd_string/example/default.xml");

    assert_eq!(obj.text, "abcd");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::Foo;

    let obj = Foo {
        text: "abcd".into(),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/xsd_string/example/default.xml",
    );
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
        "tests/feature/xsd_string/schema.xsd",
        "tests/feature/xsd_string/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Foo;

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/xsd_string/example/default.xml",
    );

    assert_eq!(obj.text, "abcd");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(dead_code, unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

/* serde_quick_xml */

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/xsd_string/schema.xsd",
        "tests/feature/xsd_string/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(dead_code, unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
