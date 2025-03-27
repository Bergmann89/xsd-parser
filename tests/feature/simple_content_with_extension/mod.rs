use xsd_parser::{types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/simple_content_with_extension/schema.xsd",
        "tests/feature/simple_content_with_extension/expected/default.rs",
        Config::test_default().with_generate([
            (IdentType::Element, "tns:SupplierId"),
            (IdentType::Element, "tns:UnitName"),
        ]),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
