use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Foo")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/attribute/schema.xsd",
        "tests/feature/attribute/expected/default.rs",
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
        "tests/feature/attribute/schema.xsd",
        "tests/feature/attribute/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_attribute_needs_unescape() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/attribute/example/needs_unescape.xml",
    );

    assert_eq!(obj.bar, "Attribute >Value".to_string());
}

/* serde_xml_rs */
