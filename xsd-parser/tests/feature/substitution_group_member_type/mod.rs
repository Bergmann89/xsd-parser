//! Elements without a type that are member of a substitution group take the
//! type of the head of the group, `xs:anyType` is used if there is none.

use xsd_parser::{config::TypedefMode, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_typedef_mode(TypedefMode::Typedef)
        .with_generate([
            (IdentType::Element, "tns:Head"),
            (IdentType::Element, "tns:Member"),
            (IdentType::Element, "tns:Chain2"),
            (IdentType::Element, "tns:Chain1"),
            (IdentType::Element, "tns:ChainHead"),
            (IdentType::Element, "tns:UntypedHead"),
            (IdentType::Element, "tns:UntypedMember"),
        ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/substitution_group_member_type/schema.xsd",
        "tests/feature/substitution_group_member_type/expected/default.rs",
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
        "tests/feature/substitution_group_member_type/schema.xsd",
        "tests/feature/substitution_group_member_type/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_member() {
    use quick_xml::Member;

    let obj = crate::utils::quick_xml_read_test::<Member, _>(
        "tests/feature/substitution_group_member_type/example/member.xml",
    );

    assert_eq!(obj.name, "x");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_member() {
    use quick_xml::BaseType;

    let obj = BaseType { name: "x".into() };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Member",
        "tests/feature/substitution_group_member_type/example/member.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
