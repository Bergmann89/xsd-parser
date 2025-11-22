//! Short example that shows how to assign custom defined names to
//! enumeration variants. See `custom_names.rs` for more details.
//!
//! Simple schema used for this example:
//!
//! ```xml
//! <xs:schema targetNamespace="http://www.example.com/signtype"
//!   xmlns:xs="http://www.w3.org/2001/XMLSchema"
//!   xml:lang="en">
//!
//!   <simpleType name="SignType">
//!     <restriction base="xs:string">
//!         <enumeration value="-"/>
//!         <enumeration value="+"/>
//!     </restriction>
//!   </simpleType>
//!
//! </xs:schema>
//! ```

#![allow(missing_docs)]

use anyhow::Error;
use quote::ToTokens;
use xsd_parser::{
    config::{
        Config, GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema,
    },
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::meta::{MetaTypeVariant, MetaTypes},
};

fn main() -> Result<(), Error> {
    // Create the configuration that is used by the generation process. For more
    // details about the different values refer to the `simple` example or directly
    // to the documentation of the values.
    let mut config = Config::default();
    config.parser.resolver = vec![Resolver::File];
    config.parser.flags = ParserFlags::all();
    config.parser.schemas = vec![Schema::File("examples/restriction.xsd".into())];
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GeneratorFlags::all();

    // Execute the different steps of the code generation process. The `replace_variant_names`
    // function defines a custom step inside the process to set custom names for specified variants.
    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = replace_variant_names(meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;
    let code = module.to_token_stream().to_string();

    // Print the generated code to stdout.
    println!("{code}");

    Ok(())
}

/// Define custom names for specific variants. Plus and minus characters are invalid identifiers in Rust.
fn replace_variant_names(mut types: MetaTypes) -> MetaTypes {
    for ty in types.iter_types_mut() {
        if let MetaTypeVariant::Enumeration(enum_meta) = &mut ty.variant {
            for variant in enum_meta.variants.iter_mut() {
                match variant.ident.name.as_str() {
                    "+" => {
                        variant.display_name = Some("Plus".to_string());
                    }
                    "-" => {
                        variant.display_name = Some("Minus".to_string());
                    }
                    _ => {}
                }
            }
        }
    }

    types
}
