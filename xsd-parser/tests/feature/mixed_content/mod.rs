use xsd_parser::{
    config::{GeneratorFlags, OptimizerFlags, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_optimizer_flags(OptimizerFlags::SIMPLIFY_MIXED_TYPES)
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::MIXED_TYPE_SUPPORT)
        .with_generate([
            (IdentType::Element, "tns:MixedAll"),
            (IdentType::Element, "tns:MixedChoice"),
            (IdentType::Element, "tns:MixedChoiceList"),
            (IdentType::Element, "tns:MixedSequence"),
        ])
}

fn config_serde() -> Config {
    Config::test_default()
        .with_optimizer_flags(OptimizerFlags::SIMPLIFY_MIXED_TYPES)
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::MIXED_TYPE_SUPPORT)
        .with_generate([(IdentType::Element, "tns:MixedChoiceList")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/mixed_content/schema.xsd",
        "tests/feature/mixed_content/expected/default.rs",
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
        "tests/feature/mixed_content/schema.xsd",
        "tests/feature/mixed_content/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_all() {
    use quick_xml::MixedAll;
    use xsd_parser_types::xml::Text;

    let obj = crate::utils::quick_xml_read_test::<MixedAll, _>(
        "tests/feature/mixed_content/example/all.xml",
    );

    assert_eq!(
        obj.text_before.as_ref().map(Text::as_str),
        Some("\n    Text before &\n    ")
    );
    assert_eq!(obj.fuu.value, 111);
    assert_eq!(
        obj.fuu.text_after.as_ref().map(Text::as_str),
        Some("\n    Text between\n    ")
    );
    assert_eq!(obj.bar.value, "Hello World");
    assert_eq!(
        obj.bar.text_after.as_ref().map(Text::as_str),
        Some("\n    Text after ä\n")
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_choice_single() {
    use quick_xml::{MixedChoice, MixedChoiceTypeContent};
    use xsd_parser_types::xml::Text;

    let obj = crate::utils::quick_xml_read_test::<MixedChoice, _>(
        "tests/feature/mixed_content/example/choice.xml",
    );

    assert_eq!(
        obj.text_before.as_ref().map(Text::as_str),
        Some("\n    Text before &\n    ")
    );

    let MixedChoiceTypeContent::Fuu(mixed) = &obj.content else {
        panic!(
            "Expected `MixedChoiceTypeContent::Fuu(_)` but got {:?}",
            obj.content
        );
    };

    assert_eq!(mixed.value, 111);
    assert_eq!(
        mixed.text_after.as_ref().map(Text::as_str),
        Some("\n    Text after ä\n")
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_choice_list() {
    use quick_xml::{MixedChoiceList, MixedChoiceListTypeContent};

    let obj = crate::utils::quick_xml_read_test::<MixedChoiceList, _>(
        "tests/feature/mixed_content/example/all.xml",
    );

    let mut it = obj.content.into_iter();

    assert!(
        matches!(it.next().unwrap(), MixedChoiceListTypeContent::Text(x) if x.0 == "\n    Text before &\n    ")
    );
    assert!(matches!(
        it.next().unwrap(),
        MixedChoiceListTypeContent::Fuu(111)
    ));
    assert!(
        matches!(it.next().unwrap(), MixedChoiceListTypeContent::Text(x) if x.0 == "\n    Text between\n    ")
    );
    assert!(matches!(it.next().unwrap(), MixedChoiceListTypeContent::Bar(x) if x == "Hello World"));
    assert!(
        matches!(it.next().unwrap(), MixedChoiceListTypeContent::Text(x) if x.0 == "\n    Text after ä\n")
    );
    assert!(it.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_sequence() {
    use quick_xml::MixedSequence;
    use xsd_parser_types::xml::Text;

    let obj = crate::utils::quick_xml_read_test::<MixedSequence, _>(
        "tests/feature/mixed_content/example/sequence.xml",
    );

    assert_eq!(
        obj.text_before.as_ref().map(Text::as_str),
        Some("\n    Text before &\n    ")
    );
    assert_eq!(obj.fuu, 111);
    assert_eq!(
        obj.text_after_fuu.as_ref().map(Text::as_str),
        Some("\n    Text between\n    ")
    );
    assert_eq!(obj.bar, "Hello World");
    assert_eq!(
        obj.text_after_bar.as_ref().map(Text::as_str),
        Some("\n    Text after ä\n")
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_all() {
    use quick_xml::MixedAll;
    use xsd_parser_types::xml::{Mixed, Text};

    let obj = MixedAll {
        text_before: Some(Text::new("Text before &")),
        fuu: Mixed {
            value: 111,
            text_after: Some(Text::new("Text between")),
        },
        bar: Mixed {
            value: "Hello World".into(),
            text_after: Some(Text::new("Text after ä")),
        },
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:MixedAll",
        "tests/feature/mixed_content/example/all.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_choice_single() {
    use quick_xml::{MixedChoice, MixedChoiceTypeContent};
    use xsd_parser_types::xml::{Mixed, Text};

    let obj = MixedChoice {
        text_before: Some(Text::new("Text before &")),
        content: MixedChoiceTypeContent::Fuu(Mixed {
            value: 111,
            text_after: Some(Text::new("Text after ä")),
        }),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:MixedChoice",
        "tests/feature/mixed_content/example/choice.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_choice_list() {
    use quick_xml::{MixedChoiceList, MixedChoiceListTypeContent};

    let obj = MixedChoiceList {
        content: vec![
            MixedChoiceListTypeContent::Text("\n    Text before &\n    ".into()),
            MixedChoiceListTypeContent::Fuu(111),
            MixedChoiceListTypeContent::Text("\n    Text between\n    ".into()),
            MixedChoiceListTypeContent::Bar("Hello World".into()),
            MixedChoiceListTypeContent::Text("\n    Text after ä\n".into()),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:MixedAll",
        "tests/feature/mixed_content/example/all.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_sequence() {
    use quick_xml::MixedSequence;
    use xsd_parser_types::xml::Text;

    let obj = MixedSequence {
        text_before: Some(Text::new("Text before &")),
        fuu: 111,
        text_after_fuu: Some(Text::new("Text between")),
        bar: "Hello World".into(),
        text_after_bar: Some(Text::new("Text after ä")),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:MixedSequence",
        "tests/feature/mixed_content/example/sequence.xml",
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
        "tests/feature/mixed_content/schema.xsd",
        "tests/feature/mixed_content/expected/serde_xml_rs.rs",
        config_serde().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

/* serde_quick_xml */

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/mixed_content/schema.xsd",
        "tests/feature/mixed_content/expected/serde_quick_xml.rs",
        config_serde().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml_choice_list() {
    use serde_quick_xml::{MixedChoiceList, MixedChoiceListTypeContent};

    let obj = crate::utils::serde_quick_xml_read_test::<MixedChoiceList, _>(
        "tests/feature/mixed_content/example/all.xml",
    );

    let mut it = obj.content.into_iter();

    assert!(
        matches!(it.next().unwrap(), MixedChoiceListTypeContent::Text(x) if x.0 == "\n    Text before &\n    ")
    );
    assert!(matches!(
        it.next().unwrap(),
        MixedChoiceListTypeContent::Fuu(111)
    ));
    assert!(
        matches!(it.next().unwrap(), MixedChoiceListTypeContent::Text(x) if x.0 == "\n    Text between\n    ")
    );
    assert!(matches!(it.next().unwrap(), MixedChoiceListTypeContent::Bar(x) if x == "Hello World"));
    assert!(
        matches!(it.next().unwrap(), MixedChoiceListTypeContent::Text(x) if x.0 == "\n    Text after ä\n")
    );
    assert!(it.next().is_none());
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
