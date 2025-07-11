use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::MIXED_TYPE_SUPPORT)
        .with_generate([
            (IdentType::Element, "tns:MixedAll"),
            (IdentType::Element, "tns:MixedChoice"),
            (IdentType::Element, "tns:MixedChoiceList"),
            (IdentType::Element, "tns:MixedSequence"),
        ])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/mixed_content/schema.xsd",
        "tests/feature/mixed_content/expected/default.rs",
        config(),
    );
}

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

    let obj = crate::utils::quick_xml_read_test::<MixedAll, _>(
        "tests/feature/mixed_content/example/all.xml",
    );

    assert_eq!(obj.text_before.as_deref(), Some("\n    Text before\n    "));
    assert_eq!(obj.fuu.value, 111);
    assert_eq!(
        obj.fuu.text_after.as_deref(),
        Some("\n    Text between\n    ")
    );
    assert_eq!(obj.bar.value, "Hello World");
    assert_eq!(obj.bar.text_after.as_deref(), Some("\n    Text after\n"));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_choice_single() {
    use quick_xml::{MixedChoice, MixedChoiceTypeContent};

    let obj = crate::utils::quick_xml_read_test::<MixedChoice, _>(
        "tests/feature/mixed_content/example/choice.xml",
    );

    assert_eq!(obj.text_before.as_deref(), Some("\n    Text before\n    "));

    let MixedChoiceTypeContent::Fuu(mixed) = &obj.content else {
        panic!(
            "Expected `MixedChoiceTypeContent::Fuu(_)` but got {:?}",
            obj.content
        );
    };

    assert_eq!(mixed.value, 111);
    assert_eq!(mixed.text_after.as_deref(), Some("\n    Text after\n"));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_choice_list() {
    use quick_xml::{MixedChoiceList, MixedChoiceListTypeContent};

    let obj = crate::utils::quick_xml_read_test::<MixedChoiceList, _>(
        "tests/feature/mixed_content/example/all.xml",
    );

    assert_eq!(obj.text_before.as_deref(), Some("\n    Text before\n    "));

    let mut it = obj.content.into_iter();

    let item = it.next().unwrap();
    let MixedChoiceListTypeContent::Fuu(mixed) = item else {
        panic!(
            "Expected `MixedChoiceTypeContent::Fuu(_)` but got {:?}",
            item
        );
    };
    assert_eq!(mixed.value, 111);
    assert_eq!(
        mixed.text_after.as_deref(),
        Some("\n    Text between\n    ")
    );

    let item = it.next().unwrap();
    let MixedChoiceListTypeContent::Bar(mixed) = item else {
        panic!(
            "Expected `MixedChoiceTypeContent::Bar(_)` but got {:?}",
            item
        );
    };
    assert_eq!(mixed.value, "Hello World");
    assert_eq!(mixed.text_after.as_deref(), Some("\n    Text after\n"));

    assert!(it.next().is_none());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_sequence() {
    use quick_xml::MixedSequence;

    let obj = crate::utils::quick_xml_read_test::<MixedSequence, _>(
        "tests/feature/mixed_content/example/sequence.xml",
    );

    assert_eq!(obj.text_before.as_deref(), Some("\n    Text before\n    "));
    assert_eq!(obj.fuu.value, 111);
    assert_eq!(
        obj.fuu.text_after.as_deref(),
        Some("\n    Text between\n    ")
    );
    assert_eq!(obj.bar.value, "Hello World");
    assert_eq!(obj.bar.text_after.as_deref(), Some("\n    Text after\n"));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_all() {
    use quick_xml::MixedAll;
    use xsd_parser::quick_xml::Mixed;

    let obj = MixedAll {
        text_before: Some("Text before".into()),
        fuu: Mixed {
            value: 111,
            text_after: Some("Text between".into()),
        },
        bar: Mixed {
            value: "Hello World".into(),
            text_after: Some("Text after".into()),
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
    use xsd_parser::quick_xml::Mixed;

    let obj = MixedChoice {
        text_before: Some("Text before".into()),
        content: MixedChoiceTypeContent::Fuu(Mixed {
            value: 111,
            text_after: Some("Text after".into()),
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
    use xsd_parser::quick_xml::Mixed;

    let obj = MixedChoiceList {
        text_before: Some("Text before".into()),
        content: vec![
            MixedChoiceListTypeContent::Fuu(Mixed {
                value: 111,
                text_after: Some("Text between".into()),
            }),
            MixedChoiceListTypeContent::Bar(Mixed {
                value: "Hello World".into(),
                text_after: Some("Text after".into()),
            }),
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
    use xsd_parser::quick_xml::Mixed;

    let obj = MixedSequence {
        text_before: Some("Text before".into()),
        fuu: Mixed {
            value: 111,
            text_after: Some("Text between".into()),
        },
        bar: Mixed {
            value: "Hello World".into(),
            text_after: Some("Text after".into()),
        },
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:MixedSequence",
        "tests/feature/mixed_content/example/sequence.xml",
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
