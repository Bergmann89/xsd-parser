//! Regression case for content types that are shared between complex types.
//!
//! Two complex types reference the same `xs:group`, so after the optimizer merged the
//! duplicates both point at one content type. The generator has to emit that single
//! shared content type instead of a per-parent `*TypeContent` copy, which is what
//! `assert_shared_content!` checks below. Whether the shared type renders as a bare
//! enum or as a struct wrapping its own content enum depends on
//! `GeneratorFlags::FLATTEN_CONTENT`; only the sharing is the criterion.
//!
//! The `include!` modules below are the boxing guard: a missing `Box` makes them fail
//! to compile, not to diff.

use xsd_parser::{
    config::{Generate, GeneratorFlags, OptimizerFlags},
    Config,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
}

/// Both parents must reference the very same content type. This stops compiling as
/// soon as a per-parent copy of the content type comes back.
#[cfg(not(feature = "update-expectations"))]
macro_rules! assert_shared_content {
    () => {
        fn _shared_content(a: ElementAType) -> ElementBType {
            ElementBType {
                attr_2: None,
                content: a.content,
            }
        }
    };
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/shared_group_content/schema.xsd",
        "tests/feature/shared_group_content/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");

    assert_shared_content!();
}

/* single */

// The group is referenced with `1..1`, so the parents store the shared type directly
// instead of in a `Vec`. The variants of the shared type must be boxed or the type is
// infinitely sized - which the `include!` below compiles.
#[test]
fn generate_single() {
    generate_test(
        "tests/feature/shared_group_content/schema_single.xsd",
        "tests/feature/shared_group_content/expected/single.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod single {
    #![allow(unused_imports)]

    include!("expected/single.rs");

    assert_shared_content!();
}

/* all */

// `schema_recursive.xsd` loops back into its own owners, and the second owner is
// generated from an anonymous `xs:complexType`, so it sorts behind the shared content
// type. With `Generate::All` the shared type is therefore generated before that owner,
// which is the order in which the loop detection used to miss the cycle. All variants
// of the shared type must be boxed or the type is infinitely sized - which the
// `include!` below compiles.
#[test]
fn generate_all() {
    let mut config = config();
    config.generator.generate = Generate::All;

    generate_test(
        "tests/feature/shared_group_content/schema_recursive.xsd",
        "tests/feature/shared_group_content/expected/all.rs",
        config,
    );
}

#[cfg(not(feature = "update-expectations"))]
mod all {
    #![allow(unused_imports)]

    include!("expected/all.rs");

    assert_shared_content!();
}

/* mixed */

// The owners are `mixed="true"`, so `SIMPLIFY_MIXED_TYPES` rewrites the text content
// into a `Text` element and clears the mixed flag on the owners and on the shared
// group. The group must be shared afterwards, not copied per owner.
#[test]
fn generate_mixed() {
    generate_test(
        "tests/feature/shared_group_content/schema_mixed.xsd",
        "tests/feature/shared_group_content/expected/mixed.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod mixed {
    #![allow(unused_imports)]

    include!("expected/mixed.rs");

    assert_shared_content!();
}

/* two_groups */

// Two shared content types form a loop over `1..1` group references:
// `GroupA` -> `NestedBeta` -> `GroupB` -> `NestedAlpha` -> `GroupA`. The `include!`
// below only compiles if the lookahead walks the elements of a shared content type.
#[test]
fn generate_two_groups() {
    let mut config = config();
    config.generator.generate = Generate::All;

    generate_test(
        "tests/feature/shared_group_content/schema_two_groups.xsd",
        "tests/feature/shared_group_content/expected/two_groups.rs",
        config,
    );
}

#[cfg(not(feature = "update-expectations"))]
mod two_groups {
    #![allow(unused_imports)]

    include!("expected/two_groups.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/shared_group_content/schema.xsd",
        "tests/feature/shared_group_content/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{ElementAChoiceGroupType, ElementAType};

    let obj = crate::utils::quick_xml_read_test::<ElementAType, _>(
        "tests/feature/shared_group_content/example/default.xml",
    );

    assert_eq!(obj.attr_1.as_deref(), Some("a"));
    assert_eq!(obj.content.len(), 2);
    assert!(matches!(
        &obj.content[0],
        ElementAChoiceGroupType::ElementA(x) if x.attr_1.as_deref() == Some("inner-a"),
    ));
    assert!(matches!(
        &obj.content[1],
        ElementAChoiceGroupType::ElementB(x) if x.attr_2.as_deref() == Some("inner-b"),
    ));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{ElementAChoiceGroupType, ElementAType, ElementBType};

    let obj = ElementAType {
        attr_1: Some("a".into()),
        content: vec![
            ElementAChoiceGroupType::ElementA(ElementAType {
                attr_1: Some("inner-a".into()),
                content: Vec::new(),
            }),
            ElementAChoiceGroupType::ElementB(ElementBType {
                attr_2: Some("inner-b".into()),
                content: Vec::new(),
            }),
        ],
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Root",
        "tests/feature/shared_group_content/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");

    assert_shared_content!();
}
