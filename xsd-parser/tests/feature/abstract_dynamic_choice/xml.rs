use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::generate_test_no_input;

fn config(flatten: bool) -> Config {
    let mut config = super::config().with_generate([
        (IdentType::Type, "tns:Payload"),
        (IdentType::Type, "tns:Middle"),
        (IdentType::Type, "tns:Empty"),
        (IdentType::Element, "tns:Head"),
        (IdentType::Element, "tns:AbstractMember"),
        (IdentType::Element, "tns:EmptyHead"),
        (IdentType::Element, "tns:EmptyAbstractMember"),
        (IdentType::Element, "tns:Message"),
    ]);
    config.generator.flags = if flatten {
        GeneratorFlags::FLATTEN_CONTENT
    } else {
        GeneratorFlags::empty()
    };
    config
}

#[test]
fn generate_default() {
    generate_test_no_input(
        "tests/feature/abstract_dynamic_choice/expected/default.rs",
        config(false),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]
    #![deny(unreachable_code)]
    include!("expected/default.rs");
}

#[test]
fn generate_default_flatten() {
    generate_test_no_input(
        "tests/feature/abstract_dynamic_choice/expected/default_flatten.rs",
        config(true),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default_flatten {
    #![allow(unused_imports)]
    #![deny(unreachable_code)]
    include!("expected/default_flatten.rs");
}

#[test]
fn generate_quick_xml() {
    generate_test_no_input(
        "tests/feature/abstract_dynamic_choice/expected/quick_xml.rs",
        config(false).with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]
    #![deny(unreachable_code)]
    include!("expected/quick_xml.rs");
}

#[test]
fn generate_quick_xml_flatten() {
    generate_test_no_input(
        "tests/feature/abstract_dynamic_choice/expected/quick_xml_flatten.rs",
        config(true).with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_flatten {
    #![allow(unused_imports)]
    #![deny(unreachable_code)]
    include!("expected/quick_xml_flatten.rs");
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_xml {
    ($module:ident) => {{
        use xsd_parser_types::quick_xml::{DeserializeSync, SliceReader};

        let obj = crate::utils::quick_xml_read_test::<$module::Message, _>(
            "tests/feature/abstract_dynamic_choice/example/default.xml",
        );
        crate::utils::quick_xml_write_test(
            &obj,
            "tns:Message",
            "tests/feature/abstract_dynamic_choice/example/default.xml",
        );

        let payload = crate::utils::quick_xml_read_test::<$module::PayloadType, _>(
            "tests/feature/abstract_dynamic_choice/example/payload.xml",
        );
        crate::utils::quick_xml_write_test_default_root(
            &payload, "tests/feature/abstract_dynamic_choice/example/payload.xml",
        );
        let member = crate::utils::quick_xml_read_test::<$module::Head, _>(
            "tests/feature/abstract_dynamic_choice/example/member.xml",
        );
        crate::utils::quick_xml_write_test_default_root(
            &member, "tests/feature/abstract_dynamic_choice/example/member.xml",
        );

        for tag in ["Head", "AbstractMember"] {
            let xml = format!(
                r#"<tns:Message xmlns:tns="urn:example" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                    <tns:Payload xsi:type="tns:Leaf" id="7" label="concrete leaf"/>
                    <tns:{tag}>abstract member</tns:{tag}>
                </tns:Message>"#,
            );
            let mut reader = SliceReader::new(&xml);
            assert!($module::Message::deserialize(&mut reader).is_err(), "accepted abstract tag {tag}");
        }
        for type_ in ["Payload", "Middle"] {
            let xml = format!(
                r#"<tns:Message xmlns:tns="urn:example" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                    <tns:Payload xsi:type="tns:{type_}" id="7"/>
                    <tns:Member>concrete member</tns:Member>
                </tns:Message>"#,
            );
            let mut reader = SliceReader::new(&xml);
            assert!($module::Message::deserialize(&mut reader).is_err(), "accepted abstract type {type_}");
        }
        for tag in ["EmptyHead", "EmptyAbstractMember"] {
            let xml = format!(r#"<tns:{tag} xmlns:tns="urn:example"/>"#);
            let mut reader = SliceReader::new(&xml);
            assert!($module::EmptyHead::deserialize(&mut reader).is_err(), "accepted uninhabited tag {tag}");
        }
        let mut reader = SliceReader::new(r#"<tns:Empty xmlns:tns="urn:example"/>"#);
        assert!($module::EmptyType::deserialize(&mut reader).is_err());
    }};
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn round_trip_concrete_members_and_reject_abstract_tags() {
    check_xml!(quick_xml);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn round_trip_concrete_members_and_reject_abstract_tags_flatten() {
    check_xml!(quick_xml_flatten);
}
