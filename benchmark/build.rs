#![allow(missing_docs)]

use std::fs::{create_dir_all, write};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use anyhow::{Context, Error};

use xsd_parser::{
    config::{
        GeneratorFlags, IdentQuadruple, InterpreterFlags, NamespaceIdent, OptimizerFlags, Schema,
        SerdeXmlRsVersion,
    },
    generate, Config, IdentType,
};

fn main() -> Result<(), Error> {
    let generator = Generator::new()?;

    /* XMLSchema.xsd */

    {
        let mut config = Config::default()
            .with_interpreter_flags(InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT)
            .with_optimizer_flags(OptimizerFlags::all() - OptimizerFlags::SIMPLIFY_MIXED_TYPES)
            .with_generator_flags(
                GeneratorFlags::all()
                    - GeneratorFlags::USE_MODULES
                    - GeneratorFlags::ANY_TYPE_SUPPORT
                    - GeneratorFlags::MIXED_TYPE_SUPPORT
                    - GeneratorFlags::NILLABLE_TYPE_SUPPORT
                    - GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS
                    - GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS,
            )
            .with_generate([IdentQuadruple::from((IdentType::ElementType, "xs:schema"))]);
        config.generator.type_postfix.element = String::default();
        config.generator.type_postfix.element_type = String::default();

        generator
            .generate(
                config.clone().with_quick_xml(),
                ["../xsd-parser/schema/XMLSchema.xsd"],
                "xs_quick_xml.rs",
            )
            .context("Unable to generate `xs_quick_xml.rs`")?;
        generator
            .generate(
                config
                    .with_quick_xml_serialize()
                    .with_quick_xml_deserialize_config(true),
                ["../xsd-parser/schema/XMLSchema.xsd"],
                "xs_quick_xml_boxed.rs",
            )
            .context("Unable to generate `xs_quick_xml_boxed.rs`")?;
    }

    /* iDEAL 3.3.1 Merchant–Acquirer */

    {
        let mut config = Config::default()
            .with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow)
            .with_generate([(
                IdentType::Element,
                Some(NamespaceIdent::namespace(
                    b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1",
                )),
                "DirectoryReq",
            )]);
        config.optimizer.flags |= OptimizerFlags::RESOLVE_TYPEDEFS;
        config.optimizer.flags |= OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE_SIMPLE;
        config.generator.type_postfix.element_type = "Type".into();
        config.generator.flags |= GeneratorFlags::FLATTEN_STRUCT_CONTENT;

        generator
            .generate(
                config.clone().with_quick_xml(),
                ["../xsd-parser/tests/schema/ideal_merchant_acquirer/schema.xsd"],
                "ideal_merchant_acquirer_quick_xml.rs",
            )
            .context("Unable to generate `ideal_merchant_acquirer_quick_xml.rs`")?;
        generator
            .generate(
                config
                    .clone()
                    .with_quick_xml_serialize()
                    .with_quick_xml_deserialize_config(true),
                ["../xsd-parser/tests/schema/ideal_merchant_acquirer/schema.xsd"],
                "ideal_merchant_acquirer_quick_xml_boxed.rs",
            )
            .context("Unable to generate `ideal_merchant_acquirer_quick_xml_boxed.rs`")?;
        generator
            .generate(
                config
                    .clone()
                    .with_serde_xml_rs(SerdeXmlRsVersion::Version07AndBelow),
                ["../xsd-parser/tests/schema/ideal_merchant_acquirer/schema.xsd"],
                "ideal_merchant_acquirer_serde_xml_rs_v7.rs",
            )
            .context("Unable to generate `ideal_merchant_acquirer_serde_xml_rs_v7.rs`")?;
        generator
            .generate(
                config.clone().with_serde_quick_xml(),
                ["../xsd-parser/tests/schema/ideal_merchant_acquirer/schema.xsd"],
                "ideal_merchant_acquirer_serde_quick_xml.rs",
            )
            .context("Unable to generate `ideal_merchant_acquirer_serde_quick_xml.rs`")?;
    }

    Ok(())
}

struct Generator {
    out_dir: PathBuf,
    cargo_dir: PathBuf,
}

impl Generator {
    fn new() -> Result<Self, Error> {
        let out_dir = env_path("OUT_DIR");
        let cargo_dir = env_path("CARGO_MANIFEST_DIR")
            .canonicalize()
            .context("Unable to canonicalizing path `CARGO_MANIFEST_DIR`")?;

        create_dir_all(&out_dir).context("Unable to create `OUT_DIR`")?;
        let out_dir = out_dir
            .canonicalize()
            .context("Unable to canonicalizing path `OUT_DIR`")?;

        Ok(Self { out_dir, cargo_dir })
    }

    fn generate<I, O>(&self, mut config: Config, input: I, output: O) -> Result<(), Error>
    where
        I: IntoIterator,
        I::Item: AsRef<Path>,
        O: AsRef<Path>,
    {
        let input = input
            .into_iter()
            .map(|input| {
                let input = self.cargo_dir.join(input).canonicalize()?;

                println!("cargo:rerun-if-changed={}", input.display());

                Ok(Schema::File(input))
            })
            .collect::<Result<Vec<_>, Error>>()
            .context("Unable to canonicalize input files")?;
        config.parser.schemas.extend(input);

        let code = generate(config).context("Error while generating code")?;
        let code = code.to_string();
        let code = rustfmt_pretty_print(&code)
            .context("Error while formatting generated code using `rustfmt`")?;

        let output = self.out_dir.join(output);

        write(&output, code)
            .with_context(|| format!("Unable to write generate code to `{}`", output.display()))?;

        Ok(())
    }
}

fn env_path(var: &str) -> PathBuf {
    let Ok(value) = std::env::var(var) else {
        panic!("Missing `{var}` environment variable!");
    };

    PathBuf::from(value)
}

fn rustfmt_pretty_print(code: &str) -> Result<String, Error> {
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
                assert!(
                    code != 0,
                    "The `rustfmt` command failed with return code {code}!\n{stderr}"
                );
            }
            None => {
                panic!("The `rustfmt` command failed!\n{stderr}")
            }
        }
    }

    Ok(stdout.into())
}
