use xsd_parser::{config::BoxFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn auto() {
    generate_test(
        "tests/generator/box_flags/schema.xsd",
        "tests/generator/box_flags/expected/auto.rs",
        Config::test_default()
            .with_box_flags(BoxFlags::AUTO)
            .with_generate([(IdentType::Type, "tns:MyChoice")]),
    );
}

#[test]
fn enum_elements() {
    generate_test(
        "tests/generator/box_flags/schema.xsd",
        "tests/generator/box_flags/expected/enum_elements.rs",
        Config::test_default()
            .with_box_flags(BoxFlags::ENUM_ELEMENTS)
            .with_generate([(IdentType::Type, "tns:MyChoice")]),
    );
}

#[test]
fn struct_elements() {
    generate_test(
        "tests/generator/box_flags/schema.xsd",
        "tests/generator/box_flags/expected/struct_elements.rs",
        Config::test_default()
            .with_box_flags(BoxFlags::STRUCT_ELEMENTS)
            .with_generate([(IdentType::Type, "tns:MyChoice")]),
    );
}
