use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Person")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/group_ref_static_list/schema.xsd",
        "tests/feature/group_ref_static_list/expected/default.rs",
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
        "tests/feature/group_ref_static_list/schema.xsd",
        "tests/feature/group_ref_static_list/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

// Single group occurrence followed by Name — exercises the StaticList handler_none path
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_single() {
    use quick_xml::Person;

    let obj = crate::utils::quick_xml_read_test::<Person, _>(
        "tests/feature/group_ref_static_list/example/single.xml",
    );

    assert_eq!(obj.address.len(), 1);
    assert_eq!(obj.address[0].street, "123 Main St");
    assert_eq!(obj.address[0].city, "Springfield");
    assert_eq!(obj.name, "John Doe");
}

// Multiple group occurrences followed by Name
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_multiple() {
    use quick_xml::Person;

    let obj = crate::utils::quick_xml_read_test::<Person, _>(
        "tests/feature/group_ref_static_list/example/multiple.xml",
    );

    assert_eq!(obj.address.len(), 2);
    assert_eq!(obj.address[0].street, "123 Main St");
    assert_eq!(obj.address[0].city, "Springfield");
    assert_eq!(obj.address[1].street, "456 Oak Ave");
    assert_eq!(obj.address[1].city, "Shelbyville");
    assert_eq!(obj.name, "Jane Doe");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
