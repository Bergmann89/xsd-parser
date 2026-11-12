use quote::ToTokens;
use xsd_parser::{
    config::{GeneratorFlags, IdentTriple, OptimizerFlags, RenderStep, TypedefMode},
    models::{
        data::{ComplexData, DataTypeVariant, StructMode},
        meta::{MetaTypeVariant, ReferenceMeta},
        schema::MaxOccurs,
    },
    Config, IdentType, TypeIdent,
};

use crate::utils::{generate_test_validate, render_with_customizations, ConfigEx};

fn render(mode: TypedefMode, split: bool, merge_enums: bool) -> String {
    let mut config = Config::test_default()
        .set_optimizer_flags(if merge_enums {
            OptimizerFlags::MERGE_ENUM_UNIONS
        } else {
            OptimizerFlags::empty()
        })
        .with_generate([(IdentType::Element, "tns:Message")])
        .with_typedef_mode(mode)
        .with_derive(["Debug", "PartialEq"])
        .with_render_step(RenderStep::TypesSerdeQuickXml {
            large_arrays: true,
            skip_none: false,
            skip_empty_vecs: false,
        });
    if split {
        config
            .generator
            .flags
            .insert(GeneratorFlags::USE_TYPE_MODULES);
    }
    let module = render_with_customizations(
        "tests/feature/serde_large_arrays/wrappers.xsd",
        config,
        |schemas, meta| {
            let row = IdentTriple::from((IdentType::Type, "tns:RowType"))
                .resolve(schemas)
                .unwrap();
            let matrix = IdentTriple::from((IdentType::Type, "tns:MatrixType"))
                .resolve(schemas)
                .unwrap();
            // Exercise array references through the public meta model. Schema frontends
            // may otherwise choose a record or a list newtype for these shapes.
            let mut reference = ReferenceMeta::new(TypeIdent::STRING);
            reference.min_occurs = 64;
            reference.max_occurs = MaxOccurs::Bounded(64);
            meta.items.get_mut(&row).unwrap().variant = MetaTypeVariant::Reference(reference);
            let mut reference = ReferenceMeta::new(row);
            reference.min_occurs = 2;
            reference.max_occurs = MaxOccurs::Bounded(2);
            meta.items.get_mut(&matrix).unwrap().variant = MetaTypeVariant::Reference(reference);
        },
        |data| {
            let mut boxed = false;
            for item in data.items.values_mut() {
                if let DataTypeVariant::Complex(ComplexData::Struct { type_, .. }) =
                    &mut item.variant
                {
                    if let StructMode::Sequence { elements, .. } = &mut type_.mode {
                        for element in elements {
                            if matches!(element.s_name.as_str(), "Boxed" | "BigBoxes") {
                                element.need_indirection = true;
                                boxed = true;
                            }
                        }
                    }
                }
            }
            assert!(boxed);
        },
    );
    module.to_token_stream().to_string()
}

fn generate(expected: &str, mode: TypedefMode, split: bool) {
    generate_test_validate(render(mode, split, false), expected);
}

#[test]
fn merged_enumeration_payloads_receive_array_adapters() {
    // Typed serde(other) fallbacks cannot derive Serde; compiled fixtures keep this union unmerged.
    let code = render(TypedefMode::Typedef, false, true);
    let enumeration = code.split("pub enum TaggedType").nth(1).unwrap();
    assert!(
        enumeration.contains("RowType (# [serde (with ="),
        "{enumeration}"
    );
    assert!(
        enumeration.contains("[::serde_with::Same; 64]"),
        "{enumeration}"
    );
}

#[test]
fn generate_aliases() {
    generate(
        "tests/feature/serde_large_arrays/expected/aliases.rs",
        TypedefMode::Typedef,
        false,
    );
}

#[test]
fn generate_newtypes() {
    generate(
        "tests/feature/serde_large_arrays/expected/newtypes.rs",
        TypedefMode::NewType,
        false,
    );
}

#[test]
fn generate_aliases_type_modules() {
    generate(
        "tests/feature/serde_large_arrays/expected/aliases_type_modules.rs",
        TypedefMode::Typedef,
        true,
    );
}

#[test]
fn generate_newtypes_type_modules() {
    generate(
        "tests/feature/serde_large_arrays/expected/newtypes_type_modules.rs",
        TypedefMode::NewType,
        true,
    );
}

#[cfg(not(feature = "update-expectations"))]
mod aliases {
    include!("expected/aliases.rs");
}
#[cfg(not(feature = "update-expectations"))]
mod newtypes {
    include!("expected/newtypes.rs");
}
#[cfg(not(feature = "update-expectations"))]
mod aliases_type_modules {
    // Public API re-exports are intentionally only partly used by this private fixture.
    #![allow(unused_imports)]
    include!("expected/aliases_type_modules.rs");
}
#[cfg(not(feature = "update-expectations"))]
mod newtypes_type_modules {
    #![allow(unused_imports)]
    include!("expected/newtypes_type_modules.rs");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn aliases_and_newtypes_round_trip_in_both_layouts() {
    fn check<T: serde::Serialize + serde::de::DeserializeOwned>() {
        let row = serde_json::json!(vec!["value"; 64]);
        let value = serde_json::json!({
            "Row": row,
            "Rows": [row],
            "Matrix": [row, row],
            "Boxed": row,
            "BigBoxes": row,
            "List": [row],
            "Choice": {"RowType": row},
            "Tagged": {"RowType": row},
        });
        let decoded: T = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), value);
        let mut missing = value;
        missing.as_object_mut().unwrap().remove("Row");
        let decoded: T = serde_json::from_value(missing.clone()).unwrap();
        missing["Row"] = serde_json::Value::Null;
        assert_eq!(serde_json::to_value(decoded).unwrap(), missing);
    }
    check::<aliases::MessageType>();
    check::<newtypes::MessageType>();
    check::<aliases_type_modules::MessageType>();
    check::<newtypes_type_modules::MessageType>();
}
