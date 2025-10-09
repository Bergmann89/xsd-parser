use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags},
    Config,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    let mut config = Config::test_default()
        .with_parser_flags(ParserFlags::all())
        .with_interpreter_flags(InterpreterFlags::all())
        .with_optimizer_flags(OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES)
        .with_generator_flags(GeneratorFlags::all());

    config.generator.type_postfix.type_ = "Type".into();
    config.generator.type_postfix.element = "Element".into();
    config.generator.type_postfix.element_type = "ElementType".into();

    config
}

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/vsme/schema/xbrl.efrag.org/taxonomy/vsme/2025-07-30/vsme-all.xsd",
        "tests/schema/vsme/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/vsme/schema/xbrl.efrag.org/taxonomy/vsme/2025-07-30/vsme-all.xsd",
        "tests/schema/vsme/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
