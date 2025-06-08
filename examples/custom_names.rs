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

use std::fs::write;

use anyhow::{anyhow, Error};
use quote::ToTokens;
use xsd_parser::{
    config::{
        Config, GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema,
        SerdeSupport,
    },
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::{
        meta::{ElementMetaVariant, MetaTypeVariant, MetaTypes},
        IdentType,
    },
    Ident, Name, TypesPrinter,
};

fn main() -> Result<(), Error> {
    // Create the configuration that is used by the generation process. For more
    // details about the different values refer to the `simple` example or directly
    // to the documentation of the values.
    let mut config = Config::default();
    config.parser.resolver = vec![Resolver::File];
    config.parser.flags = ParserFlags::all();
    config.parser.schemas = vec![Schema::File("schema/test.xsd".into())];
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GeneratorFlags::all();
    config.generator.serde_support = SerdeSupport::QuickXml;

    // Execute the different steps of the code generation process. The `define_custom_names`
    // function defines a custom step inside the process to set custom names for specified types.
    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;

    let printer = TypesPrinter::new(&meta_types);
    let debug = format!("{printer}");
    write("target/interpreter.log", debug)?;

    let meta_types = rename_my_element_created_date(meta_types)?;
    let meta_types = change_my_other_element_created_date_type(meta_types)?;

    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;
    let code = module.to_token_stream().to_string();

    // Print the generated code to stdout.
    println!("{code}");

    Ok(())
}

/// Rename the `created_date` element inside the content of `myElement`.
fn rename_my_element_created_date(mut types: MetaTypes) -> Result<MetaTypes, Error> {
    let my_element_ident = Ident {
        ns: None,
        name: Name::generated("RootElementMyElement"),
        type_: IdentType::ElementType,
    };
    let my_element = types
        .get_resolved_type(&my_element_ident)
        .ok_or_else(|| anyhow!("Unable to get type for `RootElementMyElement`!"))?;

    let MetaTypeVariant::ComplexType(meta) = &my_element.variant else {
        return Err(anyhow!("`RootElementMyElement` is not a complex type!"));
    };
    let content_ident = meta
        .content
        .as_ref()
        .ok_or_else(|| anyhow!("`RootElementMyElement` does not have a content identifier!"))?
        .clone();

    let my_element_content = types
        .items
        .get_mut(&content_ident)
        .ok_or_else(|| anyhow!("Unable to get content type for `RootElementMyElement`!"))?;
    let MetaTypeVariant::Sequence(meta) = &mut my_element_content.variant else {
        return Err(anyhow!("`RootElementMyElement` content is not a sequence!"));
    };

    let element_meta = meta.elements.get_mut(0).ok_or_else(|| {
        anyhow!("Unable to get first element from `RootElementMyElement` content sequence!")
    })?;

    element_meta.display_name = Some("created_date_field".into()); // to set the name of the generated field
    element_meta.ident.name = Name::named("created_date_element"); // to set the name of the element inside the XML

    Ok(types)
}

/// Create a new `RecordDateType` and use it for the `created_date` element inside the content of `myOtherElement`.
fn change_my_other_element_created_date_type(mut types: MetaTypes) -> Result<MetaTypes, Error> {
    let record_date_ident = Ident {
        ns: None,
        name: Name::named("RecordDateType"),
        type_: IdentType::Type,
    };
    let new_record_date_ident = Ident {
        ns: None,
        name: Name::named("CustomRecordDateType"),
        type_: IdentType::Type,
    };
    let new_record_date = types
        .items
        .get(&record_date_ident)
        .ok_or_else(|| anyhow!("Unable to get type for `RecordDateType`!"))?
        .clone();
    types
        .items
        .insert(new_record_date_ident.clone(), new_record_date);

    let my_other_element_ident = Ident {
        ns: None,
        name: Name::generated("RootElementMyOtherElement"),
        type_: IdentType::ElementType,
    };
    let my_other_element = types
        .get_resolved_type(&my_other_element_ident)
        .ok_or_else(|| anyhow!("Unable to get type for `RootElementMyOtherElement`!"))?;

    let MetaTypeVariant::ComplexType(meta) = &my_other_element.variant else {
        return Err(anyhow!(
            "`RootElementMyOtherElement` is not a complex type!"
        ));
    };
    let content_ident = meta
        .content
        .as_ref()
        .ok_or_else(|| anyhow!("`RootElementMyOtherElement` does not have a content identifier!"))?
        .clone();

    let my_other_element_content = types
        .items
        .get_mut(&content_ident)
        .ok_or_else(|| anyhow!("Unable to get content type for `RootElementMyOtherElement`!"))?;
    let MetaTypeVariant::Sequence(meta) = &mut my_other_element_content.variant else {
        return Err(anyhow!(
            "`RootElementMyOtherElement` content is not a sequence!"
        ));
    };

    let element_meta = meta.elements.get_mut(0).ok_or_else(|| {
        anyhow!("Unable to get first element from `RootElementMyOtherElement` content sequence!")
    })?;
    element_meta.type_ = ElementMetaVariant::Type(new_record_date_ident);

    Ok(types)
}
