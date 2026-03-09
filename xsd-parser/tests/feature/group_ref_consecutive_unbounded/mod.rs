use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Record")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/group_ref_consecutive_unbounded/schema.xsd",
        "tests/feature/group_ref_consecutive_unbounded/expected/default.rs",
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
        "tests/feature/group_ref_consecutive_unbounded/schema.xsd",
        "tests/feature/group_ref_consecutive_unbounded/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

// Single occurrence of each group followed by Name
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_single_each() {
    use quick_xml::Record;

    let obj = crate::utils::quick_xml_read_test::<Record, _>(
        "tests/feature/group_ref_consecutive_unbounded/example/single_each.xml",
    );

    assert_eq!(obj.contact.len(), 1);
    assert_eq!(obj.contact[0].email, "a@example.com");
    assert_eq!(obj.address.len(), 1);
    assert_eq!(obj.address[0].street, "123 Main St");
    assert_eq!(obj.address[0].city, "Springfield");
    assert_eq!(obj.name, "Test Record");
}

// Multiple occurrences of each group followed by Name
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_multiple_each() {
    use quick_xml::Record;

    let obj = crate::utils::quick_xml_read_test::<Record, _>(
        "tests/feature/group_ref_consecutive_unbounded/example/multiple_each.xml",
    );

    assert_eq!(obj.contact.len(), 2);
    assert_eq!(obj.contact[0].email, "a@example.com");
    assert_eq!(obj.contact[1].email, "b@example.com");
    assert_eq!(obj.address.len(), 2);
    assert_eq!(obj.address[0].street, "123 Main St");
    assert_eq!(obj.address[0].city, "Springfield");
    assert_eq!(obj.address[1].street, "456 Oak Ave");
    assert_eq!(obj.address[1].city, "Shelbyville");
    assert_eq!(obj.name, "Multi Record");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
