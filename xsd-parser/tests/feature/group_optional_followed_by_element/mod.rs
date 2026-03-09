use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Item")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/group_optional_followed_by_element/schema.xsd",
        "tests/feature/group_optional_followed_by_element/expected/default.rs",
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
        "tests/feature/group_optional_followed_by_element/schema.xsd",
        "tests/feature/group_optional_followed_by_element/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

// Group present with partial children — should parse fine
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_with_group() {
    use quick_xml::Item;

    let obj = crate::utils::quick_xml_read_test::<Item, _>(
        "tests/feature/group_optional_followed_by_element/example/with_group.xml",
    );

    let meta = obj
        .metadata
        .as_ref()
        .expect("metadata group should be present");
    assert_eq!(meta.label.as_deref(), Some("Test Label"));
    assert_eq!(meta.description, None);
    assert_eq!(obj.name, "Test Item");
}

// Group absent — only the required Name element is present.
// This exercises the (true, _, _) handler_none template.
// If the latent bug exists, this will fail with UnexpectedEvent(<Name>).
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_without_group() {
    use quick_xml::Item;

    let obj = crate::utils::quick_xml_read_test::<Item, _>(
        "tests/feature/group_optional_followed_by_element/example/without_group.xml",
    );

    assert!(obj.metadata.is_none(), "metadata group should be absent");
    assert_eq!(obj.name, "No Metadata");
}

// Group fully present with both children
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_full_group() {
    use quick_xml::Item;

    let obj = crate::utils::quick_xml_read_test::<Item, _>(
        "tests/feature/group_optional_followed_by_element/example/full_group.xml",
    );

    let meta = obj
        .metadata
        .as_ref()
        .expect("metadata group should be present");
    assert_eq!(meta.label.as_deref(), Some("Full Label"));
    assert_eq!(meta.description.as_deref(), Some("Full Description"));
    assert_eq!(obj.name, "Full Item");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
