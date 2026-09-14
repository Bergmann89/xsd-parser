use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[cfg(not(feature = "update-expectations"))]
use crate::utils::quick_xml_read_test_result;

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Root")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/unknown_element_in_sequence/schema.xsd",
        "tests/feature/unknown_element_in_sequence/expected/default.rs",
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
        "tests/feature/unknown_element_in_sequence/schema.xsd",
        "tests/feature/unknown_element_in_sequence/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* read */

macro_rules! read_ok_test {
    ($name:ident, $file:literal) => {
        #[test]
        #[cfg(not(feature = "update-expectations"))]
        fn $name() {
            use quick_xml::RootType;

            let obj = crate::utils::quick_xml_read_test::<RootType, _>(concat!(
                "tests/feature/unknown_element_in_sequence/example/",
                $file
            ));

            assert_eq!(obj.a, "before");
            assert_eq!(obj.b.as_deref(), Some("after"));
        }
    };
}

read_ok_test!(read_unknown_element_before, "unknown_before.xml");
read_ok_test!(read_unknown_element_between, "unknown_between.xml");
read_ok_test!(read_unknown_element_after, "unknown_after.xml");
read_ok_test!(read_unknown_element_nested, "unknown_nested.xml");
read_ok_test!(read_unknown_element_empty, "unknown_empty.xml");
read_ok_test!(
    read_unknown_element_with_children,
    "unknown_with_children.xml"
);

// The unknown element must not satisfy the cardinality checks of the content
// model: the required element `A` is missing and the document has to be
// rejected.
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_missing_required_element_rejected() {
    use quick_xml::RootType;

    let result = quick_xml_read_test_result::<RootType, _>(
        "tests/feature/unknown_element_in_sequence/example/missing_required.xml",
    );

    assert!(result.is_err());
}

// An element that is known to the content model but appears twice must still
// be rejected. It must not be silently skipped like an unknown element.
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_duplicate_known_element_rejected() {
    use quick_xml::RootType;

    let result = quick_xml_read_test_result::<RootType, _>(
        "tests/feature/unknown_element_in_sequence/example/duplicate_known.xml",
    );

    assert!(result.is_err());
}

// Unknown attributes are handled by `raise_unexpected_attrib` and must still
// be rejected.
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_unknown_attribute_rejected() {
    use quick_xml::RootType;

    let result = quick_xml_read_test_result::<RootType, _>(
        "tests/feature/unknown_element_in_sequence/example/unknown_attribute.xml",
    );

    assert!(result.is_err());
}
