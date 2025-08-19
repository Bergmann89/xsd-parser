use quote::ToTokens;
use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render, Config,
};

use crate::utils::{generate_test_validate, ConfigEx};

fn config() -> Config {
    let mut config = Config::test_default()
        .with_schemas([
            Schema::file("tests/schema/ofd/schema/Annotations.xsd"),
            Schema::file("tests/schema/ofd/schema/Annotion.xsd"),
            Schema::file("tests/schema/ofd/schema/Attachments.xsd"),
            Schema::file("tests/schema/ofd/schema/CustomTags.xsd"),
            Schema::file("tests/schema/ofd/schema/Definition.xsd"),
            Schema::file("tests/schema/ofd/schema/Document.xsd"),
            Schema::file("tests/schema/ofd/schema/Extensions.xsd"),
            Schema::file("tests/schema/ofd/schema/OFD.xsd"),
            Schema::file("tests/schema/ofd/schema/Page.xsd"),
            Schema::file("tests/schema/ofd/schema/Res.xsd"),
            Schema::file("tests/schema/ofd/schema/Signatures.xsd"),
            Schema::file("tests/schema/ofd/schema/Signature.xsd"),
            Schema::file("tests/schema/ofd/schema/Version.xsd"),
        ])
        .with_parser_flags(ParserFlags::all())
        .with_interpreter_flags(InterpreterFlags::all())
        .with_generator_flags(GeneratorFlags::all())
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::all());

    config.parser.debug_output = Some("target/parser.log".into());
    config.interpreter.debug_output = Some("target/interpreter.log".into());
    config.optimizer.debug_output = Some("target/optimizer.log".into());

    config.generator.type_postfix.type_ = "XType".into();
    config.generator.type_postfix.element = String::new();
    config.generator.type_postfix.element_type = "XElementType".into();

    config
}

#[test]
fn generate_default() {
    generate_test("tests/schema/ofd/expected/default.rs", config());
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/ofd/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{Ofd, Page};

    let _obj = crate::utils::quick_xml_read_test::<Ofd, _>("tests/schema/ofd/examples/OFD.xml");
    let _obj =
        crate::utils::quick_xml_read_test::<Page, _>("tests/schema/ofd/examples/Content.xml");
}

fn generate_test(expected_rs: &str, mut config: Config) {
    let schemas = exec_parser(config.parser).unwrap();
    let meta_types = exec_interpreter(config.interpreter, &schemas).unwrap();
    let meta_types = exec_optimizer(config.optimizer, meta_types).unwrap();
    let data_types = exec_generator(config.generator, &schemas, &meta_types).unwrap();
    let module = exec_render(config.renderer, &data_types).unwrap();
    let code = module.to_token_stream().to_string();

    generate_test_validate(code, expected_rs);
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
