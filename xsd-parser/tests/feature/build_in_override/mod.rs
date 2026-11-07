use xsd_parser::{
    config::{Config, IdentTriple, NamespaceIdent},
    models::{
        meta::{BuildInMeta, MetaType},
        IdentType,
    },
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    let mut config = Config::test_default().with_generate([
        (
            IdentType::Element,
            Some(NamespaceIdent::namespace(b"urn:example:minimal")),
            "Value",
        ),
        (
            IdentType::Element,
            Some(NamespaceIdent::namespace(b"urn:example:minimal")),
            "Plain",
        ),
    ]);

    config.interpreter.types = vec![(
        IdentTriple::from((IdentType::Type, "xs:integer")),
        MetaType::from(BuildInMeta::I64),
    )];

    config
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/build_in_override/schema.xsd",
        "tests/feature/build_in_override/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/build_in_override/schema.xsd",
        "tests/feature/build_in_override/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    include!("expected/quick_xml.rs");
}
