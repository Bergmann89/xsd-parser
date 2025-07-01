use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Foo")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/choice_flatten_content/schema.xsd",
        "tests/feature/choice_flatten_content/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/choice_flatten_content/schema.xsd",
        "tests/feature/choice_flatten_content/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/choice_flatten_content/example/default.xml",
    );

    assert!(matches!(obj, Foo::Once(222)));
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
