use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::USE_MODULES)
        .with_generate([(IdentType::Type, "Outer")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/group_modules/schema.xsd",
        "tests/feature/group_modules/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
