use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_any_type_support()
        .with_generator_flags(GeneratorFlags::MIXED_TYPE_SUPPORT)
        .with_generate([(IdentType::Element, "tns:AttributeValue")])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($module:ident, $obj:expr) => {{
        use xsd_parser_types::xml::Text;

        let obj = $obj;

        assert_eq!(obj.base_attrib, "base attrib content");
        assert_eq!(obj.data_type, "data type attrib content");
        assert!(matches!(
            obj.text_before.as_ref().map(Text::as_str),
            Some("\n    text before\n    ")
        ));
        assert_eq!(obj.base_element.value, "base element content");
        assert!(matches!(
            obj.base_element.text_after.as_ref().map(Text::as_str),
            Some("\n    text in the middle\n    ")
        ));

        let mut attribs = obj.any_attribute.attributes();
        assert!(matches!(attribs.next(), Some(_)));
        assert!(attribs.next().is_none());

        let mut elements = obj.any.into_iter();
        let element = elements.next().unwrap();
        assert_eq!(&element.name[..], &b"unknown-element"[..]);
        assert!(matches!(
            element.text_after.as_ref().map(Text::as_str),
            Some("\n    text after\n")
        ));
        assert!(elements.next().is_none());
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use xsd_parser_types::xml::{AnyAttributes, AnyElement, Mixed};
        use $module::AttributeValue;

        let mut any_attribute = AnyAttributes::default();
        any_attribute.insert(b"unknown-attrib", b"unknown content");

        AttributeValue {
            base_attrib: "base attrib content".into(),
            data_type: "data type attrib content".into(),
            any_attribute,
            text_before: Some("text before".into()),
            base_element: Mixed {
                value: "base element content".into(),
                text_after: Some("text in the middle".into()),
            },
            any: vec![Mixed {
                value: AnyElement::new().name(b"unknown-element"),
                text_after: Some("text after".into()),
            }],
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/extension_mixed_content/schema.xsd",
        "tests/feature/extension_mixed_content/expected/default.rs",
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
        "tests/feature/extension_mixed_content/schema.xsd",
        "tests/feature/extension_mixed_content/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::AttributeValue;

    let obj = crate::utils::quick_xml_read_test::<AttributeValue, _>(
        "tests/feature/extension_mixed_content/example/default.xml",
    );

    check_obj!(quick_xml, obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    let obj = test_obj!(quick_xml);

    crate::utils::quick_xml_write_test(
        &obj,
        "AttributeValue",
        "tests/feature/extension_mixed_content/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
