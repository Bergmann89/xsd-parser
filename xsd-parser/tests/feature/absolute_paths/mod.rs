use xsd_parser::{
    config::{GeneratorFlags, OptimizerFlags},
    Config,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .without_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(
            GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS | GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS,
        )
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/absolute_paths/schema.xsd",
        "tests/feature/absolute_paths/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/absolute_paths/schema.xsd",
        "tests/feature/absolute_paths/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
