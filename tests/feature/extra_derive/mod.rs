use std::path::PathBuf;

use quote::{format_ident, ToTokens};
use xsd_parser::{
    config::{IdentTriple, OptimizerFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::data::ConfigValue,
    Config, IdentType,
};

use crate::utils::{generate_test_validate, ConfigEx};

#[test]
fn generate_default() {
    let mut config = Config::test_default()
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generate([
            (IdentType::Type, "tns:FooType"),
            (IdentType::Type, "tns:BarType"),
            (IdentType::Type, "tns:BazType"),
        ]);

    config.parser.schemas.push(Schema::File(PathBuf::from(
        "tests/feature/extra_derive/schema.xsd",
    )));

    let schemas = exec_parser(config.parser).expect("Parser failed");
    let meta_types = exec_interpreter(config.interpreter, &schemas).expect("Interpreter failed");
    let meta_types = exec_optimizer(config.optimizer, meta_types).expect("Optimizer failed");
    let mut data_types =
        exec_generator(config.generator, &schemas, &meta_types).expect("Generator failed");

    // Set extra derive for `tns:FooType`
    let ident = IdentTriple::from((IdentType::Type, "tns:FooType"))
        .resolve(&schemas)
        .expect("Unable to resolve ident for `tns:FooType`");
    let data_type = data_types
        .items
        .get_mut(&ident)
        .expect("Unable to get data type");
    data_type.derive = ConfigValue::Extend(vec![
        format_ident!("Default"),
        format_ident!("Eq"),
        format_ident!("PartialEq"),
        format_ident!("Hash"),
    ]);

    // Set overwrite derive for `tns:BarType`
    let ident = IdentTriple::from((IdentType::Type, "tns:BarType"))
        .resolve(&schemas)
        .expect("Unable to resolve ident for `tns:BarType`");
    let data_type = data_types
        .items
        .get_mut(&ident)
        .expect("Unable to get data type");
    data_type.derive = ConfigValue::Overwrite(vec![]);

    let module = exec_render(config.renderer, &data_types).expect("Renderer failed");
    let code = module.to_token_stream().to_string();

    generate_test_validate(code, "tests/feature/extra_derive/expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
