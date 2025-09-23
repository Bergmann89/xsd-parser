use xsd_parser::{
    config::{OptimizerFlags, RendererFlags},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .without_optimizer_flags(OptimizerFlags::all())
        .with_renderer_flags(RendererFlags::RENDER_DOCS)
        .with_generate([(IdentType::Element, "tns:SomeDetails")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/documentation/schema.xsd",
        "tests/feature/documentation/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
