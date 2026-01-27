use xsd_parser::config::{NamespaceIdent, RenderStep};
use xsd_parser::{Config, IdentType};
use xsd_parser_types::misc::Namespace;

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::Namespace(Namespace::new_const(
                b"http://example.com/test/01p00",
            ))),
            "Message",
        )])
        .with_render_steps([RenderStep::Types, RenderStep::WithNamespaceTrait])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/with_namespace_trait/schema.xsd",
        "tests/feature/with_namespace_trait/expected/default.rs",
        config(),
    );
}

/// Verify the generated code compiles without conflicting trait implementations.
/// This test catches the bug where WithNamespaceTrait generates duplicate impls
/// for both a type alias and its underlying struct.
#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
