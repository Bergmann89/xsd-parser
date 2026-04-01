/*
 * Tests how quick_xml deserialization handles enums that come from an
 * xs:choice, when that enum has only one variant, and that variant is
 * a group. Previously the logic for determining what could implement
 * Default in this scenario was incomplete, creating code that couldn't
 * compile due to a missing Default impl. In the provided schema, one
 * element has an xs:choice that is defaultable and the other has one
 * one that isn't
 */

use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([
        (IdentType::Element, "tns:foo"),
        (IdentType::Element, "tns:bar"),
    ])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/choice_single_option_defaults/schema.xsd",
        "tests/feature/choice_single_option_defaults/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]
    include!("expected/default.rs");
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/choice_single_option_defaults/schema.xsd",
        "tests/feature/choice_single_option_defaults/expected/quick_xml.rs",
        config().with_quick_xml_deserialize(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]
    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
#[test]
fn read_quick_xml_defaultable() {
    use quick_xml::Bar;
    use quick_xml::BarTypeContent;
    let obj = crate::utils::quick_xml_read_test::<Bar, _>(
        "tests/feature/choice_single_option_defaults/example/defaultable.xml",
    );
    let expected = Bar {
        content: BarTypeContent::Content4(vec![]),
    };
    assert!(matches!(obj, expected))
}

#[cfg(not(feature = "update-expectations"))]
#[test]
fn read_quick_xml_not_defaultable() {
    use quick_xml::Foo;
    use quick_xml::FooContent2Type;
    use quick_xml::FooTypeContent;
    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/choice_single_option_defaults/example/not_defaultable.xml",
    );
    let expected = Foo {
        content: FooTypeContent::Content2(vec![
            FooContent2Type {
                info: "test 1".into(),
            },
            FooContent2Type {
                info: "test 2".into(),
            },
        ]),
    };
    assert!(matches!(obj, expected))
}
