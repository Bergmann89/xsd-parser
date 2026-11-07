use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[cfg(not(feature = "update-expectations"))]
use xsd_parser_types::quick_xml::ErrorKind;

#[cfg(not(feature = "update-expectations"))]
use crate::utils::{quick_xml_read_test, quick_xml_read_test_result};

fn config(skip_unknown_elements: bool) -> Config {
    Config::test_default()
        .with_generate([
            (IdentType::Element, "tns:Reference"),
            (IdentType::Element, "tns:Signature"),
        ])
        .with_quick_xml_deserialize_config(false, skip_unknown_elements)
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/unknown_element_in_wildcard/schema.xsd",
        "tests/feature/unknown_element_in_wildcard/expected/quick_xml.rs",
        config(true),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    include!("expected/quick_xml.rs");
}

/* quick_xml (strict) */

#[test]
fn generate_quick_xml_strict() {
    generate_test(
        "tests/feature/unknown_element_in_wildcard/schema.xsd",
        "tests/feature/unknown_element_in_wildcard/expected/quick_xml_strict.rs",
        config(false),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_strict {
    include!("expected/quick_xml_strict.rs");
}

/* read */

macro_rules! read_wildcard_test {
    ($name:ident, $module:ident) => {
        #[test]
        #[cfg(not(feature = "update-expectations"))]
        fn $name() {
            let obj = quick_xml_read_test::<$module::Reference, _>(
                "tests/feature/unknown_element_in_wildcard/example/wildcard.xml",
            );

            let transforms = obj.transforms.unwrap();
            assert_eq!(transforms.transform.len(), 1);
            assert!(transforms.transform[0].content.is_empty());
            assert_eq!(obj.digest_method, "sha256");
            assert_eq!(obj.digest_value, "abc");
        }
    };
}

read_wildcard_test!(read_element_in_wildcard_skipped, quick_xml);
read_wildcard_test!(read_element_in_wildcard_skipped_strict, quick_xml_strict);

// `SignatureValue` inside `SignedInfo` must not be taken for the one of `Signature`.
#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_known_element_nested_in_child_rejected_strict() {
    let err = quick_xml_read_test_result::<quick_xml_strict::Signature, _>(
        "tests/feature/unknown_element_in_wildcard/example/nested_known.xml",
    )
    .unwrap_err();

    assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
}
