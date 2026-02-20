use std::fs::{create_dir_all, write};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Error};
use xsd_parser::config::NamespaceIdent;
use xsd_parser::IdentType;
use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags},
    generate, Config,
};

fn main() -> Result<(), Error> {
    let out_dir = env_path("OUT_DIR");
    let cargo_dir = env_path("CARGO_MANIFEST_DIR")
        .canonicalize()
        .context("Missing environment variable `CARGO_MANIFEST_DIR`")?;
    let schema_file = cargo_dir
        .join("schema/BPMN20.xsd")
        .canonicalize()
        .context("Missing or invalid schema file!")?;

    println!("cargo:rerun-if-changed={}", schema_file.display());

    let mut config = Config::default()
        .with_schema(schema_file)
        .set_interpreter_flags(InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT)
        .set_optimizer_flags(OptimizerFlags::all())
        .set_generator_flags(GeneratorFlags::all() - GeneratorFlags::ADVANCED_ENUMS)
        .with_type_postfix("XType")
        .with_quick_xml()
        .with_generate([(
            IdentType::Element,
            NamespaceIdent::namespace(b"http://www.omg.org/spec/BPMN/20100524/MODEL"),
            "definitions",
        )]);

    let debug_dir = cargo_dir.join("../../target/examples/bpmn");
    create_dir_all(&debug_dir).context("Unable to create debug output dir!")?;
    config.parser.debug_output = Some(debug_dir.join("parser.log"));
    config.interpreter.debug_output = Some(debug_dir.join("interpreter.log"));
    config.optimizer.debug_output = Some(debug_dir.join("optimizer.log"));

    // Generate the code based on the configuration above.
    let code = generate(config)?;
    let code = code.to_string();
    let code = rustfmt_pretty_print(code)?;

    let out_path = out_dir.join("schema.rs");
    write(&out_path, code)?;

    Ok(())
}

fn env_path(var: &str) -> PathBuf {
    let Ok(value) = std::env::var(var) else {
        panic!("Missing `{var}` environment variable!");
    };

    PathBuf::from(value)
}

fn rustfmt_pretty_print(code: String) -> Result<String, Error> {
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();

    write!(stdin, "{code}")?;
    stdin.flush()?;
    drop(stdin);

    let std::process::Output {
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
