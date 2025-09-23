use crate::utils::{generate_test, ConfigEx};
use xsd_parser::config::NamespaceIdent;
use xsd_parser::{Config, IdentType};

fn config() -> Config {
    Config::test_default().with_generate([(
        IdentType::Element,
        Some(NamespaceIdent::namespace(b"http://example.com")),
        "Root",
    )])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/include_target_namespace/schema.xsd",
        "tests/feature/include_target_namespace/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
