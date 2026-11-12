use std::path::Path;

use quote::ToTokens;
use xsd_parser::{
    config::{GeneratorFlags, IdentTriple, OptimizerFlags, RenderStep, Schema, TypedefMode},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::{
        meta::{MetaTypeVariant, ReferenceMeta},
        schema::MaxOccurs,
    },
    Config, IdentType, TypeIdent,
};

use crate::utils::ConfigEx;

fn generate(large_arrays: bool) -> String {
    let mut config = Config::test_default()
        .set_optimizer_flags(OptimizerFlags::empty())
        .with_generator_flags(
            GeneratorFlags::FLATTEN_CONTENT
                | GeneratorFlags::NILLABLE_TYPE_SUPPORT
                | GeneratorFlags::MIXED_TYPE_SUPPORT,
        )
        .with_generate([
            (IdentType::Type, "tns:NillableType"),
            (IdentType::Type, "tns:MixedType"),
            (IdentType::Type, "tns:AliasesType"),
        ])
        .with_typedef_mode(TypedefMode::Typedef);
    config = config.with_render_step(RenderStep::TypesSerdeQuickXml { large_arrays });
    config.parser.schemas.push(Schema::File(
        Path::new("tests/feature/serde_large_arrays/xml_wrappers.xsd")
            .canonicalize()
            .unwrap(),
    ));
    let schemas = exec_parser(config.parser).unwrap();
    let mut meta = exec_interpreter(config.interpreter, &schemas).unwrap();
    let row = IdentTriple::from((IdentType::Type, "tns:RowType"))
        .resolve(&schemas)
        .unwrap();
    let nillable = IdentTriple::from((IdentType::Type, "tns:NillableRowType"))
        .resolve(&schemas)
        .unwrap();
    let mut reference = ReferenceMeta::new(TypeIdent::STRING);
    reference.min_occurs = 64;
    reference.max_occurs = MaxOccurs::Bounded(64);
    meta.items.get_mut(&row).unwrap().variant = MetaTypeVariant::Reference(reference);
    let mut reference = ReferenceMeta::new(row);
    reference.nillable = true;
    meta.items.get_mut(&nillable).unwrap().variant = MetaTypeVariant::Reference(reference);
    let meta = exec_optimizer(config.optimizer, meta).unwrap();
    let data = exec_generator(config.generator, &schemas, &meta).unwrap();
    exec_render(config.renderer, &data)
        .unwrap()
        .to_token_stream()
        .to_string()
}

#[test]
fn xml_wrappers_do_not_receive_adapters_for_their_inner_aliases() {
    let code = generate(true);
    assert!(code.contains("Nillable < RowType >"), "{code}");
    assert!(code.contains("Mixed < RowType >"), "{code}");
    assert_eq!(code, generate(false));
}
