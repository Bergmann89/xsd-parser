//! Short example that shows how to assign custom defined names to the
//! generated types.
//!
//! This example is more or less based on the `simple` example and shows how the
//! name of the different generated types can be changed to match the expectations
//! of the user.
//!
//! It will also show the more advanced technique to use the [`exec_parser`],
//! [`exec_interpreter`], [`exec_optimizer`] and [`exec_generator_module`]
//! function directly, instead of relying on the more simpler but also more
//! restricted [`generate`] function.

#![allow(missing_docs)]

use anyhow::{anyhow, Error};
use quote::ToTokens;
use xsd_parser::{
    config::{
        Config, GeneratorFlags, IdentTriple, InterpreterFlags, OptimizerFlags, ParserFlags,
        Resolver, Schema,
    },
    exec_generator_module, exec_interpreter, exec_optimizer, exec_parser,
    models::{meta::MetaTypes, schema::Schemas, IdentType},
};

fn main() -> Result<(), Error> {
    // Create the configuration that is used by the generation process. For more
    // details about the different values refer to the `simple` example or directly
    // to the documentation of the values.
    let mut config = Config::default();
    config.parser.resolver = vec![Resolver::File];
    config.parser.flags = ParserFlags::all();
    config.parser.schemas = vec![Schema::File("schema/XMLSchema.xsd".into())];
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GeneratorFlags::all();

    // Execute the different steps of the code generation process. The `define_custom_names`
    // function defines a custom step inside the process to set custom names for specified types.
    let schemas = exec_parser(config.parser)?;
    let types = exec_interpreter(config.interpreter, &schemas)?;
    let types = define_custom_names(&schemas, types)?;
    let types = exec_optimizer(config.optimizer, types)?;
    let module = exec_generator_module(config.generator, &schemas, &types)?;
    let code = module.to_token_stream().to_string();

    // Print the generated code to stdout.
    println!("{code}");

    Ok(())
}

/// Define custom names for specific types.
fn define_custom_names(schemas: &Schemas, mut types: MetaTypes) -> Result<MetaTypes, Error> {
    // Types are identified by a triple of namespace, element type and name, which are
    // combined into `IdentTriple`.
    let ident = IdentTriple::from((IdentType::Element, "xs:schema"));

    // There are multiple ways to define a `IdentTriple`. Each of the following
    // will resolve to the same `Ident`.
    //
    //      let ident = IdentTriple::from((
    //          IdentType::Element,
    //          Some(NamespaceIdent::prefix(b"xs")),
    //          "schema",
    //      ));
    //
    //      let ident = IdentTriple::from((
    //          IdentType::Element,
    //          Some(NamespaceIdent::namespace(b"http://www.w3.org/2001/XMLSchema")),
    //          "schema",
    //      ));
    //
    //      let ident = IdentTriple {
    //          ns: Some(NamespaceIdent::prefix(b"xs")),
    //          name: "schema".into(),
    //          type_: IdentType::Element,
    //      };

    // Namespaces are internally represented by ids (for performance reasons), so
    // the information from the `IdentTriple` must be resolved to an actual `Ident`
    // by using the current `Schemas` information.
    let ident = ident.resolve(schemas)?;

    // `Types` is more or less a map of `Ident` to `Type`, so we can use `get_mut`,
    // to get a mutable reference to the type we want to rename.
    let ty = types
        .items
        .get_mut(&ident)
        .ok_or_else(|| anyhow!("Unable to find `xs:schema` element in the types map!"))?;

    // Simply set the `display_name` to something that fits more.
    ty.display_name = Some("MyCustomSchema".into());

    Ok(types)
}
