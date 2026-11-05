//! Names of merged content types.
//!
//! `REMOVE_DUPLICATES` merges identical content types. The merged type must get a
//! name that is derived from the schema and must be defined in the module of the
//! schema it comes from, not in the module of the first owner.

use xsd_parser::{
    config::{GeneratorFlags, NamespaceIdent, OptimizerFlags},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::USE_MODULES)
        .with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::namespace(b"Foo")),
            "Root",
        )])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/content_type_names/schema.xsd",
        "tests/feature/content_type_names/expected/default.rs",
        config(),
    );
}

/* types only */

// Without `REMOVE_DUPLICATE_CONTENTS` each complex type keeps its own content
// type, like in version 1.5.
#[test]
fn generate_types_only() {
    generate_test(
        "tests/feature/content_type_names/schema.xsd",
        "tests/feature/content_type_names/expected/types_only.rs",
        config()
            .set_optimizer_flags(OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATE_CONTENTS),
    );
}
