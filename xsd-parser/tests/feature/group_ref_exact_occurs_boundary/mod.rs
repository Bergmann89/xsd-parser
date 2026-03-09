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
        "tests/feature/group_ref_exact_occurs_boundary/schema.xsd",
        "tests/feature/group_ref_exact_occurs_boundary/expected/default.rs",
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
        "tests/feature/group_ref_exact_occurs_boundary/schema.xsd",
        "tests/feature/group_ref_exact_occurs_boundary/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_exact_two() {
    use quick_xml::Collection;

    let obj = crate::utils::quick_xml_read_test::<Collection, _>(
        "tests/feature/group_ref_exact_occurs_boundary/example/exact_two.xml",
    );

    assert_eq!(obj.entry.len(), 2);
    assert_eq!(obj.entry[0].code, "A");
    assert_eq!(obj.entry[1].code, "B");
    assert_eq!(obj.name, "Exact Two");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_below_min_rejected() {
    use quick_xml::Collection;

    let result = crate::utils::quick_xml_read_test_result::<Collection, _>(
        "tests/feature/group_ref_exact_occurs_boundary/example/below_min.xml",
    );

    assert!(result.is_err());
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_above_max_rejected() {
    use quick_xml::Collection;

    let result = crate::utils::quick_xml_read_test_result::<Collection, _>(
        "tests/feature/group_ref_exact_occurs_boundary/example/above_max.xml",
    );

    assert!(result.is_err());
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
