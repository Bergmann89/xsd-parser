use std::fs::File;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, Schema},
    generate, Config, Error,
};

fn main() -> Result<(), Error> {
    // This is almost the starting point defined in the main `[README.md]`.
    let config = Config::default()
        .with_schema(Schema::File("my-schema.xsd".into()))
        .with_interpreter_flags(InterpreterFlags::all())
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::all())
        .with_quick_xml();

    // Generate the code based on the configuration above.
    let code = generate(config)?;
    let code = code.to_string();

    // Use a small helper to pretty-print the code (it uses `RUSTFMT`).
    // Actually, this is easier to use, if one has to compare the result of
    // 2 versions of `my-schema.xsd`.
    let code = rustfmt_pretty_print(code).unwrap();

    // Generate my_schema.rs, containing all structures and implementations defined from
    // `my-schema.xsd` and the configuration above.
    let mut file = File::create("src/my_schema.rs")?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}

// A small helper to call `rustfmt` when generating file(s).
// This may be useful to compare different versions of generated files.
pub fn rustfmt_pretty_print(code: String) -> Result<String, Error> {
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
