use xsd_parser::{config::GeneratorFlags, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    let mut config = Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Foo")]);

    config.generator.any_type = Some("xsd_parser::xml::AnyElement".into());
    config.generator.any_attribute_type = Some("xsd_parser::xml::AnyAttributes".into());

    config
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/any/schema.xsd",
        "tests/feature/any/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/any/schema.xsd",
        "tests/feature/any/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{ChoiceTypeContent, Foo};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/any/example/default.xml");

    // <tns:Name />
    assert_eq!("abc", obj.name);
    assert_eq!(1, obj.any_attribute.len());
    assert_eq!(
        b"fuu",
        obj.any_attribute.get(b"any".as_ref()).unwrap().as_ref()
    );

    // <xs:any />
    assert_eq!(2, obj.any_0.len());
    assert_eq!(b"AnyElement", obj.any_0[0].name.as_ref());
    assert_eq!(b"AnotherElement", obj.any_0[1].name.as_ref());

    // <tns:Choice />
    assert_eq!(3, obj.choice.len());
    assert_eq!(
        b"bar",
        obj.choice[0]
            .any_attribute
            .get(b"any".as_ref())
            .unwrap()
            .as_ref()
    );
    assert!(matches!(&obj.choice[0].content, ChoiceTypeContent::Name(name) if name == "def"));
    assert!(
        matches!(&obj.choice[1].content, ChoiceTypeContent::Any(x) if x.name.as_ref() == b"AnyElement2")
    );
    assert!(
        matches!(&obj.choice[2].content, ChoiceTypeContent::Any(x) if x.name.as_ref() == b"AnotherElement2")
    );

    // <xs:any />
    assert_eq!(1, obj.any_1.len());
    assert_eq!(b"LastElement", obj.any_1[0].name.as_ref());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{ChoiceType, ChoiceTypeContent, FooType};

    use xsd_parser::xml::{AnyAttributes, AttributeKey, AttributeValue, Element};

    let obj = FooType {
        any_attribute: AnyAttributes::from_iter([(
            AttributeKey(b"any".into()),
            AttributeValue(b"fuu".into()),
        )]),
        name: "abc".into(),
        any_0: vec![
            Element::default().name(b"AnyElement"),
            Element::default().name(b"AnotherElement"),
        ],
        choice: vec![
            ChoiceType {
                any_attribute: AnyAttributes::from_iter([(
                    AttributeKey(b"any".into()),
                    AttributeValue(b"bar".into()),
                )]),
                content: ChoiceTypeContent::Name("def".into()),
            },
            ChoiceType {
                any_attribute: AnyAttributes::default(),
                content: ChoiceTypeContent::Any(Element::default().name(b"AnyElement2")),
            },
            ChoiceType {
                any_attribute: AnyAttributes::default(),
                content: ChoiceTypeContent::Any(Element::default().name(b"AnotherElement2")),
            },
        ],
        any_1: vec![Element::default().name(b"LastElement")],
    };

    crate::utils::quick_xml_write_test(&obj, "tns:Foo", "tests/feature/any/example/default.xml");
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
