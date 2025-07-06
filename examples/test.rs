#![allow(missing_docs)]

use anyhow::Error;
use quote::ToTokens as _;
use std::fs::write;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use xsd_parser::{
    config::{
        GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema,
    },
    exec_generator, exec_optimizer, exec_parser, exec_render, Config, Interpreter,
};
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> Result<(), Error> {
    fmt()
        .without_time()
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut config = Config::default();
    config.parser.resolver = vec![Resolver::File];
    config.parser.flags = ParserFlags::all();
    config.parser.schemas = vec![Schema::File("xmls/scap-source-data-stream_1.2.xsd".into())];
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GeneratorFlags::all();
    let schemas = exec_parser(config.parser)?;

    let meta_types = Interpreter::new(&schemas)
        .with_buildin_types()?
        .with_default_typedefs()?
        .with_xs_any_type()?
        .finish()?;
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;
    let code = module.to_token_stream().to_string();
    let formatted_code = rustfmt_pretty_print(code.to_string())?;

    write("target/test.rs", formatted_code)?;

    Ok(())
}

fn rustfmt_pretty_print(code: String) -> Result<String, Error> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();

    write!(stdin, "{code}")?;
    stdin.flush()?;
    drop(stdin);

    let Output {
        status,
        stdout,
        stderr,
    } = child.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&stdout);
    let stderr = String::from_utf8_lossy(&stderr);

    if !status.success() {
        let code = status.code();
        match code {
            Some(code) => {
                if code != 0 {
                    panic!("The `rustfmt` command failed with return code {code}!\n{stderr}");
                }
            }
            None => {
                panic!("The `rustfmt` command failed!\n{stderr}")
            }
        }
    }

    Ok(stdout.into())
}
