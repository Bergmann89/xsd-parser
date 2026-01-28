use quote::ToTokens;
use xsd_parser::{
    config::Schema,
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::{meta::MetaTypeVariant, ExplicitNaming},
    Config, MetaTypes,
};

use crate::utils::{generate_test_validate, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_naming(ExplicitNaming::new().with_attribute_field_postfix("_attrib"))
        .with_type_postfix("XType")
        .with_element_type_postfix("XElementType")
}

fn generate_test(input_xsd: &str, expected_rs: &str, mut config: Config) {
    config.parser.schemas.push(Schema::File(input_xsd.into()));

    let schemas = exec_parser(config.parser).unwrap();
    let meta_types = exec_interpreter(config.interpreter, &schemas).unwrap();
    let meta_types = replace_variant_names(meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types).unwrap();
    let data_types = exec_generator(config.generator, &schemas, &meta_types).unwrap();
    let module = exec_render(config.renderer, &data_types).unwrap();
    let code = module.to_token_stream().to_string();

    generate_test_validate(code, expected_rs);
}

fn replace_variant_names(mut types: MetaTypes) -> MetaTypes {
    for (ident, ty) in types.items.iter_mut() {
        if let MetaTypeVariant::Enumeration(x) = &mut ty.variant {
            for var in &mut *x.variants {
                if var.ident.name.as_str() == "+" {
                    var.display_name = Some("Plus".into());
                } else if var.ident.name.as_str() == "-" {
                    var.display_name = Some("Minus".into());
                } else if var.ident.name.as_str() == "%" {
                    var.display_name = Some("Percent".into());
                }
            }
        }
    }

    types
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/opendrive/v1_4/OpenDRIVE_1.4H.xsd",
        "tests/schema/opendrive/v1_4/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
