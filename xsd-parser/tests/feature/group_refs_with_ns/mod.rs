use xsd_parser::{
    config::{GeneratorFlags, NamespaceIdent},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::namespace(b"Foo")),
            "Outer",
        )])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/group_refs_with_ns/schema.xsd",
        "tests/feature/group_refs_with_ns/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
