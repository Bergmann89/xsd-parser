use inflector::Inflector;
use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser,
    generator::SerdeSupport,
    schema::{Namespace, Schemas},
    types::{DynamicInfo, Ident, IdentType, TypeVariant, Types},
    Config,
};

use crate::utils::{generate_test_validate, ConfigEx};

#[test]
fn generate_serde_quick_xml() {
    let mut config = Config::test_default()
        .with_serde_support(SerdeSupport::QuickXml)
        .set_interpreter_flags(InterpreterFlags::all())
        .set_optimizer_flags(OptimizerFlags::empty())
        .set_generator_flags(GeneratorFlags::all());

    config.parser.schemas.push(Schema::File(
        "tests/schema/aixm_5_1_1/schema/aixm/message/AIXM_BasicMessage.xsd".into(),
    ));

    config.generator.type_postfix.type_ = "XType".into();
    config.generator.type_postfix.element = "XElement".into();
    config.generator.type_postfix.element_type = "XElementType".into();

    config.parser.debug_output = Some("target/parser.log".into());
    config.interpreter.debug_output = Some("target/interpreter.log".into());
    config.optimizer.debug_output = Some("target/optimizer.log".into());

    let schemas = exec_parser(config.parser).unwrap();
    let mut types = exec_interpreter(config.interpreter, &schemas).unwrap();

    fix_gml_property_names(&schemas, &mut types);
    rename_gml_property_elements(
        &schemas,
        &mut types,
        &[
            ("conversion", IdentType::Element, "ConversionProperty"),
            (
                "parameterValue",
                IdentType::Element,
                "ParameterValueProperty",
            ),
            (
                "secondDefiningParameter",
                IdentType::Element,
                "SecondDefiningParameterProperty",
            ),
            (
                "secondDefiningParameter",
                IdentType::ElementType,
                "SecondDefiningParameterPropertyType",
            ),
        ],
    );

    for ty in types.values_mut() {
        let TypeVariant::Enumeration(ei) = &mut ty.variant else {
            continue;
        };

        for var in &mut *ei.variants {
            match var.ident.name.as_str() {
                "*" => var.display_name = Some("Wildcard".into()),
                "+" => var.display_name = Some("Plus".into()),
                "-" => var.display_name = Some("Minus".into()),
                "+x+y" => var.display_name = Some("PlusXPlusY".into()),
                "+y+x" => var.display_name = Some("PlusYPlusX".into()),
                "+x-y" => var.display_name = Some("PlusXMinusY".into()),
                "-x-y" => var.display_name = Some("MinusXMinusY".into()),
                s if s.starts_with("UTC-") => {
                    var.display_name = Some(s.replace("UTC-", "UtcMinus"))
                }
                s if s.starts_with("UTC+") => var.display_name = Some(s.replace("UTC+", "UtcPlus")),
                _ => (),
            }
        }
    }

    let types = exec_optimizer(config.optimizer, types).unwrap();
    let code = exec_generator(config.generator, &schemas, &types).unwrap();
    let code = code.to_string();

    generate_test_validate(&code, "tests/schema/aixm_5_1_1/expected/serde_quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    include!("expected/serde_quick_xml.rs");
}

fn fix_gml_property_names(schemas: &Schemas, types: &mut Types) {
    let gml = Namespace::new_const(b"http://www.opengis.net/gml/3.2");
    let gml = schemas
        .resolve_namespace(&Some(gml))
        .expect("Namespace not found");

    for (ident, ty) in &mut **types {
        if ident.ns != Some(gml) {
            continue;
        }

        let type_ = match &ty.variant {
            TypeVariant::Dynamic(DynamicInfo {
                type_: Some(type_), ..
            }) => type_.name.as_str().to_lowercase(),
            TypeVariant::Reference(ri) => ri.type_.name.as_str().to_lowercase(),
            _ => continue,
        };

        let name = ident.name.as_str().to_lowercase();
        if let Some(s) = type_.strip_prefix(&name) {
            if s.starts_with("property") {
                let name = ident.name.as_str().to_pascal_case();
                ty.display_name = Some(format!("{name}PropertyElement"));
            }
        }
    }
}

fn rename_gml_property_elements(
    schemas: &Schemas,
    types: &mut Types,
    names: &'static [(&'static str, IdentType, &'static str)],
) {
    let gml = Namespace::new_const(b"http://www.opengis.net/gml/3.2");
    let gml = schemas
        .resolve_namespace(&Some(gml))
        .expect("Namespace not found");

    for (ident, type_, name) in names {
        if let Some(ty) = dbg!(types.get_mut(&dbg!(Ident::element(ident)
            .with_type(*type_)
            .with_ns(Some(gml)))))
        {
            ty.display_name = Some((*name).into());
        }
    }
}
