use std::path::PathBuf;

use xsd_parser::{
    config::{IdentTriple, InterpreterFlags, OptimizerFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser,
    types::IdentType,
    Config,
};

use crate::utils::{generate_test_validate, ConfigEx};

#[test]
fn generate_default() {
    let ident_1 = IdentTriple::from((IdentType::Type, "tns:fooType"));
    let ident_2 = IdentTriple::from((IdentType::Type, "tns:FooType"));

    let mut config = Config::test_default()
        .with_interpreter_flags(InterpreterFlags::all())
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

    let types = exec_interpreter(config.interpreter, &schemas).expect("Interpreter failed");
    let mut types = exec_optimizer(config.optimizer, types).expect("Optimizer failed");

    let ty1 = types
        .get_mut(&ident_1)
        .expect("Failed to resolve `tns:fooType`");
    *ty1.display_name_mut() = Some("Bar".into());

    let ty2 = types
        .get_mut(&ident_2)
        .expect("Failed to resolve `tns:FooType`");
    *ty2.display_name_mut() = Some("Baz".into());

    let code = exec_generator(config.generator, &schemas, &types).expect("Generator failed");
    let code = code.to_string();

    generate_test_validate(code, "tests/feature/type_name_clash/expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}
