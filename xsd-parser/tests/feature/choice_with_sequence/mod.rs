use xsd_parser::{
    config::{GeneratorFlags, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Foo")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/choice_with_sequence/schema.xsd",
        "tests/feature/choice_with_sequence/expected/default.rs",
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
        "tests/feature/choice_with_sequence/schema.xsd",
        "tests/feature/choice_with_sequence/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/choice_with_sequence/example/default.xml",
    );

    let Foo::Content2(content) = obj else {
        panic!("Expected `Foo::Content2`");
    };

    assert_eq!(content.element_1, 3);
    assert_eq!(content.element_2, "test");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, FooContent2Type};

    let obj = Foo::Content2(FooContent2Type {
        element_1: 3,
        element_2: "test".into(),
    });

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/choice_with_sequence/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* serde_xml_rs */

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/choice_with_sequence/schema.xsd",
        "tests/feature/choice_with_sequence/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::{Foo, FooTypeContent};

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/choice_with_sequence/example/default.xml",
    );

    let mut it = obj.content.into_iter();
    assert!(matches!(it.next(), Some(FooTypeContent::Element1(3))));
    assert!(matches!(it.next(), Some(FooTypeContent::Element2(x)) if x == "test"));
    assert!(it.next().is_none());
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}
