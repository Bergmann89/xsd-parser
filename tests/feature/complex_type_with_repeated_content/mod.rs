use xsd_parser::{types::IdentType, Config};

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

    let item = items.next().unwrap();
    assert_eq!(item.a, 3);
    assert_eq!(item.b, "String B-3");
    assert_eq!(item.c.as_deref(), Some("String C-3"));

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
            FooTypeContent {
                a: 3,
                b: "String B-3".into(),
                c: Some("String C-3".into()),
            },
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/complex_type_with_repeated_content/example/default.xml",
    );
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
