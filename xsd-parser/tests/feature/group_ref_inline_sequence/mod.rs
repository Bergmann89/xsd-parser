use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([
            (IdentType::Element, "tns:Widget"),
            (IdentType::Element, "tns:Container"),
        ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/group_ref_inline_sequence/schema.xsd",
        "tests/feature/group_ref_inline_sequence/expected/default.rs",
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
        "tests/feature/group_ref_inline_sequence/schema.xsd",
        "tests/feature/group_ref_inline_sequence/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

// Control: single-occurrence group ref works fine
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_widget() {
    use quick_xml::Widget;

    let obj = crate::utils::quick_xml_read_test::<Widget, _>(
        "tests/feature/group_ref_inline_sequence/example/default.xml",
    );

    assert_eq!(obj.identity.identifier, "widget-001");
    assert_eq!(obj.identity.tag, vec!["red", "large"]);
    assert_eq!(obj.name, "My Widget");
    assert_eq!(obj.description.as_deref(), Some("A test widget"));
}

// Regression test: xs:group ref with maxOccurs="unbounded" followed by another element.
// When only a single group occurrence is present, the deserializer must transition
// from Identity to Name cleanly (fallback must not overwrite the state advance).
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_container_single() {
    use quick_xml::Container;

    let obj = crate::utils::quick_xml_read_test::<Container, _>(
        "tests/feature/group_ref_inline_sequence/example/unbounded_single.xml",
    );

    assert_eq!(obj.identity.len(), 1);
    assert_eq!(obj.identity[0].identifier, "item-001");
    assert!(obj.identity[0].tag.is_empty());
    assert_eq!(obj.name, "Single Identity");
}

// Multiple group occurrences followed by another element.
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_container_multiple() {
    use quick_xml::Container;

    let obj = crate::utils::quick_xml_read_test::<Container, _>(
        "tests/feature/group_ref_inline_sequence/example/unbounded_multiple.xml",
    );

    assert_eq!(obj.identity.len(), 3);
    assert_eq!(obj.identity[0].identifier, "item-001");
    assert_eq!(obj.identity[0].tag, vec!["alpha"]);
    assert_eq!(obj.identity[1].identifier, "item-002");
    assert!(obj.identity[1].tag.is_empty());
    assert_eq!(obj.identity[2].identifier, "item-003");
    assert_eq!(obj.identity[2].tag, vec!["beta", "gamma"]);
    assert_eq!(obj.name, "Multiple Identities");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_widget() {
    use quick_xml::{Widget, WidgetIdentityType};

    let obj = Widget {
        identity: WidgetIdentityType {
            identifier: "widget-001".into(),
            tag: vec!["red".into(), "large".into()],
        },
        name: "My Widget".into(),
        description: Some("A test widget".into()),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Widget",
        "tests/feature/group_ref_inline_sequence/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
