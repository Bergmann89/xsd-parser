use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "rootElement")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/inline_element_names/schema.xsd",
        "tests/feature/inline_element_names/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
