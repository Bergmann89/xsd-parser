use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/list/schema.xsd",
        "tests/feature/list/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/list/schema.xsd",
        "tests/feature/list/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/list/example/default.xml");

    assert_eq!(
        obj.a_list.0,
        vec![
            String::from("one"),
            String::from("two"),
            String::from("three")
        ]
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, ListType};

    let obj = Foo {
        a_list: ListType(vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ]),
    };

    crate::utils::quick_xml_write_test(&obj, "Foo", "tests/feature/list/example/default.xml");
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
