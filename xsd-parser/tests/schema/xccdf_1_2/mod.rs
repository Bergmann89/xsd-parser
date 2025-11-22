use quote::ToTokens;
use xsd_parser::{
    config::{GeneratorFlags, IdentTriple, OptimizerFlags, RendererFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::meta::{AttributeMetaVariant, MetaTypeVariant},
    Config, IdentType, MetaTypes, Schemas,
};

use crate::utils::generate_test_validate;

fn config() -> Config {
    Config::default()
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(
            GeneratorFlags::all()
                - GeneratorFlags::USE_SCHEMA_MODULES
                - GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS
                - GeneratorFlags::NILLABLE_TYPE_SUPPORT
                - GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS,
        )
        .with_renderer_flags(RendererFlags::RENDER_DOCS)
        .with_any_type_support()
        .with_generate([(IdentType::Element, "cdf:Benchmark")])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/xccdf_1_2/schema/xccdf_1.2.xsd",
        "tests/schema/xccdf_1_2/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/xccdf_1_2/schema/xccdf_1.2.xsd",
        "tests/schema/xccdf_1_2/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use crate::utils::quick_xml_read_test;
    use quick_xml::cdf::Benchmark;

    quick_xml_read_test::<Benchmark, _>("tests/schema/xccdf_1_2/examples/benchmark.xml");
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

fn generate_test(input_xsd: &str, expected_rs: &str, mut config: Config) {
    config.parser.schemas.push(Schema::File(input_xsd.into()));

    let schemas = exec_parser(config.parser).unwrap();
    let meta_types = exec_interpreter(config.interpreter, &schemas).unwrap();
    let meta_types = resolve_naming_conflicts(&schemas, meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types).unwrap();
    let data_types = exec_generator(config.generator, &schemas, &meta_types).unwrap();
    let module = exec_render(config.renderer, &data_types).unwrap();
    let code = module.to_token_stream().to_string();

    generate_test_validate(code, expected_rs);
}

fn resolve_naming_conflicts(schemas: &Schemas, mut types: MetaTypes) -> MetaTypes {
    let xsd_id = IdentTriple::from((IdentType::Type, "xs:ID"))
        .resolve(schemas)
        .unwrap();

    for (ident, ty) in types.iter_items_mut() {
        if let MetaTypeVariant::ComplexType(ci) = &mut ty.variant {
            for attrib in &mut *ci.attributes {
                if attrib.ident.name.as_str() == "Id"
                    && matches!(&attrib.variant, AttributeMetaVariant::Type(x) if x.matches(&xsd_id))
                {
                    attrib.display_name = Some("xml_id".into());
                }
            }
        }
    }

    types
}
