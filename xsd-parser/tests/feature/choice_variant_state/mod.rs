use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_dynamic_type((IdentType::Type, "tns:animal"))
        .with_generate([
            (IdentType::Element, "tns:GroupPick"),
            (IdentType::Element, "tns:ChoiceGroupPick"),
            (IdentType::Element, "tns:DynPick"),
            (IdentType::Element, "tns:MixedPick"),
        ])
        .with_quick_xml_serialize()
        .with_quick_xml_deserialize()
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/choice_variant_state/schema.xsd",
        "tests/feature/choice_variant_state/expected/quick_xml.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! read {
    ($ty:ty, $xml:expr) => {{
        use xsd_parser_types::quick_xml::{DeserializeSync, SliceReader};

        <$ty>::deserialize(&mut SliceReader::new($xml))
    }};
}

#[cfg(not(feature = "update-expectations"))]
mod read {
    use xsd_parser_types::quick_xml::ErrorKind;

    use super::quick_xml;

    #[test]
    fn choice_group_variant_rejects_unknown_element() {
        let err = read!(
            quick_xml::ChoiceGroupPick,
            "<tns:ChoiceGroupPick xmlns:tns=\"http://example.com\"><tns:U>u</tns:U><tns:Z/></tns:ChoiceGroupPick>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn group_variant_rejects_unknown_element() {
        let err = read!(
            quick_xml::GroupPick,
            "<tns:GroupPick xmlns:tns=\"http://example.com\"><tns:P>p</tns:P><tns:Q>q</tns:Q><tns:Z/></tns:GroupPick>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn dynamic_variant_rejects_unknown_element() {
        let err = read!(
            quick_xml::DynPick,
            "<tns:DynPick xmlns:tns=\"http://example.com\"><tns:animal id=\"1\"/><tns:Z/></tns:DynPick>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn mixed_choice_ignores_text_around_the_element() {
        let obj = read!(
            quick_xml::MixedPick,
            "<tns:MixedPick xmlns:tns=\"http://example.com\">t1<tns:A>a</tns:A>t2</tns:MixedPick>"
        )
        .unwrap();

        assert!(
            matches!(&obj, quick_xml::MixedPickType::A(x) if x == "a"),
            "{obj:?}"
        );
    }
}
