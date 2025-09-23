use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    let mut config = Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::MIXED_TYPE_SUPPORT)
        .with_generate([
            (IdentType::Element, "tns:Normal"),
            (IdentType::Element, "tns:Mixed"),
        ]);

    config.generator.type_postfix.element = "Element".into();

    config
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/mixed_content_groups/schema.xsd",
        "tests/feature/mixed_content_groups/expected/default.rs",
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
        "tests/feature/mixed_content_groups/schema.xsd",
        "tests/feature/mixed_content_groups/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_normal() {
    use quick_xml::NormalElement;

    let obj = crate::utils::quick_xml_read_test::<NormalElement, _>(
        "tests/feature/mixed_content_groups/example/example.xml",
    );

    assert_eq!(obj.group.fuu, 111);
    assert_eq!(obj.group.bar, "Hello World");
    assert_eq!(obj.baz, "A string");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_mixed() {
    use quick_xml::MixedElement;
    use xsd_parser::xml::Text;

    let obj = crate::utils::quick_xml_read_test::<MixedElement, _>(
        "tests/feature/mixed_content_groups/example/example.xml",
    );

    assert_eq!(
        obj.text_before.as_ref().map(Text::as_str),
        Some("\n    Text before\n    ")
    );
    assert_eq!(obj.group.fuu.value, 111);
    assert_eq!(
        obj.group.fuu.text_after.as_ref().map(Text::as_str),
        Some("\n    Text after Fuu\n    ")
    );
    assert_eq!(obj.group.bar.value, "Hello World");
    assert_eq!(
        obj.group.bar.text_after.as_ref().map(Text::as_str),
        Some("\n    Text after Bar\n    ")
    );
    assert_eq!(obj.baz.value, "A string");
    assert_eq!(
        obj.baz.text_after.as_ref().map(Text::as_str),
        Some("\n    Text after Baz\n")
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_mixed() {
    use quick_xml::{MixedElement, MixedGroupType};
    use xsd_parser::xml::{Mixed, Text};

    let obj = MixedElement {
        text_before: Some(Text::new("Text before")),
        group: MixedGroupType {
            fuu: Mixed {
                value: 111,
                text_after: Some(Text::new("Text after Fuu")),
            },
            bar: Mixed {
                value: "Hello World".into(),
                text_after: Some(Text::new("Text after Bar")),
            },
        },
        baz: Mixed {
            value: "A string".into(),
            text_after: Some(Text::new("Text after Baz")),
        },
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Example",
        "tests/feature/mixed_content_groups/example/example.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
