use xsd_parser::{generator::SerdeSupport, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/type_name_clash/schema.xsd",
        "tests/schema/type_name_clash/expected/default.rs",
        Config::default().with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/type_name_clash/schema.xsd",
        "tests/schema/type_name_clash/expected/quick_xml.rs",
        Config::default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/schema/type_name_clash/schema.xsd",
        "tests/schema/type_name_clash/expected/serde_xml_rs.rs",
        Config::default()
            .with_serde(SerdeSupport::SerdeXmlRs)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/schema/type_name_clash/schema.xsd",
        "tests/schema/type_name_clash/expected/serde_quick_xml.rs",
        Config::default()
            .with_serde(SerdeSupport::QuickXml)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/schema/type_name_clash/example/default.xml",
    );

    assert_eq!(obj.bar.a.as_deref(), Some("a-string"));
    assert_eq!(obj.bar.b.as_deref(), Some("b-string"));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, FooTypeBar};

    let obj = Foo {
        bar: FooTypeBar {
            a: Some("a-string".into()),
            b: Some("b-string".into()),
        },
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/schema/type_name_clash/example/serialize.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Foo;

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/schema/type_name_clash/example/serde.xml",
    );

    assert_eq!(obj.bar.a.as_deref(), Some("a-string"));
    assert_eq!(obj.bar.b.as_deref(), Some("b-string"));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Foo;

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/schema/type_name_clash/example/serde.xml",
    );

    assert_eq!(obj.bar.a.as_deref(), Some("a-string"));
    assert_eq!(obj.bar.b.as_deref(), Some("b-string"));
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
