//! This is a basic example for generating code from most XML schemas. It accepts
//! multiple input schema files and an output path for the generated code, making
//! it usable as a simple command-line tool.
//!
//! The output of this example is a Rust module containing types generated from
//! the XML schema. The default configuration used in this example does not enable
//! `serde` or `quick_xml` support, so only the type definitions are generated.
//! For an advanced example that includes deserialization code, see the
//! `update_schema` example.

#![allow(missing_docs)]

use std::env::current_dir;
use std::fs::{read_dir, write};
use std::path::PathBuf;
use std::thread::Builder;

use anyhow::Error;
use clap::Parser;
use num_format::{Locale, ToFormattedString};
use tracing_subscriber::{fmt, EnvFilter};
use xsd_parser::measure_stack_usage;
use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema},
    generate, Config,
};

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

    // Parse the command line arguments
    let args = Args::parse();
    tracing::info!("Run with arguments: {args:#?}");

    // Canonicalize all input files, to ensure that the files exists and that
    // the path is valid. Store it in a vector for further processing.
    let inputs = read_dir(current_dir()?.join("schema/XJustiz 3.4.1 XSD"))?
        .map(|p| p.map(|p| p.path()))
        .collect::<Result<Vec<_>, _>>()?;

    // Create a default configuration for the generator. This configuration is
    // crucial for code generation, as it directly influences the generated output.
    // It controls how the generator produces the desired code.
    // This example enables nearly all features provided by xsd-parser to demonstrate
    // the tool's full capabilities. Since this setup may not suit all users, a
    // detailed configuration is provided for customization.
    let mut config = Config::default();

    // Instruct the parser to use a file resolver for handling includes and imports
    // of locally stored files.
    config.parser.resolver = vec![Resolver::File];

    // Enable all parser flags. For details please refer to the flags documentation.
    config.parser.flags = ParserFlags::all();

    // Instructs the parser to load and parse the input files. The user can specify
    // various schema sources in the configuration. Here, a [`Schema::File`] source
    // is used. For more details, see the [`Schema`] documentation.
    config.parser.schemas = inputs.into_iter().map(Schema::File).collect();

    // Enable all interpreter flags. For details please refer to the flags documentation.
    config.interpreter.flags = InterpreterFlags::all();

    // Enable all optimizer flags, except `REMOVE_DUPLICATES` because it can cause
    // some problems in some schemas, so it is disabled by default. For details
    // please refer to the flags documentation.
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES;

    // Enable all generator flags. For details please refer to the flags documentation.
    config.generator.flags = GeneratorFlags::all();

    // Logs the configuration to display the values used during code generation.
    tracing::info!("Code generator uses the following config: {config:#?}");

    // Generates additional debug output for the various steps of the code generation process
    // if requested. This helps in tracking bugs or diagnosing issues with the tool.
    if let Some(out_dir) = args
        .enable_debug_output
        .then_some(())
        .and_then(|()| args.output.parent())
    {
        config.parser.debug_output = Some(out_dir.join("parser.log"));
        config.interpreter.debug_output = Some(out_dir.join("interpreter.log"));
        config.optimizer.debug_output = Some(out_dir.join("optimizer.log"));
    }

    // Executes the actual code generation process.
    let code = Builder::new()
        .stack_size(3 * 1024 * 1024)
        .spawn(move || {
            xsd_parser::print_stack_info!();

            let mut code = String::new();
            let used_stack = measure_stack_usage(|| {
                code = generate(config).unwrap().to_string();
            });

            tracing::info!(
                "Used stack: {} bytes",
                used_stack.to_formatted_string(&Locale::de)
            );

            code
        })?
        .join()
        .unwrap();

    // Writes the generated code to the requested output file.
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
