use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .set_generator_flags(GeneratorFlags::NONE)
        .with_generate([
            (IdentType::Type, "tns:MyChoice"),
            (IdentType::Type, "tns:MySequence"),
        ])
}

#[test]
fn empty() {
    generate_test(
        "tests/generator/generator_flags/schema.xsd",
        "tests/generator/generator_flags/expected/empty.rs",
        config(),
    );
}

#[test]
fn use_modules() {
    generate_test(
        "tests/generator/generator_flags/schema.xsd",
        "tests/generator/generator_flags/expected/use_modules.rs",
        config().set_generator_flags(GeneratorFlags::USE_MODULES),
    );
}

#[test]
fn flatten_content() {
    generate_test(
        "tests/generator/generator_flags/schema.xsd",
        "tests/generator/generator_flags/expected/flatten_content.rs",
        config().set_generator_flags(GeneratorFlags::FLATTEN_CONTENT),
    );
}

#[test]
fn mixed_type_support() {
    generate_test(
        "tests/generator/generator_flags/schema.xsd",
        "tests/generator/generator_flags/expected/mixed_type_support.rs",
        config().set_generator_flags(GeneratorFlags::MIXED_TYPE_SUPPORT),
    );
}

#[test]
fn nillable_type_support() {
    generate_test(
        "tests/generator/generator_flags/schema.xsd",
        "tests/generator/generator_flags/expected/nillable_type_support.rs",
        config().set_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT),
    );
}

#[test]
fn build_in_absolute_paths() {
    generate_test(
        "tests/generator/generator_flags/schema.xsd",
        "tests/generator/generator_flags/expected/build_in_absolute_paths.rs",
        config().set_generator_flags(GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS),
    );
}
