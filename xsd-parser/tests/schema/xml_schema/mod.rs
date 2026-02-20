use xsd_parser::{
    config::{GeneratorFlags, OptimizerFlags, ParserFlags, RendererFlags},
    Config, IdentType,
};

use crate::utils::generate_test;

fn config() -> Config {
    Config::default()
        .with_quick_xml_deserialize()
        .with_parser_flags(ParserFlags::all())
        .with_optimizer_flags(
            OptimizerFlags::all()
                - OptimizerFlags::SIMPLIFY_MIXED_TYPES
                - OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE_SIMPLE,
        )
        .with_generator_flags(
            GeneratorFlags::all()
                - GeneratorFlags::ADVANCED_ENUMS
                - GeneratorFlags::USE_NAMESPACE_MODULES
                - GeneratorFlags::USE_SCHEMA_MODULES
                - GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS
                - GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS,
        )
        .with_renderer_flags(RendererFlags::RENDER_DOCS)
        .with_any_type_support()
        .with_generate([(IdentType::Element, "xs:schema")])
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "schema/XMLSchema.xsd",
        "tests/schema/xml_schema/expected/quick_xml.rs",
        config(),
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
