use xsd_parser::{generator::SerdeSupport, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/complex_type_with_repeated_content/schema.xsd",
        "tests/feature/complex_type_with_repeated_content/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/complex_type_with_repeated_content/schema.xsd",
        "tests/feature/complex_type_with_repeated_content/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/complex_type_with_repeated_content/schema.xsd",
        "tests/feature/complex_type_with_repeated_content/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::SerdeXmlRs)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/complex_type_with_repeated_content/schema.xsd",
        "tests/feature/complex_type_with_repeated_content/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::QuickXml)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/complex_type_with_repeated_content/example/default.xml",
    );

    let mut items = obj.content.iter();

    let item = items.next().unwrap();
    assert_eq!(item.a, 1);
    assert_eq!(item.b, "String B-1");
    assert_eq!(item.c.as_deref(), Some("String C-1"));

    let item = items.next().unwrap();
    assert_eq!(item.a, 2);
    assert_eq!(item.b, "String B-2");
    assert_eq!(item.c.as_deref(), None);

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, FooTypeContent};

    let obj = Foo {
        content: vec![
            FooTypeContent {
                a: 1,
                b: "String B-1".into(),
                c: Some("String C-1".into()),
            },
            FooTypeContent {
                a: 2,
                b: "String B-2".into(),
                c: None,
            },
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/complex_type_with_repeated_content/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Foo;

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/complex_type_with_repeated_content/example/default.xml",
    );

    let mut items = obj.content.iter();

    let item = items.next().unwrap();
    assert_eq!(item.a, 1);
    assert_eq!(item.b, "String B-1");
    assert_eq!(item.c.as_deref(), Some("String C-1"));

    let item = items.next().unwrap();
    assert_eq!(item.a, 2);
    assert_eq!(item.b, "String B-2");
    assert_eq!(item.c.as_deref(), None);

    assert!(items.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Foo;

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/complex_type_with_repeated_content/example/default.xml",
    );

    let mut items = obj.content.iter();

    let item = items.next().unwrap();
    assert_eq!(item.a, 1);
    assert_eq!(item.b, "String B-1");
    assert_eq!(item.c.as_deref(), Some("String C-1"));

    let item = items.next().unwrap();
    assert_eq!(item.a, 2);
    assert_eq!(item.b, "String B-2");
    assert_eq!(item.c.as_deref(), None);

    assert!(items.next().is_none());
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
