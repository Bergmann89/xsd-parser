#![allow(missing_docs)]

use std::fs::write;
use std::path::PathBuf;

use anyhow::Error;
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};
use xsd_parser::{
    config::{GenerateFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema},
    generate, Config,
};

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

    let args = Args::parse();
    tracing::info!("Run with arguments: {args:#?}");

    let inputs = args
        .inputs
        .into_iter()
        .map(|p| p.canonicalize())
        .collect::<Result<Vec<_>, _>>()?;

    let mut config = Config::default();
    config.parser.resolver = vec![Resolver::File];
    config.parser.flags = ParserFlags::all();
    config.parser.schemas = inputs.into_iter().map(Schema::File).collect();
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GenerateFlags::all();

    if let Some(out_dir) = args
        .enable_debug_output
        .then_some(())
        .and_then(|()| args.output.parent())
    {
        config.parser.debug_output = Some(out_dir.join("parser.log"));
        config.interpreter.debug_output = Some(out_dir.join("interpreter.log"));
        config.optimizer.debug_output = Some(out_dir.join("optimizer.log"));
    }

    let code = generate(config)?.to_string();

    write(&args.output, code)?;

    Ok(())
}

/// Simple command line tool to generate code out of any XML schema that is
/// passed as input argument.
#[derive(Debug, Parser)]
struct Args {
    /// Write additional debut output in the output directory.
    #[arg(short, long)]
    enable_debug_output: bool,

    /// Path to write the generated code to.
    #[arg()]
    output: PathBuf,

    /// Paths to read the schema files from.
    #[arg()]
    inputs: Vec<PathBuf>,
}
