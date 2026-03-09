use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Collection")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/group_ref_min_occurs_threshold/schema.xsd",
        "tests/feature/group_ref_min_occurs_threshold/expected/default.rs",
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
        "tests/feature/group_ref_min_occurs_threshold/schema.xsd",
        "tests/feature/group_ref_min_occurs_threshold/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

// Exact minOccurs=2 should parse and transition to Name correctly.
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_exact_min() {
    use quick_xml::Collection;

    let obj = crate::utils::quick_xml_read_test::<Collection, _>(
        "tests/feature/group_ref_min_occurs_threshold/example/exact_min.xml",
    );

    assert_eq!(obj.entry.len(), 2);
    assert_eq!(obj.entry[0].code, "A");
    assert_eq!(obj.entry[1].code, "B");
    assert_eq!(obj.name, "Exact Min");
}

// Above minOccurs should also parse.
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_above_min() {
    use quick_xml::Collection;

    let obj = crate::utils::quick_xml_read_test::<Collection, _>(
        "tests/feature/group_ref_min_occurs_threshold/example/above_min.xml",
    );

    assert_eq!(obj.entry.len(), 3);
    assert_eq!(obj.entry[0].code, "A");
    assert_eq!(obj.entry[1].code, "B");
    assert_eq!(obj.entry[2].code, "C");
    assert_eq!(obj.name, "Above Min");
}

// Below minOccurs must fail instead of accepting Name too early.
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_below_min_rejected() {
    use quick_xml::Collection;

    let result = crate::utils::quick_xml_read_test_result::<Collection, _>(
        "tests/feature/group_ref_min_occurs_threshold/example/below_min.xml",
    );

    assert!(result.is_err());
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
