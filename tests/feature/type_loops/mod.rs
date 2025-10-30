use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Type, "tns:Entity")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/type_loops/schema.xsd",
        "tests/feature/type_loops/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
