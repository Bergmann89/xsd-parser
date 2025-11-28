use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([
        (IdentType::Type, "SimpleLiteral"),
        (IdentType::Type, "W3CDTF"),
    ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/simple_type_emptiable_complex_base/schema.xsd",
        "tests/feature/simple_type_emptiable_complex_base/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
