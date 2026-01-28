use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:OpenDRIVE")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/nested_group_reuse/schema.xsd",
        "tests/feature/nested_group_reuse/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
