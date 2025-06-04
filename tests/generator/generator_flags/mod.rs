use xsd_parser::{generator::GeneratorFlags, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn empty() {
    generate_test(
        "tests/generator/generator_flags/schema.xsd",
        "tests/generator/generator_flags/expected/empty.rs",
        Config::test_default()
            .set_generator_flags(GeneratorFlags::NONE)
            .with_generate([
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[test]
fn use_modules() {
    generate_test(
        "tests/generator/generator_flags/schema.xsd",
        "tests/generator/generator_flags/expected/use_modules.rs",
        Config::test_default()
            .set_generator_flags(GeneratorFlags::USE_MODULES)
            .with_generate([
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[test]
fn flatten_content() {
    generate_test(
        "tests/generator/generator_flags/schema.xsd",
        "tests/generator/generator_flags/expected/flatten_content.rs",
        Config::test_default()
            .set_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
            .with_generate([
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[test]
fn render_docs() {
    generate_test(
        "tests/generator/generator_flags/schema_with_docs.xsd",
        "tests/generator/generator_flags/expected/render_docs.rs",
        Config::test_default()
            .set_generator_flags(GeneratorFlags::RENDER_DOCS)
            .with_generate([
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}
