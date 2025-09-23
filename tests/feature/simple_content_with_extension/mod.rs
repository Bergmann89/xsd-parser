use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([
        (IdentType::Element, "tns:SupplierId"),
        (IdentType::Element, "tns:UnitName"),
    ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/simple_content_with_extension/schema.xsd",
        "tests/feature/simple_content_with_extension/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
