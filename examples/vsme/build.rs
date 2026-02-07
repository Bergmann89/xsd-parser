//! This is a build script to generate the code for the `vsme` schema.

use std::env::var;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;

use anyhow::{Context, Error};

use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, Schema},
    generate_modules,
    models::{meta::MetaType, Naming},
    traits::{NameBuilder as NameBuilderTrait, Naming as NamingTrait},
    Config, Name, TypeIdent,
};

fn main() -> Result<(), Error> {
    let cargo_dir =
        var("CARGO_MANIFEST_DIR").context("Missing `CARGO_MANIFEST_DIR` environment variable!")?;
    let cargo_dir = PathBuf::from(cargo_dir)
        .canonicalize()
        .context("Missing environment variable `CARGO_MANIFEST_DIR`")?;
    let schema_file = cargo_dir
        .join("schema/xbrl.efrag.org/taxonomy/vsme/2025-07-30/vsme-all.xsd")
        .canonicalize()
        .context("Missing or invalid schema file!")?;

    // This is almost the starting point defined in the main `[README.md]`.
    let config = Config::default()
        .with_schema(Schema::File(schema_file))
        .with_interpreter_flags(InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT)
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::all())
        .with_naming(CustomNaming::default())
        .with_quick_xml();

    // Generate the code based on the configuration above.
    let modules = generate_modules(config)?;

    // Write the generated code to the module directory specified by Cargo.
    let target_dir = cargo_dir.join("src/schema");
    let _ = remove_dir_all(&target_dir);
    create_dir_all(&target_dir).context("Unable to create `src/schema` directory!")?;
    modules
        .write_to_files(&target_dir)
        .context("Error while writing generated code")?;

    Ok(())
}

#[derive(Debug, Default)]
struct CustomNaming(Naming);

impl NamingTrait for CustomNaming {
    fn clone_boxed(&self) -> Box<dyn NamingTrait> {
        Box::new(Self(self.0.clone()))
    }

    fn builder(&self) -> Box<dyn NameBuilderTrait> {
        self.0.builder()
    }

    fn unify(&self, s: &str) -> String {
        self.0.unify(s)
    }

    fn make_type_name(&self, postfixes: &[String], ty: &MetaType, ident: &TypeIdent) -> Name {
        self.0.make_type_name(postfixes, ty, ident)
    }

    fn make_unknown_variant(&self, id: usize) -> xsd_parser::Ident2 {
        self.0.make_unknown_variant(id)
    }

    fn format_module_name(&self, s: &str) -> String {
        self.0.format_module_name(s)
    }

    fn format_type_name(&self, s: &str) -> String {
        self.0.format_type_name(s)
    }

    fn format_field_name(&self, s: &str) -> String {
        self.0.format_field_name(s)
    }

    fn format_variant_name(&self, s: &str) -> String {
        self.0.format_variant_name(s)
    }

    fn format_attribute_field_name(&self, s: &str) -> String {
        let s = self.0.format_attribute_field_name(s);
        let s = s.trim_end_matches('_');

        format!("{s}_attr")
    }
}
