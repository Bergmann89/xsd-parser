use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Foo")])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($obj:expr) => {{
        let obj = $obj;

        let mut items = obj.content.iter();

        let item = items.next().unwrap();
        assert_eq!(item.a, 1);
        assert_eq!(item.b.as_deref(), Some("String B-1"));
        assert_eq!(item.c.as_deref(), Some("String C-1"));

        let item = items.next().unwrap();
        assert_eq!(item.a, 2);
        assert_eq!(item.b.as_deref(), Some("String B-2"));
        assert_eq!(item.c.as_deref(), None);

        let item = items.next().unwrap();
        assert_eq!(item.a, 3);
        assert_eq!(item.b.as_deref(), None);
        assert_eq!(item.c.as_deref(), None);

        let item = items.next().unwrap();
        assert_eq!(item.a, 4);
        assert_eq!(item.b.as_deref(), Some("String B-4"));
        assert_eq!(item.c.as_deref(), Some("String C-4"));

        assert!(items.next().is_none());
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use $module::{Foo, FooTypeContent};

        Foo {
            content: vec![
                FooTypeContent {
                    a: 1,
                    b: Some("String B-1".into()),
                    c: Some("String C-1".into()),
                },
                FooTypeContent {
                    a: 2,
                    b: Some("String B-2".into()),
                    c: None,
                },
                FooTypeContent {
                    a: 3,
                    b: None,
                    c: None,
                },
                FooTypeContent {
                    a: 4,
                    b: Some("String B-4".into()),
                    c: Some("String C-4".into()),
                },
            ],
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/complex_type_with_repeated_content/schema.xsd",
        "tests/feature/complex_type_with_repeated_content/expected/default.rs",
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
        "tests/feature/complex_type_with_repeated_content/schema.xsd",
        "tests/feature/complex_type_with_repeated_content/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/complex_type_with_repeated_content/example/default.xml",
    );

    check_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    let obj = test_obj!(quick_xml);

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/complex_type_with_repeated_content/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
