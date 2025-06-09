use xsd_parser::{config::SerdeXmlRsVersion, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "tns:Foo")]),
    );
}
#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_xml_rs_v7() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/serde_xml_rs_v7.rs",
        Config::test_default()
            .with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/extension_simple_content/schema.xsd",
        "tests/feature/extension_simple_content/expected/serde_quick_xml.rs",
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
        "tests/feature/extension_simple_content/example/default.xml",
    );

    assert_eq!(obj.value.as_deref(), Some("an attribute value"));
    assert_eq!(obj.another_value.as_deref(), Some("more attribute data"));
    assert!(matches!(obj.content, EnumType::Auto));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{EnumType, FooType};

    let obj = FooType {
        value: Some("an attribute value".into()),
        another_value: Some("more attribute data".into()),
        content: EnumType::Auto,
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/extension_simple_content/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::{EnumTypeValue, Foo};

    let obj = dbg!(crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/extension_simple_content/example/default.xml",
    ));

    assert_eq!(obj.value.as_deref(), Some("an attribute value"));
    assert_eq!(obj.another_value.as_deref(), Some("more attribute data"));
    assert!(matches!(obj.content, EnumTypeValue::Auto));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs_v7() {
    use serde_xml_rs_v7::{EnumType, Foo};

    let obj = dbg!(crate::utils::serde_xml_rs_v7_read_test::<Foo, _>(
        "tests/feature/extension_simple_content/example/default.xml",
    ));

    assert_eq!(obj.value.as_deref(), Some("an attribute value"));
    assert_eq!(obj.another_value.as_deref(), Some("more attribute data"));
    assert!(matches!(obj.content, EnumType::Auto));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::{EnumType, Foo};

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/extension_simple_content/example/default.xml",
    );

    assert_eq!(obj.value.as_deref(), Some("an attribute value"));
    assert_eq!(obj.another_value.as_deref(), Some("more attribute data"));
    assert!(matches!(obj.content, EnumType::Auto));
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
