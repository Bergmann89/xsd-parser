use crate::utils::{generate_test, ConfigEx};
use xsd_parser::{models::ExplicitNaming, Config};

fn config() -> Config {
    Config::test_default().with_naming(ExplicitNaming::new().unify_names(false))
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
    #![allow(non_camel_case_types)]

    include!("expected/default.rs");
}
