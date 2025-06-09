use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/numeric_fields/schema.xsd",
        "tests/feature/numeric_fields/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/numeric_fields/schema.xsd",
        "tests/feature/numeric_fields/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/numeric_fields/schema.xsd",
        "tests/feature/numeric_fields/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_xml_rs()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/numeric_fields/schema.xsd",
        "tests/feature/numeric_fields/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{EnumType, Foo};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/numeric_fields/example/default.xml",
    );

    assert!(matches!(obj._4, EnumType::_1));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{EnumType, Foo};

    let obj = Foo { _4: EnumType::_1 };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/numeric_fields/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::{EnumType, Foo};

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/numeric_fields/example/default.xml",
    );

    assert!(matches!(obj._4, EnumType::_1));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::{EnumType, Foo};

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/numeric_fields/example/default.xml",
    );

    assert!(matches!(obj._4, EnumType::_1));
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
