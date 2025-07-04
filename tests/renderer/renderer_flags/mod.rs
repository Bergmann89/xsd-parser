use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn empty() {
    generate_test(
        "tests/renderer/renderer_flags/schema.xsd",
        "tests/renderer/renderer_flags/expected/empty.rs",
        Config::test_default()
            .set_generator_flags(GeneratorFlags::NONE)
            .with_generate([
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[test]
fn render_docs() {
    generate_test(
        "tests/renderer/renderer_flags/schema_with_docs.xsd",
        "tests/renderer/renderer_flags/expected/render_docs.rs",
        Config::test_default()
            .set_generator_flags(GeneratorFlags::RENDER_DOCS)
            .with_generate([
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}
