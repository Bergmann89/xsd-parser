use xsd_parser::{models::LegacyNaming, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Root")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/naming/schema.xsd",
        "tests/feature/naming/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

#[test]
fn generate_legacy() {
    generate_test(
        "tests/feature/naming/schema.xsd",
        "tests/feature/naming/expected/legacy.rs",
        config().with_naming(LegacyNaming::default()),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod legacy {
    #![allow(unused_imports)]

    include!("expected/legacy.rs");
}
