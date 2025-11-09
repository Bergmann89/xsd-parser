use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::MIXED_TYPE_SUPPORT)
        .with_generate([(IdentType::Element, "Foo")])
        .with_any_support(
            "xsd_parser::xml::AnyElement",
            "xsd_parser::xml::AnyAttributes",
        )
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/any_type/schema.xsd",
        "tests/feature/any_type/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_write_quick_xml() {
    use quick_xml::Foo;

    let obj =
        crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/any_type/example/default.xml");
    crate::utils::quick_xml_write_test(&obj, "Foo", "tests/feature/any_type/example/default.xml");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
