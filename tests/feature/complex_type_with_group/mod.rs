use xsd_parser::{config::SerdeXmlRsVersion, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/complex_type_with_group/schema.xsd",
        "tests/feature/complex_type_with_group/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/complex_type_with_group/schema.xsd",
        "tests/feature/complex_type_with_group/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/complex_type_with_group/schema.xsd",
        "tests/feature/complex_type_with_group/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/complex_type_with_group/schema.xsd",
        "tests/feature/complex_type_with_group/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{Foo, FooTypeContent};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/complex_type_with_group/example/default.xml",
    );

    assert!(matches!(obj.content, FooTypeContent::Baz(3)));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, FooTypeContent};

    let obj = Foo {
        content: FooTypeContent::Baz(3),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/complex_type_with_group/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::{Foo, FooTypeContent};

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/complex_type_with_group/example/default.xml",
    );

    assert!(matches!(obj.content, FooTypeContent::Baz(3)));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::{Foo, FooTypeContent};

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/complex_type_with_group/example/default.xml",
    );

    assert!(matches!(obj.content, FooTypeContent::Baz(3)));
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
