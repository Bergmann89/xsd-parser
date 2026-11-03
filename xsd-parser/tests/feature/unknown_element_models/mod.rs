use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_any_type_support()
        .with_dynamic_type((IdentType::Type, "tns:animal"))
        .with_generate([
            (IdentType::Element, "tns:Holder"),
            (IdentType::Element, "tns:TextHolder"),
            (IdentType::Element, "tns:Mixed"),
            (IdentType::Element, "tns:AnyRoot"),
            (IdentType::Element, "tns:Zoo"),
            (IdentType::Element, "tns:Many"),
            (IdentType::Element, "tns:GroupPick"),
            (IdentType::Element, "tns:DynPick"),
            (IdentType::Element, "tns:ChoiceGroupPick"),
            (IdentType::Element, "tns:SubstPick"),
        ])
        .with_quick_xml_serialize()
        .with_quick_xml_deserialize_config(false, true)
}

fn content_config() -> Config {
    Config::test_default()
        .with_generate([
            (IdentType::Element, "tns:Seq"),
            (IdentType::Element, "tns:Many"),
            (IdentType::Element, "tns:GroupPick"),
        ])
        .with_quick_xml_serialize()
        .with_quick_xml_deserialize_config(false, true)
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/unknown_element_models/schema.xsd",
        "tests/feature/unknown_element_models/expected/quick_xml.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* quick_xml (content mode) */

#[test]
fn generate_quick_xml_content() {
    generate_test(
        "tests/feature/unknown_element_models/schema.xsd",
        "tests/feature/unknown_element_models/expected/quick_xml_content.rs",
        content_config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_content {
    #![allow(unused_imports)]

    include!("expected/quick_xml_content.rs");
}

/* read */

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

    use super::{quick_xml, quick_xml_content};

    #[test]
    fn simple_typed_element_rejects_unknown_element() {
        let err = read!(
            quick_xml::Holder,
            "<tns:Holder xmlns:tns=\"http://example.com\"><tns:Simple>x<tns:Z/>y</tns:Simple></tns:Holder>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn simple_typed_element_rejects_element_known_to_parent() {
        let err = read!(
            quick_xml::Holder,
            "<tns:Holder xmlns:tns=\"http://example.com\"><tns:Simple>x<tns:Tail>t</tns:Tail></tns:Simple></tns:Holder>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn simple_content_rejects_unknown_element() {
        let err = read!(
            quick_xml::TextHolder,
            "<tns:TextHolder xmlns:tns=\"http://example.com\"><tns:Text id=\"1\">x<tns:Z/>y</tns:Text></tns:TextHolder>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn simple_content_rejects_element_known_to_parent() {
        let err = read!(
            quick_xml::TextHolder,
            "<tns:TextHolder xmlns:tns=\"http://example.com\"><tns:Text id=\"1\">x<tns:Tail>t</tns:Tail></tns:Text></tns:TextHolder>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn mixed_content_skips_unknown_elements_between_text() {
        let obj = read!(
            quick_xml::Mixed,
            "<tns:Mixed xmlns:tns=\"http://example.com\">t1<tns:Z>junk</tns:Z>t2<tns:A>a</tns:A>t3<tns:Z/><tns:B>b</tns:B></tns:Mixed>"
        )
        .unwrap();

        assert_eq!(obj.a, "a");
        assert_eq!(obj.b.as_deref(), Some("b"));
    }

    #[test]
    fn mixed_content_rejects_duplicate_known_element() {
        let err = read!(
            quick_xml::Mixed,
            "<tns:Mixed xmlns:tns=\"http://example.com\"><tns:A>a</tns:A>t<tns:Z/><tns:A>b</tns:A></tns:Mixed>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn child_of_any_sibling_skips_unknown_element() {
        let obj = read!(
            quick_xml::AnyRoot,
            "<tns:AnyRoot xmlns:tns=\"http://example.com\" xmlns:o=\"urn:other\"> \
                 <tns:Child><tns:Inner>i</tns:Inner><tns:Z/></tns:Child> \
                 <o:Other/> \
             </tns:AnyRoot>"
        )
        .unwrap();

        assert_eq!(obj.child.inner, "i");
        assert_eq!(obj.any.len(), 1);
    }

    #[test]
    fn dynamic_type_selected_by_xsi_type_skips_unknown_element() {
        let obj = read!(
            quick_xml::Zoo,
            "<tns:Zoo xmlns:tns=\"http://example.com\" xmlns=\"http://example.com\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"> \
                 <tns:Z><tns:animal id=\"9\"/></tns:Z> \
                 <tns:Name>zoo</tns:Name> \
                 <tns:animal id=\"1\"/> \
                 <tns:animal xsi:type=\"dog\" id=\"2\" name=\"Rex\"/> \
             </tns:Zoo>"
        )
        .unwrap();

        assert_eq!(obj.name, "zoo");
        assert_eq!(obj.animal.len(), 2);
        let first = obj.animal[0]
            .0
            .as_any()
            .downcast_ref::<quick_xml::AnimalDyn>();
        assert_eq!(first.map(|x| x.id), Some(1));
        let second = obj.animal[1]
            .0
            .as_any()
            .downcast_ref::<quick_xml::DogType>();
        assert_eq!(second.map(|x| x.name.as_str()), Some("Rex"));
    }

    #[test]
    fn choice_list_skips_unknown_elements_between_items() {
        let obj = read!(
            quick_xml::Many,
            "<tns:Many xmlns:tns=\"http://example.com\"><tns:Z/><tns:X>1</tns:X><tns:Z><tns:X/></tns:Z><tns:Y>2</tns:Y><tns:Z/></tns:Many>"
        )
        .unwrap();

        assert!(matches!(&obj.content[..], [
            quick_xml::ManyTypeContent::X(x),
            quick_xml::ManyTypeContent::Y(y),
        ] if x == "1" && y == "2"));
    }

    #[test]
    fn choice_group_variant_skips_unknown_element() {
        let obj = read!(
            quick_xml::GroupPick,
            "<tns:GroupPick xmlns:tns=\"http://example.com\"><tns:Z/><tns:P>p</tns:P><tns:Z/><tns:Q>q</tns:Q><tns:Z/></tns:GroupPick>"
        )
        .unwrap();

        assert!(matches!(&obj, quick_xml::GroupPickType::Pq(x) if x.p == "p" && x.q == "q"));
    }

    #[test]
    fn choice_group_variant_rejects_other_variant_after_unknown_element() {
        let err = read!(
            quick_xml::GroupPick,
            "<tns:GroupPick xmlns:tns=\"http://example.com\"><tns:P>p</tns:P><tns:Q>q</tns:Q><tns:Z/><tns:Y>y</tns:Y></tns:GroupPick>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn choice_dynamic_variant_skips_unknown_element() {
        let obj = read!(
            quick_xml::DynPick,
            "<tns:DynPick xmlns:tns=\"http://example.com\"><tns:animal id=\"1\"/><tns:Z/></tns:DynPick>"
        )
        .unwrap();

        let quick_xml::DynPickType::Animal(animal) = obj else {
            panic!("animal variant expected");
        };
        let animal = animal.0.as_any().downcast_ref::<quick_xml::AnimalDyn>();

        assert_eq!(animal.map(|x| x.id), Some(1));
    }

    #[test]
    fn choice_substitution_variant_skips_unknown_element() {
        let obj = read!(
            quick_xml::SubstPick,
            "<tns:SubstPick xmlns:tns=\"http://example.com\"><tns:Circle id=\"1\"/><tns:Z/></tns:SubstPick>"
        )
        .unwrap();

        let quick_xml::SubstPickType::Shape(shape) = obj else {
            panic!("shape variant expected");
        };
        let shape = shape.0.as_any().downcast_ref::<quick_xml::ShapeType>();

        assert_eq!(shape.map(|x| x.id), Some(Some(1)));
    }

    #[test]
    fn choice_choice_group_variant_skips_unknown_element() {
        let obj = read!(
            quick_xml::ChoiceGroupPick,
            "<tns:ChoiceGroupPick xmlns:tns=\"http://example.com\"><tns:U>u</tns:U><tns:Z/></tns:ChoiceGroupPick>"
        )
        .unwrap();

        assert!(matches!(
            obj,
            quick_xml::ChoiceGroupPickType::Uv(quick_xml::ChoiceGroupPickUvType::U(x)) if x == "u"
        ));
    }

    #[test]
    fn choice_choice_group_variant_rejects_other_variant_after_unknown_element() {
        let err = read!(
            quick_xml::ChoiceGroupPick,
            "<tns:ChoiceGroupPick xmlns:tns=\"http://example.com\"><tns:U>u</tns:U><tns:Z/><tns:Y>y</tns:Y></tns:ChoiceGroupPick>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn content_mode_skips_unknown_elements_at_every_level() {
        let obj = read!(
            quick_xml_content::Seq,
            "<tns:Seq xmlns:tns=\"http://example.com\"> \
                 <tns:Z/><tns:A>a</tns:A><tns:Z><tns:A/></tns:Z> \
                 <tns:Child><tns:Inner>i</tns:Inner><tns:Z/></tns:Child> \
                 <tns:Z/><tns:B>b</tns:B><tns:Z/> \
             </tns:Seq>"
        )
        .unwrap();

        assert_eq!(obj.a, "a");
        assert_eq!(obj.child.inner, "i");
        assert_eq!(obj.b.as_deref(), Some("b"));
    }

    #[test]
    fn content_mode_rejects_duplicate_known_element_after_unknown_element() {
        let err = read!(
            quick_xml_content::Seq,
            "<tns:Seq xmlns:tns=\"http://example.com\"><tns:A>a</tns:A><tns:Z/><tns:A>b</tns:A></tns:Seq>"
        )
        .unwrap_err();

        assert!(matches!(err.kind, ErrorKind::UnexpectedEvent(_)), "{err:?}");
    }

    #[test]
    fn content_mode_choice_group_variant_skips_unknown_element() {
        let obj = read!(
            quick_xml_content::GroupPick,
            "<tns:GroupPick xmlns:tns=\"http://example.com\"><tns:Z/><tns:P>p</tns:P><tns:Z/><tns:Q>q</tns:Q><tns:Z/></tns:GroupPick>"
        )
        .unwrap();

        assert!(
            matches!(&obj.content, quick_xml_content::GroupPickTypeContent::Pq(x) if x.p == "p" && x.q == "q")
        );
    }

    #[test]
    fn content_mode_choice_list_skips_unknown_elements() {
        let obj = read!(
            quick_xml_content::Many,
            "<tns:Many xmlns:tns=\"http://example.com\"><tns:X>1</tns:X><tns:Z/><tns:Y>2</tns:Y></tns:Many>"
        )
        .unwrap();

        assert!(matches!(&obj.content[..], [
            quick_xml_content::ManyTypeContent::X(x),
            quick_xml_content::ManyTypeContent::Y(y),
        ] if x == "1" && y == "2"));
    }
}
