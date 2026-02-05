use std::fs::remove_dir_all;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use anyhow::{Context, Error};
use inflector::Inflector;
use xsd_parser::{
    Config, Ident2, IdentType, Name, TypeIdent, config::{GeneratorFlags, InterpreterFlags, OptimizerFlags}, exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render, models::{ExplicitNameBuilder, format_ident, format_unknown_variant, meta::MetaType}, traits::{NameBuilder, Naming}
};

fn main() -> Result<(), Error> {
    let cargo_dir = env_path("CARGO_MANIFEST_DIR")
        .canonicalize()
        .context("Missing environment variable `CARGO_MANIFEST_DIR`")?;
    let schema_files = [
        "schema/xsd/NeTEx_publication.xsd",
        "schema/xsd/NeTEx_siri.xsd",
    ]
    .iter()
    .map(|path| {
        cargo_dir
            .join(path)
            .canonicalize()
            .context("Missing or invalid schema file!")
    })
    .collect::<Result<Vec<PathBuf>, Error>>()?;

    for schema_file in &schema_files {
        println!("cargo:rerun-if-changed={}", schema_file.display());
    }

    let config = Config::default()
        .with_schemas(schema_files)
        .with_naming(CustomNaming::default())
        .set_interpreter_flags(InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT)
        .set_optimizer_flags(OptimizerFlags::all() /* - OptimizerFlags::CONVERT_DYNAMIC_TO_CHOICE TODO */ )
        .set_generator_flags(GeneratorFlags::all())
        .with_quick_xml()
        .with_generate([(IdentType::Element, "netex:PublicationDelivery")]);

    // Generate the code based on the configuration above.
    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;

    let target_dir = cargo_dir.join("src/netex");
    remove_dir_all(&target_dir)?;
    make_dir_all(&target_dir)?;
    module.write_to_files(&target_dir)?;

    Ok(())
}

#[derive(Default, Debug)]
struct CustomNaming(Arc<AtomicUsize>);

impl Naming for CustomNaming {
    fn clone_boxed(&self) -> Box<dyn Naming> {
        Box::new(Self(self.0.clone()))
    }

    fn builder(&self) -> Box<dyn NameBuilder> {
        Box::new(ExplicitNameBuilder::new(self.0.clone(), self.clone_boxed()))
    }

    fn make_type_name(&self, postfixes: &[String], ty: &MetaType, ident: &TypeIdent) -> Name {
        let _ty = ty;
        let postfix = postfixes.get(ident.type_ as usize).map(String::as_str);

        let s = self.format_type_name(ident.name.as_str());

        Name::new_generated(if let Some(postfix) = postfix {
            format!("{s}{postfix}")
        } else {
            s
        })
    }

    fn make_unknown_variant(&self, id: usize) -> Ident2 {
        format_unknown_variant(id)
    }

    fn unify(&self, s: &str) -> String {
        s.to_owned()
    }

    fn format_module_name(&self, s: &str) -> String {
        let s = s.to_snake_case();

        format_ident(s)
    }

    fn format_type_name(&self, s: &str) -> String {
        let s = keep_trailing_underscores(s, |s| {
            s.to_screaming_snake_case().to_pascal_case()
        });

        format_ident(s)
    }

    fn format_field_name(&self, s: &str) -> String {
        match s {
            "parkingProperties" => "parking_properties_rel".to_string(),
            _ => {
                let s = keep_trailing_underscores(s, |s| {
                    s.to_snake_case()
                });

                format_ident(s)
            }
        }
    }

    fn format_variant_name(&self, s: &str) -> String {
        match s {
            "+" => "Plus".to_string(),
            "-" => "Minus".to_string(),
            _ => format_ident(s),
        }
    }
}

fn keep_trailing_underscores<F>(s: &str, f: F) -> String
where
    F: Fn(&str) -> String,
{
    let a = s.chars().rev().take_while(|c| *c == '_').count();
    let s = f(s);
    let b = s.chars().rev().take_while(|c| *c == '_').count();

    if b < a {
        s + &"_".repeat(a - b)
    } else {
        s
    }
}

fn env_path(var: &str) -> PathBuf {
    let Ok(value) = std::env::var(var) else {
        panic!("Missing `{var}` environment variable!");
    };

    PathBuf::from(value)
}
