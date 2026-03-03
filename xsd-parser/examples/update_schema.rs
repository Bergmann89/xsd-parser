//! This example is used to generate and update the XSD schema used in
//! `xsd-parser` (see `xs_generated.rs`).
//!
//! It demonstrates a more advanced use of the generator to create types from
//! the schema with deserialization support enabled.

#![allow(missing_docs)]

use std::borrow::Cow;
use std::env::{args, current_dir};
use std::fs::write;
use std::mem::swap;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Context, Error};
use quote::{format_ident, quote, ToTokens};
use tracing_subscriber::{fmt, EnvFilter};

use xsd_parser::config::{
    Config, Generate, GeneratorFlags, IdentQuadruple, InterpreterFlags, OptimizerFlags,
    ParserFlags, Resolver, Schema, TypedefMode,
};
use xsd_parser::models::code::IdentPath;
use xsd_parser::models::data::{
    ComplexData, ConfigValue, DataType, DataTypeVariant, Occurs, PathData, ReferenceData,
};
use xsd_parser::models::meta::{CustomMeta, MetaType, ReferenceMeta};
use xsd_parser::models::schema::MaxOccurs;
use xsd_parser::pipeline::generator::{
    Context as GeneratorContext, Error as GeneratorError, ValueGeneratorMode,
};
use xsd_parser::pipeline::renderer::ValueRendererBox;
use xsd_parser::{
    exec_generator_with_ident_cache, exec_interpreter_with_ident_cache, exec_optimizer,
    exec_parser, exec_render, DataTypes, IdentType, Name, Schemas, TypeIdent,
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
        None => cwd.join("src/models/schema/xs_generated_new.rs"),
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
            IdentQuadruple::from((IdentType::Type, "xs:allNNI")),
            MetaType::from(
                CustomMeta::new("MaxOccurs")
                    .include_from("crate::models::schema::MaxOccurs")
                    .with_default(max_occurs_default),
            ),
        ),
        (
            IdentQuadruple::from((IdentType::Type, "xs:QName")),
            MetaType::from(CustomMeta::new("QName").include_from("crate::models::schema::QName")),
        ),
        (
            IdentQuadruple::from((IdentType::Element, "xs:appinfo")),
            MetaType::from(
                CustomMeta::new("AnyElement").include_from("xsd_parser_types::xml::AnyElement"),
            ),
        ),
        (
            IdentQuadruple::from((IdentType::Element, "xs:documentation")),
            MetaType::from(
                CustomMeta::new("AnyElement").include_from("xsd_parser_types::xml::AnyElement"),
            ),
        ),
    ];

    // Enables all optimizer flags (refer to the flags documentation for details).
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::SIMPLIFY_MIXED_TYPES;

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
    config.generator.flags = GeneratorFlags::all()
        - GeneratorFlags::USE_MODULES
        - GeneratorFlags::ANY_TYPE_SUPPORT
        - GeneratorFlags::MIXED_TYPE_SUPPORT
        - GeneratorFlags::NILLABLE_TYPE_SUPPORT
        - GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS
        - GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS;
    config.generator.type_postfix.element = String::default();
    config.generator.type_postfix.element_type = String::default();
    config.generator.generate = Generate::Types(vec![IdentQuadruple::from((
        IdentType::ElementType,
        "xs:schema",
    ))]);
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

    let schemas = exec_parser(config.parser)?;
    let (meta_types, ident_cache) =
        exec_interpreter_with_ident_cache(config.interpreter, &schemas)?;
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator_with_ident_cache(
        config.generator,
        &schemas,
        Some(&ident_cache),
        &meta_types,
    )?;
    let data_types = patch_data_types(&schemas, data_types)?;
    let module = exec_render(config.renderer, &data_types)?;
    let code = module.to_token_stream().to_string();

    tracing::info!("Write Code");
    write(output, &code)?;

    Ok(())
}

fn patch_data_types<'types>(
    schemas: &Schemas,
    mut types: DataTypes<'types>,
) -> Result<DataTypes<'types>, Error> {
    /* xs:attribute */
    let ident = IdentQuadruple::from((IdentType::Type, "xs:attribute"));
    let ident = ident.resolve(schemas)?;

    wrap_struct(&mut types, ident, "Attribute", "AttributeInner")?;

    /* xs:facet */
    let ident = IdentQuadruple::from((IdentType::Type, "xs:facet"));
    let ident = ident.resolve(schemas)?;

    wrap_struct(&mut types, ident, "Facet", "FacetInner")?;

    Ok(types)
}

fn wrap_struct(
    types: &mut DataTypes<'_>,
    ident: TypeIdent,
    outer: &str,
    inner: &str,
) -> Result<(), Error> {
    let index = types
        .items
        .get_index_of(&ident)
        .context("Unable to get type for xs:attribute")?;
    let inner_item = types.items.get_index_mut(index).unwrap().1;
    let DataTypeVariant::Complex(inner_type) = &mut inner_item.variant else {
        anyhow::bail!("Expected {outer} to be a complex type");
    };
    let ComplexData::Struct {
        type_: inner_struct,
        content_type: _,
    } = inner_type
    else {
        anyhow::bail!("Expected {outer} to be a struct");
    };
    inner_struct.base.type_ident = format_ident!("{inner}Type");
    inner_struct.base.serializer_ident = format_ident!("{inner}TypeSerializer");
    inner_struct.base.serializer_state_ident = format_ident!("{inner}TypeSerializerState");
    inner_struct.base.deserializer_ident = format_ident!("{inner}TypeDeserializer");
    inner_struct.base.deserializer_state_ident = format_ident!("{inner}TypeDeserializerState");

    let mut outer_item = DataType {
        meta: inner_item.meta.clone(),
        derive: ConfigValue::Default,
        variant: DataTypeVariant::Reference(ReferenceData {
            meta: Cow::Owned(ReferenceMeta {
                type_: ident.clone(),
                nillable: false,
                min_occurs: 1,
                max_occurs: MaxOccurs::Bounded(1),
            }),
            mode: TypedefMode::Typedef,
            occurs: Occurs::Single,
            type_ident: format_ident!("{outer}Type"),
            target_type: PathData::from_path(IdentPath::from_str("NamespaceScope").unwrap())
                .with_using("xsd_parser_types::xml::NamespaceScope")
                .with_generic(IdentPath::from_ident(format_ident!("{inner}Type"))),
            trait_impls: Vec::new(),
        }),
        extra_attributes: Vec::new(),
    };

    swap(inner_item, &mut outer_item);

    let item = outer_item;
    types.items.insert_before(
        index + 1,
        ident.with_name(Name::new_named(format!("{outer}Type"))),
        item,
    );

    Ok(())
}

fn max_occurs_default(
    ctx: &GeneratorContext<'_, '_>,
    value: &str,
    mode: ValueGeneratorMode,
) -> Result<ValueRendererBox, GeneratorError> {
    if value == "unbound" {
        return Ok(Box::new(quote!(MaxOccurs::Unbounded)));
    }

    let val = value
        .parse::<usize>()
        .map_err(|_| GeneratorError::InvalidDefaultValue {
            ident: ctx.ident.clone(),
            value: value.into(),
            mode,
        })?;

    Ok(Box::new(quote!(MaxOccurs::Bounded(#val))))
}
