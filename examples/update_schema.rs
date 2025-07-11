//! This example is used to generate and update the XSD schema used in
//! `xsd-parser` (see `xs_generated.rs`).
//!
//! It demonstrates a more advanced use of the generator to create types from
//! the schema with deserialization support enabled.

#![allow(missing_docs)]

use std::env::{args, current_dir};
use std::fs::write;
use std::path::PathBuf;

use anyhow::Error;
use proc_macro2::TokenStream;
use quote::quote;
use tracing_subscriber::{fmt, EnvFilter};

use xsd_parser::config::GeneratorFlags;
use xsd_parser::config::{
    Config, Generate, IdentTriple, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema,
};
use xsd_parser::models::meta::{CustomMeta, MetaType};
use xsd_parser::{generate, IdentType};

fn main() -> Result<(), Error> {
    // Initialize the logging framework. Log output can be controlled using the
    // `RUST_LOG` environment variable.
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

    let cwd = current_dir()?;
    let mut args = args().skip(1);

    // Use the first command line parameter as input for the schema. If it is not set
    // we will fall back to `schema/XMLSchema.xsd`.
    let input = match args.next() {
        Some(input) => PathBuf::from(input),
        None => cwd.join("schema/XMLSchema.xsd").canonicalize()?,
    };

    // Uses the first command-line parameter as the schema input. If not provided,
    // it defaults to schema/XMLSchema.xsd.
    let output = match args.next() {
        Some(output) => PathBuf::from(output),
        None => cwd.join("src/schema/xs_generated_new.rs"),
    };

    tracing::info!("Generate Code for {input:#?} to {output:#?}");

    // Creates the default configuration and enables code generation for
    // `quick_xml` deserialization.
    let mut config = Config::default().with_quick_xml_deserialize_config(true);

    // Enables all parser flags (see the flags documentation for details), sets
    // the input file, and activates file resolvers to handle imports and includes.
    config.parser.flags = ParserFlags::all();
    config.parser.schemas = vec![Schema::File(input)];
    config.parser.resolver = vec![Resolver::File];

    // Enables all interpreter flags (refer to the flags documentation for details)
    // and supplies custom type definitions for `xs:allNNI` (using the `MaxOccurs` type),
    // `xs:QName` (using the `QName` type) and `xs:appinfo` as well as `xs:documentation`
    // (using the `AnyElement` type).
    // Using the AnyElement type is useful to store unstructured data for later usage.
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.interpreter.types = vec![
        (
            IdentTriple::from((IdentType::Type, "xs:allNNI")),
            MetaType::from(CustomMeta::new("MaxOccurs").with_default(max_occurs_default)),
        ),
        (
            IdentTriple::from((IdentType::Type, "xs:QName")),
            MetaType::from(CustomMeta::new("QName")),
        ),
        (
            IdentTriple::from((IdentType::Element, "xs:appinfo")),
            MetaType::from(CustomMeta::new("AnyElement")),
        ),
        (
            IdentTriple::from((IdentType::Element, "xs:documentation")),
            MetaType::from(CustomMeta::new("AnyElement")),
        ),
    ];

    // Enables all optimizer flags (refer to the flags documentation for details).
    config.optimizer.flags = OptimizerFlags::all();

    // The generator configuration sets the following:
    //   -  Enables all generator flags except USE_MODULES to avoid separate modules
    //      for different schemas (like XSD and XML).
    //   -  Sets the module name to crate instead of xsd_parser to support includes
    //      of other types.
    //   -  Disables name postfixing for elements and element types, as the schema
    //      already uses distinct names.
    //   -  Instructs the generator to produce code for the xs:schema element and
    //      its dependent types.
    //   -  Derives the generated types from `Debug`, `Clone`, `Eq`, and `PartialEq`.
    config.generator.flags =
        GeneratorFlags::all() - GeneratorFlags::USE_MODULES - GeneratorFlags::MIXED_TYPE_SUPPORT;
    config.generator.type_postfix.element = String::default();
    config.generator.type_postfix.element_type = String::default();
    config.generator.generate =
        Generate::Types(vec![IdentTriple::from((IdentType::Element, "xs:schema"))]);
    config.renderer.xsd_parser = "crate".into();
    config.renderer.derive = Some(
        ["Debug", "Clone", "Eq", "PartialEq"]
            .into_iter()
            .map(String::from)
            .collect(),
    );

    // Enable additional debug output (if needed).
    // config.parser.debug_output = Some(PathBuf::from("./schemas.log"));
    // config.interpreter.debug_output = Some(PathBuf::from("./interpreter.log"));
    // config.optimizer.debug_output = Some(PathBuf::from("./optimizer.log"));

    let code = generate(config)?;

    tracing::info!("Write Code");
    write(output, code.to_string())?;

    Ok(())
}

fn max_occurs_default(s: &str) -> Option<TokenStream> {
    if s == "unbound" {
        return Some(quote!(MaxOccurs::Unbounded));
    }

    let val = s.parse::<usize>().ok()?;

    Some(quote!(MaxOccurs::Bounded(#val)))
}
