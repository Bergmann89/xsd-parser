use std::fs::File;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use quote::ToTokens;
use xsd_parser::models::meta::{
    AnyAttributeMeta, AttributeMeta, AttributeMetaVariant, MetaTypeVariant,
};
use xsd_parser::models::schema::xs::ProcessContentsType;
use xsd_parser::{
    Config, Error,
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, RenderStep, Schema},
};
use xsd_parser::{
    Ident, MetaTypes, exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
};

fn main() -> Result<(), Error> {
    let mut config = Config::default();
    config.parser.schemas = vec![Schema::File("schema.xsd".into())];
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.optimizer.flags = OptimizerFlags::all();
    config.generator.flags = GeneratorFlags::all();

    let config = config.with_render_steps([
        RenderStep::Types,
        RenderStep::Defaults,
        RenderStep::NamespaceConstants,
        RenderStep::QuickXmlDeserialize {
            boxed_deserializer: false,
        },
        RenderStep::QuickXmlSerialize {
            with_namespaces: true,
            default_namespace: None,
        },
    ]);

    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let meta_types = fix_any_attributes(meta_types);
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;

    let code = module.to_token_stream().to_string();
    let code = rustfmt_pretty_print(code).unwrap();

    let mut file = File::create("src/schema.rs")?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}

fn fix_any_attributes(mut types: MetaTypes) -> MetaTypes {
    for ty in types.items.values_mut() {
        let MetaTypeVariant::ComplexType(ci) = &mut ty.variant else {
            continue;
        };
        let has_any = ci
            .attributes
            .iter()
            .any(|attrib| matches!(&attrib.variant, AttributeMetaVariant::Any(_)));
        if !has_any {
            let any = AnyAttributeMeta {
                id: None,
                namespace: None,
                not_q_name: None,
                not_namespace: None,
                process_contents: ProcessContentsType::Lax,
            };
            let any_attrib = AttributeMeta::any(Ident::name("any-attribute"), any);
            ci.attributes.push(any_attrib);
        }
    }

    types
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
