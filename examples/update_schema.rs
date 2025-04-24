#![allow(missing_docs)]

use std::env::{args, current_dir};
use std::fs::write;
use std::path::PathBuf;

use anyhow::Error;
use proc_macro2::TokenStream;
use quote::quote;
use tracing_subscriber::{fmt, EnvFilter};

use xsd_parser::config::{
    Config, Generate, IdentTriple, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema,
};
use xsd_parser::generate;
use xsd_parser::generator::GeneratorFlags;
use xsd_parser::types::{BuildInInfo, CustomType, IdentType, SimpleType, Type};

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

    let cwd = current_dir()?;

    let mut args = args().skip(1);
    let input = match args.next() {
        Some(input) => PathBuf::from(input),
        None => cwd.join("schema/XMLSchema.xsd").canonicalize()?,
    };
    let output = match args.next() {
        Some(output) => PathBuf::from(output),
        None => cwd.join("src/schema/xs_generated_new.rs"),
    };

    tracing::info!("Generate Code for {input:#?} to {output:#?}");

    let mut config = Config::default().with_quick_xml_serialize();

    // parser
    config.parser.flags = ParserFlags::all();
    config.parser.schemas = vec![Schema::File(input)];
    config.parser.resolver = vec![Resolver::File];

    // interpreter
    config.interpreter.flags = InterpreterFlags::all();
    config.interpreter.types = vec![
        (
            IdentTriple::from((IdentType::Type, "xs:allNNI")),
            Type::SimpleType(SimpleType::from(BuildInInfo::Custom(
                CustomType::new("MaxOccurs").with_default(max_occurs_default),
            ))),
        ),
        (
            IdentTriple::from((IdentType::Type, "xs:QName")),
            Type::SimpleType(SimpleType::from(BuildInInfo::Custom(CustomType::new(
                "QName",
            )))),
        ),
    ];

    // optimizer
    config.optimizer.flags = OptimizerFlags::all();

    // generator
    config.generator.flags = GeneratorFlags::all() - GeneratorFlags::USE_MODULES;
    config.generator.xsd_parser = "crate".into();
    config.generator.type_postfix.element = String::default();
    config.generator.type_postfix.element_type = String::default();
    config.generator.generate =
        Generate::Types(vec![IdentTriple::from((IdentType::Element, "xs:schema"))]);
    config.generator.derive = Some(
        ["Debug", "Clone", "Eq", "PartialEq"]
            .into_iter()
            .map(String::from)
            .collect(),
    );

    // debug
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
