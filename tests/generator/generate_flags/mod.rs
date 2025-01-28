use xsd_parser::{generator::GenerateFlags, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn empty() {
    generate_test(
        "tests/generator/generate_flags/schema.xsd",
        "tests/generator/generate_flags/expected/empty.rs",
        Config::test_default().with_generate([(IdentType::Type, "tns:MyChoice")]),
    );
}

#[test]
fn use_modules() {
    generate_test(
        "tests/generator/generate_flags/schema.xsd",
        "tests/generator/generate_flags/expected/use_modules.rs",
        Config::test_default()
            .with_generate_flags(GenerateFlags::USE_MODULES)
            .with_generate([(IdentType::Type, "tns:MyChoice")]),
    );
}

#[test]
fn flatten_content() {
    generate_test(
        "tests/generator/generate_flags/schema.xsd",
        "tests/generator/generate_flags/expected/flatten_content.rs",
        Config::test_default()
            .with_generate_flags(GenerateFlags::FLATTEN_CONTENT)
            .with_generate([(IdentType::Type, "tns:MyChoice")]),
    );
}
