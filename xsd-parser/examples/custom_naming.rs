//! Example that shows how to implement a custom naming strategy using
//! [`Config::with_naming`] with a custom [`Naming`] implementation.
//!
//! The default naming strategy normalizes names into `PascalCase`, which means an
//! XSD type like `String..1` can collapse into the same Rust type name as
//! `String1`. This example keeps separators during `unify`, so both schema
//! names stay distinct while the final identifier formatting is still delegated
//! to the library helpers.

#![allow(missing_docs)]

use std::sync::{atomic::AtomicUsize, Arc};

use anyhow::{ensure, Error};
use inflector::Inflector;
use xsd_parser::{
    config::Schema,
    generate,
    models::{
        format_ident, format_unknown_variant, make_type_name, meta::MetaType,
        NameBuilder as DefaultNameBuilder,
    },
    traits::{NameBuilder, Naming},
    Config, Ident2, IdentType, Name, TypeIdent,
};

fn main() -> Result<(), Error> {
    let mut config = Config::default();
    config.parser.schemas = vec![Schema::named_schema(
        "custom-naming.xsd",
        include_str!("custom-naming.xsd"),
    )];
    config = config
        .with_naming(PreserveSeparatorsNaming::default())
        .with_generate([(IdentType::Type, "Document")]);

    let code = generate(config)?.to_string();

    ensure!(
        code.contains("String__1Type"),
        "expected generated code to keep `String..1` distinct"
    );
    ensure!(
        code.contains("String1Type"),
        "expected generated code to keep `String1` available as its own type"
    );
    ensure!(
        code.contains("type_"),
        "expected keyword field names to still be formatted into valid Rust identifiers"
    );

    println!("{code}");

    Ok(())
}

#[derive(Debug, Clone, Default)]
struct PreserveSeparatorsNaming(Arc<AtomicUsize>);

impl Naming for PreserveSeparatorsNaming {
    fn clone_boxed(&self) -> Box<dyn Naming> {
        Box::new(self.clone())
    }

    fn builder(&self) -> Box<dyn NameBuilder> {
        Box::new(DefaultNameBuilder::new(
            self.0.clone(),
            Box::new(self.clone()),
        ))
    }

    fn unify(&self, s: &str) -> String {
        s.chars()
            .map(|ch| if ch.is_alphanumeric() { ch } else { '_' })
            .collect()
    }

    fn make_type_name(&self, postfixes: &[String], ty: &MetaType, ident: &TypeIdent) -> Name {
        make_type_name(self, postfixes, ty, ident)
    }

    fn make_unknown_variant(&self, id: usize) -> Ident2 {
        format_unknown_variant(id)
    }

    fn format_module_name(&self, s: &str) -> String {
        format_ident(self.unify(s).to_snake_case())
    }

    fn format_type_name(&self, s: &str) -> String {
        format_ident(self.unify(s))
    }

    fn format_field_name(&self, s: &str) -> String {
        format_ident(self.unify(s).to_snake_case())
    }

    fn format_variant_name(&self, s: &str) -> String {
        format_ident(self.unify(s))
    }

    fn format_constant_name(&self, s: &str) -> String {
        format_ident(self.unify(s).to_screaming_snake_case())
    }
}
