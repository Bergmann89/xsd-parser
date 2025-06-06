use xsd_parser::{
    config::{Generate, GeneratorFlags, IdentTriple, OptimizerFlags, ParserFlags, Resolver},
    Config, IdentType,
};

use crate::utils::generate_test;

#[test]
fn generate_quick_xml() {
    let mut config = Config::default().with_quick_xml_deserialize();

    config.parser.flags = ParserFlags::all();
    config.parser.resolver = vec![Resolver::File];

    config.optimizer.flags = OptimizerFlags::all();

    config.generator.flags =
        GeneratorFlags::all() - GeneratorFlags::USE_MODULES - GeneratorFlags::RENDER_DOCS;
    config.generator.any_type = Some("xsd_parser::xml::AnyElement".into());
    config.generator.any_attribute_type = Some("xsd_parser::xml::AnyAttributes".into());
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

    let _obj = crate::utils::quick_xml_read_test::<Schema, _>("schema/xml.xsd");
    let _obj = crate::utils::quick_xml_read_test::<Schema, _>("schema/XMLSchema.xsd");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
