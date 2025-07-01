use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Foo")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/globally_allowed_attribute/schema.xsd",
        "tests/feature/globally_allowed_attribute/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/globally_allowed_attribute/schema.xsd",
        "tests/feature/globally_allowed_attribute/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{Foo, FooTypeContent};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/globally_allowed_attribute/example/default.xml",
    );

    assert!(matches!(obj.content, FooTypeContent::Once(222)));
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
