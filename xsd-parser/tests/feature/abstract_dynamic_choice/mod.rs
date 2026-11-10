mod xml;

use xsd_parser::{
    config::{IdentQuadruple, OptimizerFlags, Schema},
    exec_interpreter, exec_parser,
    models::meta::{DynamicMeta, MetaTypeVariant},
    Config, IdentType, Optimizer,
};

use crate::utils::ConfigEx;

fn config() -> Config {
    Config::test_default()
        .with_schema(Schema::schema(include_str!("schema.xsd")))
        .with_optimizer_flags(OptimizerFlags::CONVERT_DYNAMIC_TO_CHOICE)
}

#[test]
fn individual_conversions_are_order_independent() {
    for (order, bulk) in [
        (["tns:Payload", "tns:Middle"], false),
        (["tns:Middle", "tns:Payload"], false),
        (["tns:Middle", "tns:Payload"], true),
    ] {
        let config = config();
        let schemas = exec_parser(config.parser).unwrap();
        let meta = exec_interpreter(config.interpreter, &schemas).unwrap();
        let idents = order.map(|name| {
            IdentQuadruple::from((IdentType::Type, name))
                .resolve(&schemas)
                .unwrap()
        });
        let mut optimizer = Optimizer::new(meta);
        if bulk {
            optimizer = optimizer
                .convert_dynamic_to_choice(idents[0].clone())
                .unwrap()
                .convert_dynamics_to_choices();
        } else {
            for ident in &idents {
                optimizer = optimizer.convert_dynamic_to_choice(ident.clone()).unwrap();
            }
        }
        let meta = optimizer.finish();
        for ident in &idents {
            let MetaTypeVariant::ComplexType(complex) = meta.get_variant(ident).unwrap() else {
                panic!("expected converted dynamic type");
            };
            let MetaTypeVariant::Choice(group) =
                meta.get_variant(complex.content.as_ref().unwrap()).unwrap()
            else {
                panic!("expected choice content");
            };
            assert_eq!(group.elements.len(), 1);
            assert_eq!(group.elements[0].ident.name.as_str(), "Leaf");
        }
    }
}

#[test]
fn individual_conversion_preserves_other_dynamic_types() {
    let config = config();
    let schemas = exec_parser(config.parser).unwrap();
    let meta = exec_interpreter(config.interpreter, &schemas).unwrap();
    let empty = IdentQuadruple::from((IdentType::Type, "tns:Empty"))
        .resolve(&schemas)
        .unwrap();
    let unconverted = meta
        .items
        .iter()
        .filter_map(|(ident, ty)| {
            if ident == &empty {
                return None;
            }
            match &ty.variant {
                MetaTypeVariant::Dynamic(dynamic) => Some((ident.clone(), dynamic.clone())),
                _ => None,
            }
        })
        .collect::<Vec<_>>();
    let meta = Optimizer::new(meta)
        .convert_dynamic_to_choice(empty)
        .unwrap()
        .finish();
    for (ident, expected) in unconverted {
        let MetaTypeVariant::Dynamic(actual) = meta.get_variant(&ident).unwrap() else {
            panic!("unselected type {ident} must remain dynamic");
        };
        assert_eq!(actual.type_, expected.type_, "{ident}");
        assert_eq!(actual.is_abstract, expected.is_abstract, "{ident}");
        let entries = |dynamic: &DynamicMeta| {
            dynamic
                .derived_types
                .iter()
                .map(|(ident, derived)| {
                    (
                        ident.clone(),
                        derived.type_.clone(),
                        derived.display_name.clone(),
                        derived.relationship,
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(entries(actual), entries(&expected), "{ident}");
    }
}
