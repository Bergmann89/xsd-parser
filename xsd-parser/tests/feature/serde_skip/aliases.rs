use quote::ToTokens;
use xsd_parser::{
    config::{IdentTriple, OptimizerFlags, RenderStep, TypedefMode},
    models::{
        data::{ComplexData, ComplexDataStruct, DataTypeVariant, StructMode},
        meta::{MetaTypeVariant, ReferenceMeta},
        schema::MaxOccurs,
    },
    Config, IdentType, TypeIdent,
};

use crate::utils::{generate_test_validate, render_with_customizations, ConfigEx};

#[test]
fn generate_option_and_vector_aliases() {
    generate_custom("tests/feature/serde_skip/expected/aliases.rs", |_| {});
}

pub(super) fn generate_custom(expected: &str, customize: impl Fn(&mut ComplexDataStruct<'_>)) {
    generate_test_validate(render(customize), expected);
}

fn render(customize: impl Fn(&mut ComplexDataStruct<'_>)) -> String {
    let config = Config::test_default()
        .set_optimizer_flags(OptimizerFlags::empty())
        .with_generate([(IdentType::Type, "tns:AliasesType")])
        .with_typedef_mode(TypedefMode::Typedef)
        .with_derive(["Debug", "PartialEq"])
        .with_render_step(RenderStep::TypesSerdeQuickXml {
            large_arrays: true,
            skip_none: true,
            skip_empty_vecs: true,
        });
    let module = render_with_customizations(
        "tests/feature/serde_skip/aliases.xsd",
        config,
        |schemas, meta| {
            // Use the public meta model to exercise aliases whose outer Option or Vec
            // is hidden from a procedural macro inspecting only the Rust field syntax.
            let array = IdentTriple::from((IdentType::Type, "tns:ArrayType"))
                .resolve(schemas)
                .unwrap();
            for (name, target, min, max) in [
                (
                    "tns:OptionalType",
                    TypeIdent::STRING,
                    0,
                    MaxOccurs::Bounded(1),
                ),
                ("tns:ItemsType", TypeIdent::STRING, 1, MaxOccurs::Unbounded),
                (
                    "tns:ArrayType",
                    TypeIdent::STRING,
                    64,
                    MaxOccurs::Bounded(64),
                ),
                (
                    "tns:OptionalArrayType",
                    array.clone(),
                    0,
                    MaxOccurs::Bounded(1),
                ),
                ("tns:ArrayItemsType", array, 1, MaxOccurs::Unbounded),
            ] {
                let ident = IdentTriple::from((IdentType::Type, name))
                    .resolve(schemas)
                    .unwrap();
                let mut reference = ReferenceMeta::new(target);
                reference.min_occurs = min;
                reference.max_occurs = max;
                meta.items.get_mut(&ident).unwrap().variant = MetaTypeVariant::Reference(reference);
            }
        },
        |data| {
            for item in data.items.values_mut() {
                if let DataTypeVariant::Complex(ComplexData::Struct { type_, .. }) =
                    &mut item.variant
                {
                    if let StructMode::Sequence { elements, .. } = &mut type_.mode {
                        for element in elements {
                            if element.s_name == "BoxedItems" {
                                element.need_indirection = true;
                            }
                        }
                    }
                    customize(type_);
                }
            }
        },
    );
    module.to_token_stream().to_string()
}

#[test]
fn required_vector_aliases_receive_defaults() {
    let code = render(|_| {});
    for field in ["items", "boxed_items", "required_values", "array_items"] {
        let before = code.split(&format!("pub {field} :")).next().unwrap();
        let attribute = before.rsplit("# [serde").next().unwrap();
        assert!(attribute.contains("skip_serializing_if"), "{field}: {code}");
        assert!(attribute.contains("default"), "{field}: {code}");
    }
}

#[cfg(not(feature = "update-expectations"))]
mod model {
    include!("expected/aliases.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn recognizes_optional_and_vector_aliases_including_boxed_fields() {
    let empty: model::AliasesType = serde_json::from_value(serde_json::json!({
        "Optional": null, "Items": [], "BoxedItems": [], "OptionalArray": null, "ArrayItems": [],
        "@requiredValues": [],
    }))
    .unwrap();
    assert!(empty.optional.is_none());
    assert!(empty.items.is_empty());
    assert!(empty.boxed_items.is_empty());
    assert!(empty.optional_array.is_none());
    assert!(empty.array_items.is_empty());
    assert!(empty.required_values.is_empty());
    assert_eq!(serde_json::to_string(&empty).unwrap(), "{}");
    let decoded: model::AliasesType = serde_json::from_str("{}").unwrap();
    assert_eq!(decoded, empty);
    let value = serde_json::json!({
        "Optional": "value", "Items": ["item"], "BoxedItems": ["boxed"],
        "OptionalArray": vec!["array"; 64], "ArrayItems": [vec!["array item"; 64]],
        "@requiredValues": ["required"],
    });
    let decoded: model::AliasesType = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), value);
}
