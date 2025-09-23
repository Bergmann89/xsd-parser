use xsd_parser::{config::RendererFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_renderer_flags(RendererFlags::RENDER_DOCS)
        .with_generate([(IdentType::Element, "tns:Foo")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/enumeration_with_annotation/schema.xsd",
        "tests/feature/enumeration_with_annotation/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
