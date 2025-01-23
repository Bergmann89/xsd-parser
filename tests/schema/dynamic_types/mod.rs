use xsd_parser::{types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/dynamic_types/schema.xsd",
        "tests/schema/dynamic_types/expected/default.rs",
        Config::test_default()
            .with_derive([""])
            .with_generate([(IdentType::Element, "tns:base")]),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
