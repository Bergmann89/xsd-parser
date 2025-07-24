use xsd_parser::{config::SerdeXmlRsVersion, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_serde_xml_rs_v7() {
    generate_test(
        "tests/feature/empty_string/schema.xsd",
        "tests/feature/empty_string/expected/serde_xml_rs_v7.rs",
        Config::test_default().with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow),
    );
}

#[test]
fn generate_serde_xml_rs_v8() {
    generate_test(
        "tests/feature/empty_string/schema.xsd",
        "tests/feature/empty_string/expected/serde_xml_rs_v8.rs",
        Config::test_default().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/empty_string/schema.xsd",
        "tests/feature/empty_string/expected/serde_quick_xml.rs",
        Config::test_default().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs_v7() {
    use serde_xml_rs_v7::{ComplexContent, SimpleContent};

    let obj = crate::utils::serde_xml_rs_v7_read_test::<SimpleContent, _>(
        "tests/feature/empty_string/example/simple.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");

    let obj = crate::utils::serde_xml_rs_v7_read_test::<ComplexContent, _>(
        "tests/feature/empty_string/example/complex.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs_v8() {
    use serde_xml_rs_v8::{ComplexContent, SimpleContent};

    let obj = crate::utils::serde_xml_rs_read_test::<SimpleContent, _>(
        "tests/feature/empty_string/example/simple.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");

    let obj = crate::utils::serde_xml_rs_read_test::<ComplexContent, _>(
        "tests/feature/empty_string/example/complex.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::{ComplexContent, SimpleContent};

    let obj = crate::utils::serde_quick_xml_read_test::<SimpleContent, _>(
        "tests/feature/empty_string/example/simple.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");

    let obj = crate::utils::serde_quick_xml_read_test::<ComplexContent, _>(
        "tests/feature/empty_string/example/complex.xml",
    );

    assert_eq!(obj.lang, "");
    assert_eq!(obj.content, "");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v7 {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs_v7.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v8 {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs_v8.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
