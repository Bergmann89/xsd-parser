use xsd_parser::{
    config::{GeneratorFlags, OptimizerFlags},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/documentation/schema.xsd",
        "tests/feature/documentation/expected/default.rs",
        Config::test_default()
            .without_optimizer_flags(OptimizerFlags::all())
            .with_generator_flags(GeneratorFlags::RENDER_DOCS)
            .with_generate([(IdentType::Element, "tns:SomeDetails")]),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
