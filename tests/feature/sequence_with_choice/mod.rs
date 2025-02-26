use xsd_parser::{types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/sequence_with_choice/schema.xsd",
        "tests/feature/sequence_with_choice/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/sequence_with_choice/schema.xsd",
        "tests/feature/sequence_with_choice/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{Foo, FooContent3Type, FooContent6Type};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/sequence_with_choice/example/default.xml",
    );

    assert!(matches!(obj.content_3, FooContent3Type::Element1(3)));
    assert!(matches!(
        obj.content_6,
        FooContent6Type::Element4(s) if s == "test"
    ));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, FooContent3Type, FooContent6Type};

    let obj = Foo {
        content_3: FooContent3Type::Element1(3),
        content_6: FooContent6Type::Element4("test".into()),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/sequence_with_choice/example/default.xml",
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
