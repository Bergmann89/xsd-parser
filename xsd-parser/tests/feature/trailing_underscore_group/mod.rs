use crate::utils::{generate_test, ConfigEx};
use xsd_parser::Config;

fn config() -> Config {
    Config::test_default()
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/trailing_underscore_group/schema.xsd",
        "tests/feature/trailing_underscore_group/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports, dead_code, clippy::all)]
    include!("expected/default.rs");
}
