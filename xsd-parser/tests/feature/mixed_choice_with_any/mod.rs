use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, RendererFlags},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_parser_flags(ParserFlags::all())
        .set_interpreter_flags(InterpreterFlags::BUILDIN_TYPES | InterpreterFlags::DEFAULT_TYPEDEFS)
        .set_optimizer_flags(OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES)
        .set_generator_flags(
            GeneratorFlags::all()
                - GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS
                - GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS
                - GeneratorFlags::ANY_TYPE_SUPPORT,
        )
        .set_renderer_flags(RendererFlags::all())
        .with_generate([(IdentType::Element, "tns:SDL")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/mixed_choice_with_any/schema.xsd",
        "tests/feature/mixed_choice_with_any/expected/default.rs",
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
        "tests/feature/mixed_choice_with_any/schema.xsd",
        "tests/feature/mixed_choice_with_any/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::tns::{ContainerTypeContent, Sdl};

    let obj = crate::utils::quick_xml_read_test::<Sdl, _>(
        "tests/feature/mixed_choice_with_any/example/default.xml",
    );

    let mut it = obj.container.content.into_iter();

    assert!(matches!(it.next(), Some(ContainerTypeContent::Text(x)) if &x.0 == "\n        "));
    assert!(matches!(it.next(), Some(ContainerTypeContent::Text(x)) if &x.0 == "\n    "));
    assert!(it.next().is_none());
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
