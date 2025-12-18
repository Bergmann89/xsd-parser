use std::path::PathBuf;

use quote::ToTokens;
use xsd_parser::{
    config::{IdentTriple, OptimizerFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render, Config, IdentType,
};

use crate::utils::{generate_test_validate, ConfigEx};

#[test]
fn generate_default() {
    let ident_1 = IdentTriple::from((IdentType::Type, "tns:fooType"));
    let ident_2 = IdentTriple::from((IdentType::Type, "tns:FooType"));

    let mut config = Config::test_default()
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generate([ident_1.clone(), ident_2.clone()]);

    config.parser.schemas.push(Schema::File(PathBuf::from(
        "tests/feature/type_name_clash/schema.xsd",
    )));

    let schemas = exec_parser(config.parser).expect("Parser failed");
    let ident_1 = ident_1
        .resolve(&schemas)
        .expect("Unable to resolve ident triple");
    let ident_2 = ident_2
        .resolve(&schemas)
        .expect("Unable to resolve ident triple");

    let meta_types = exec_interpreter(config.interpreter, &schemas).expect("Interpreter failed");
    let mut meta_types = exec_optimizer(config.optimizer, meta_types).expect("Optimizer failed");

    let ty1 = meta_types
        .items
        .get_mut(&ident_1)
        .expect("Failed to resolve `tns:fooType`");
    ty1.display_name = Some("Bar".into());

    let ty2 = meta_types
        .items
        .get_mut(&ident_2)
        .expect("Failed to resolve `tns:FooType`");

    let data_types =
        exec_generator(config.generator, &schemas, &meta_types).expect("Generator failed");
    let module = exec_render(config.renderer, &data_types).expect("Renderer failed");
    let code = module.to_token_stream().to_string();

    generate_test_validate(code, "tests/feature/type_name_clash/expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
