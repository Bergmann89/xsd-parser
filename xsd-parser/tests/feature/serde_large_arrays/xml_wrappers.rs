use quote::ToTokens;
use xsd_parser::{
    config::{GeneratorFlags, IdentTriple, OptimizerFlags, RenderStep, TypedefMode},
    models::{
        meta::{MetaTypeVariant, ReferenceMeta},
        schema::MaxOccurs,
    },
    Config, IdentType, TypeIdent,
};

use crate::utils::{render_with_customizations, ConfigEx};

fn generate(large_arrays: bool, skip: bool) -> String {
    let config = Config::test_default()
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
        .with_typedef_mode(TypedefMode::Typedef)
        .with_render_step(RenderStep::TypesSerdeQuickXml {
            large_arrays,
            skip_none: false,
            skip_empty_vecs: skip,
        });
    render_with_customizations(
        "tests/feature/serde_large_arrays/xml_wrappers.xsd",
        config,
        |schemas, meta| {
            for (name, min, max) in [
                ("tns:RowType", 64, MaxOccurs::Bounded(64)),
                ("tns:ItemsType", 0, MaxOccurs::Unbounded),
            ] {
                let ident = IdentTriple::from((IdentType::Type, name))
                    .resolve(schemas)
                    .unwrap();
                let mut reference = ReferenceMeta::new(TypeIdent::STRING);
                reference.min_occurs = min;
                reference.max_occurs = max;
                meta.items.get_mut(&ident).unwrap().variant = MetaTypeVariant::Reference(reference);
            }
            for (name, target) in [
                ("tns:NillableRowType", "tns:RowType"),
                ("tns:NillableItemsType", "tns:ItemsType"),
            ] {
                let ident = IdentTriple::from((IdentType::Type, name))
                    .resolve(schemas)
                    .unwrap();
                let target = IdentTriple::from((IdentType::Type, target))
                    .resolve(schemas)
                    .unwrap();
                let mut reference = ReferenceMeta::new(target);
                reference.nillable = true;
                meta.items.get_mut(&ident).unwrap().variant = MetaTypeVariant::Reference(reference);
            }
        },
        |_| {},
    )
    .to_token_stream()
    .to_string()
}

#[test]
fn xml_wrappers_do_not_receive_adapters_for_their_inner_aliases() {
    let code = generate(true, false);
    assert!(code.contains("Nillable < RowType >"), "{code}");
    assert!(code.contains("Mixed < RowType >"), "{code}");
    assert_eq!(code, generate(false, false));
}

#[test]
fn xml_wrappers_are_not_treated_as_vector_fields() {
    let code = generate(true, true);
    assert_ne!(code, generate(true, false));
    assert_eq!(code.matches("Vec::is_empty").count(), 1, "{code}");
    let plain = code.split("pub plain :").next().unwrap();
    let attribute = plain.rsplit("# [serde").next().unwrap();
    assert!(attribute.contains("Vec::is_empty"), "{code}");
}
