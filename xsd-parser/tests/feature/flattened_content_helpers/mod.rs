use xsd_parser::{pipeline::renderer::ContentHelpersRenderStep, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_render_step(ContentHelpersRenderStep)
        .with_generate([(IdentType::Element, "tns:Foo")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/flattened_content_helpers/schema.xsd",
        "tests/feature/flattened_content_helpers/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_keyword() {
    generate_test(
        "tests/feature/flattened_content_helpers/schema_keyword.xsd",
        "tests/feature/flattened_content_helpers/expected/keyword.rs",
        Config::test_default()
            .with_render_step(ContentHelpersRenderStep)
            .with_generate([(IdentType::Element, "tns:KeywordFoo")]),
    );
}
