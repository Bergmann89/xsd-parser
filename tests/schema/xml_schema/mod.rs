use xsd_parser::{
    config::{
        Generate, GenerateFlags, IdentTriple, InterpreterFlags, OptimizerFlags, ParserFlags,
        Resolver,
    },
    types::IdentType,
    Config,
};

use crate::utils::generate_test;

#[test]
fn generate_quick_xml() {
    let mut config = Config::default();

    config.parser.flags = ParserFlags::all();
    config.parser.resolver = vec![Resolver::File];

    config.interpreter.flags = InterpreterFlags::all();

    config.optimizer.flags = OptimizerFlags::all();

    config.generator.flags =
        GenerateFlags::all() - GenerateFlags::QUICK_XML_SERIALIZE - GenerateFlags::USE_MODULES;
    config.generator.generate =
        Generate::Types(vec![IdentTriple::from((IdentType::Element, "xs:schema"))]);

    generate_test(
        "schema/XMLSchema.xsd",
        "tests/schema/xml_schema/expected/quick_xml.rs",
        config,
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Schema;

    let _obj = crate::utils::quick_xml_read_test::<Schema, _>("schema/XMLSchema.xsd");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
