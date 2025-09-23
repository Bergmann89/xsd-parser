use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Type, None, "myComplexType")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/include_references/schema.xsd",
        "tests/feature/include_references/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
