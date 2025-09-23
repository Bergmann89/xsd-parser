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
        "tests/feature/sequence_with_choice/schema.xsd",
        "tests/feature/sequence_with_choice/expected/default.rs",
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
        "tests/feature/sequence_with_choice/schema.xsd",
        "tests/feature/sequence_with_choice/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{Foo, FooContent2Type, FooContent3Type};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/sequence_with_choice/example/default.xml",
    );

    assert!(matches!(obj.content_2, FooContent2Type::Element1(3)));
    assert!(matches!(
        obj.content_3,
        FooContent3Type::Element4(s) if s == "test"
    ));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, FooContent2Type, FooContent3Type};

    let obj = Foo {
        content_2: FooContent2Type::Element1(3),
        content_3: FooContent3Type::Element4("test".into()),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/sequence_with_choice/example/default.xml",
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
        "tests/feature/sequence_with_choice/schema.xsd",
        "tests/feature/sequence_with_choice/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::{Foo, FooTypeContent};

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/sequence_with_choice/example/default.xml",
    );

    let mut iter = obj.content.into_iter();
    assert!(matches!(iter.next(), Some(FooTypeContent::Element1(3))));
    assert!(matches!(iter.next(), Some(FooTypeContent::Element4(x)) if x == "test"));
    assert!(iter.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_serde_xml_rs() {
    use serde_xml_rs::{Foo, FooTypeContent};

    let obj = Foo {
        content: vec![
            FooTypeContent::Element1(3),
            FooTypeContent::Element4("test".into()),
        ],
    };

    crate::utils::serde_xml_rs_write_test(
        &obj,
        "tests/feature/sequence_with_choice/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}
