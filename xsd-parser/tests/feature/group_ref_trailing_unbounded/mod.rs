use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Catalog")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/group_ref_trailing_unbounded/schema.xsd",
        "tests/feature/group_ref_trailing_unbounded/expected/default.rs",
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
        "tests/feature/group_ref_trailing_unbounded/schema.xsd",
        "tests/feature/group_ref_trailing_unbounded/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

// Single trailing group occurrence
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_single_entry() {
    use quick_xml::Catalog;

    let obj = crate::utils::quick_xml_read_test::<Catalog, _>(
        "tests/feature/group_ref_trailing_unbounded/example/single_entry.xml",
    );

    assert_eq!(obj.title, "My Catalog");
    assert_eq!(obj.entry.len(), 1);
    assert_eq!(obj.entry[0].key, "color");
    assert_eq!(obj.entry[0].value.as_deref(), Some("red"));
}

// Multiple trailing group occurrences, some with optional Value omitted
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_multiple_entries() {
    use quick_xml::Catalog;

    let obj = crate::utils::quick_xml_read_test::<Catalog, _>(
        "tests/feature/group_ref_trailing_unbounded/example/multiple_entries.xml",
    );

    assert_eq!(obj.title, "My Catalog");
    assert_eq!(obj.entry.len(), 3);
    assert_eq!(obj.entry[0].key, "color");
    assert_eq!(obj.entry[0].value.as_deref(), Some("red"));
    assert_eq!(obj.entry[1].key, "size");
    assert_eq!(obj.entry[1].value, None);
    assert_eq!(obj.entry[2].key, "weight");
    assert_eq!(obj.entry[2].value.as_deref(), Some("10kg"));
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
