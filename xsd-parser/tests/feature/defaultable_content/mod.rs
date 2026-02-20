use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags},
    Config, IdentType,
};

use crate::utils::generate_test;

fn config() -> Config {
    Config::default()
        .with_parser_flags(ParserFlags::all())
        .with_interpreter_flags(InterpreterFlags::all())
        .without_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(
            GeneratorFlags::all()
                - GeneratorFlags::ADVANCED_ENUMS
                - GeneratorFlags::FLATTEN_CONTENT
                - GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS
                - GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS,
        )
        .with_generate([
            (IdentType::Element, "Simple"),
            (IdentType::Element, "Sequence"),
            (IdentType::Element, "NestedSeq"),
        ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/defaultable_content/schema.xsd",
        "tests/feature/defaultable_content/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/defaultable_content/schema.xsd",
        "tests/feature/defaultable_content/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{NestedSeq, Sequence};

    crate::utils::quick_xml_read_test::<Sequence, _>(
        "tests/feature/defaultable_content/example/sequence.xml",
    );

    crate::utils::quick_xml_read_test::<NestedSeq, _>(
        "tests/feature/defaultable_content/example/nested.xml",
    );

    crate::utils::quick_xml_read_test::<NestedSeq, _>(
        "tests/feature/defaultable_content/example/nested2.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
