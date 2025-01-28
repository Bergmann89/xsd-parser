use xsd_parser::{generator::ContentMode, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn auto() {
    generate_test(
        "tests/generator/content_mode/schema.xsd",
        "tests/generator/content_mode/expected/auto.rs",
        Config::test_default()
            .with_content_mode(ContentMode::Auto)
            .with_generate([
                (IdentType::Type, "tns:MyAll"),
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[test]
fn enum_() {
    generate_test(
        "tests/generator/content_mode/schema.xsd",
        "tests/generator/content_mode/expected/enum.rs",
        Config::test_default()
            .with_content_mode(ContentMode::Enum)
            .with_generate([
                (IdentType::Type, "tns:MyAll"),
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[test]
fn struct_() {
    generate_test(
        "tests/generator/content_mode/schema.xsd",
        "tests/generator/content_mode/expected/struct.rs",
        Config::test_default()
            .with_content_mode(ContentMode::Struct)
            .with_generate([
                (IdentType::Type, "tns:MyAll"),
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}
