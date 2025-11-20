use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::MIXED_TYPE_SUPPORT)
        .with_generate([(IdentType::Element, "Root")])
        .with_any_type_support()
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/any_type_nested/schema.xsd",
        "tests/feature/any_type_nested/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Root;

    let obj = crate::utils::quick_xml_read_test::<Root, _>(
        "tests/feature/any_type_nested/example/default.xml",
    );

    assert!(obj.container.content.len() > 0);
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
