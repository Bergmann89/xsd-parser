use xsd_parser::{config::SerdeXmlRsVersion, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Array")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/default.rs",
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
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/quick_xml.rs",
        config().with_quick_xml(),
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
        "Array",
        "tests/feature/static_list/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* serde_xml_rs v0.8 */

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
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
fn write_serde_xml_rs() {
    use serde_xml_rs::Array;

    let obj = Array {
        item: vec![111, 222, 333, 444, 555],
    };

    crate::utils::serde_xml_rs_write_test(&obj, "tests/feature/static_list/example/default.xml");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

/* serde_xml_rs v0.7 */

#[test]
fn generate_serde_xml_rs_v7() {
    generate_test(
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/serde_xml_rs_v7.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow),
    );
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
fn write_serde_xml_rs_v7() {
    use serde_xml_rs::Array;

    let obj = Array {
        item: vec![111, 222, 333, 444, 555],
    };

    crate::utils::serde_xml_rs_v7_write_test(&obj, "tests/feature/static_list/example/default.xml");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs_v7 {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs_v7.rs");
}

/* serde_quick_xml */

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/static_list/schema.xsd",
        "tests/feature/static_list/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
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
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
