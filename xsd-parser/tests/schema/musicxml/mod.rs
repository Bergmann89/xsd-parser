use quote::ToTokens;

use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Schema, TypePostfix},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::meta::MetaTypeVariant,
    Config, MetaTypes,
};

use crate::utils::generate_test_validate;

fn config() -> Config {
    let mut config = Config::default()
        .with_parser_flags(ParserFlags::all())
        .with_interpreter_flags(InterpreterFlags::all())
        .with_optimizer_flags(OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES)
        .with_generator_flags(GeneratorFlags::all() - GeneratorFlags::FLATTEN_CONTENT);

    config.generator.type_postfix = TypePostfix {
        type_: String::new(),
        element: "Element".to_string(),
        element_type: "ElementType".to_string(),
        nillable_content: "NotNil".to_string(),
        dynamic_element: "Dyn".to_string(),
    };

    config
}

fn generate_test(input_xsd: &str, expected_rs: &str, mut config: Config) {
    config.parser.schemas.push(Schema::File(input_xsd.into()));

    let schemas = exec_parser(config.parser).unwrap();
    let meta_types = exec_interpreter(config.interpreter, &schemas).unwrap();
    let meta_types = replace_variant_names(meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types).unwrap();
    let data_types = exec_generator(config.generator, &schemas, &meta_types).unwrap();
    let module = exec_render(config.renderer, &data_types).unwrap();
    let code = module.to_token_stream().to_string();

    generate_test_validate(code, expected_rs);
}

fn replace_variant_names(mut types: MetaTypes) -> MetaTypes {
    for (ident, ty) in types.items.iter_mut() {
        if ident.name.as_named_str() == Some("barline") {
            if let MetaTypeVariant::ComplexType(complex_type) = &mut ty.variant {
                for attribute in complex_type.attributes.iter_mut() {
                    match attribute.ident.name.as_str() {
                        "segno" => attribute.display_name = Some("segno_attr".to_string()),
                        "coda" => attribute.display_name = Some("coda_attr".to_string()),
                        _ => {}
                    }
                }
            }
        }
    }

    types
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/musicxml/schema/schema.xsd",
        "tests/schema/musicxml/expected/default.rs",
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
        "tests/schema/musicxml/schema/schema.xsd",
        "tests/schema/musicxml/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::ScorePartwiseElement;

    let _obj = crate::utils::quick_xml_read_test::<ScorePartwiseElement, _>(
        "tests/schema/musicxml/examples/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
