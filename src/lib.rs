#![recursion_limit = "256"]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/README.md"))]

pub mod config;
pub mod models;
pub mod pipeline;
pub mod quick_xml;
pub mod traits;
pub mod xml;

mod macros;
mod meta_types_printer;

/// Type alias for [`pipeline::renderer::Error`].
pub type RendererError = self::pipeline::renderer::Error;

/// Type alias for [`pipeline::generator::Error`].
pub type GeneratorError = self::pipeline::generator::Error;

/// Type alias for [`pipeline::interpreter::Error`].
pub type InterpreterError = self::pipeline::interpreter::Error;

/// Type alias for [`pipeline::parser::Error`].
pub type ParserError<E> = self::pipeline::parser::Error<E>;

use std::fmt::Debug;
use std::fs::write;
use std::io::Error as IoError;

pub use self::config::Config;
pub use self::meta_types_printer::MetaTypesPrinter;
pub use self::models::{
    code::{Module, SubModules},
    data::DataTypes,
    meta::MetaTypes,
    schema::Schemas,
    Ident, IdentType, Name,
};
pub use self::pipeline::{
    generator::Generator,
    interpreter::Interpreter,
    optimizer::Optimizer,
    parser::Parser,
    renderer::{
        DefaultsRenderStep, NamespaceConstantsRenderStep, QuickXmlDeserializeRenderStep,
        QuickXmlSerializeRenderStep, RenderStep, Renderer, SerdeQuickXmlTypesRenderStep,
        SerdeXmlRsV7TypesRenderStep, SerdeXmlRsV8TypesRenderStep, TypesRenderStep,
        WithNamespaceTraitRenderStep,
    },
};
pub use self::traits::{AsAny, VecHelper, WithIdent, WithNamespace};

use anyhow::Error as AnyError;
use proc_macro2::TokenStream;
use quote::ToTokens;
use thiserror::Error as ThisError;
use tracing::instrument;

use self::config::{
    Generate, GeneratorConfig, InterpreterConfig, InterpreterFlags, OptimizerConfig,
    OptimizerFlags, ParserConfig, ParserFlags, RendererConfig, Resolver, Schema,
};
use self::macros::{assert, assert_eq, unreachable};
use self::pipeline::{
    optimizer::UnrestrictedBaseFlags,
    parser::resolver::{FileResolver, ManyResolver},
};

/// Generates rust code from a XML schema using the passed `config`.
///
/// This is the most easiest way to use the `xsd-parser` crate. The `generate`
/// function provides a simple way to generate rust code from XML schemas using
/// the passed configuration.
///
/// If you need more detailed control over the generation process or only a part
/// of it, use the [`Parser`], [`Interpreter`], [`Optimizer`] or [`Generator`]
/// directly.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace")]
pub fn generate(config: Config) -> Result<TokenStream, Error> {
    let module = generate_modules(config)?;
    let code = module.to_token_stream();

    Ok(code)
}

/// Generates rust code split into different modules from a XML schema using the
/// passed `config`.
///
/// Like [`generate`] but instead of returning the whole code as token stream it
/// returns a [`Module`], holding the code for itself and his sub-modules.
/// Call [`Module::write_to_files()`] or [`Module::write_to_files_with()`] to
/// actually create the source code files recursively.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace")]
pub fn generate_modules(config: Config) -> Result<Module, Error> {
    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;

    Ok(module)
}

/// Executes the [`Parser`] with the passed `config`.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace")]
pub fn exec_parser(config: ParserConfig) -> Result<Schemas, Error> {
    tracing::info!("Parse Schemas");

    let mut resolver = ManyResolver::new();
    for r in config.resolver {
        match r {
            #[cfg(feature = "web-resolver")]
            Resolver::Web => {
                let web_resolver = self::pipeline::parser::resolver::WebResolver::new();

                resolver = resolver.add_resolver(web_resolver);
            }
            Resolver::File => {
                let file_resolver = FileResolver::new();

                resolver = resolver.add_resolver(file_resolver);
            }
        }
    }

    let mut parser = Parser::new()
        .with_resolver(resolver)
        .resolve_includes(config.flags.contains(ParserFlags::RESOLVE_INCLUDES));

    if config.flags.contains(ParserFlags::DEFAULT_NAMESPACES) {
        parser = parser.with_default_namespaces();
    }

    for (prefix, namespace) in config.namespaces {
        parser = parser.with_namespace(prefix, namespace);
    }

    for schema in config.schemas {
        match schema {
            Schema::Url(url) => parser = parser.add_schema_from_url(url)?,
            Schema::File(path) => parser = parser.add_schema_from_file(path)?,
            Schema::Schema(schema) => parser = parser.add_schema_from_str(&schema)?,
            Schema::NamedSchema(name, schema) => {
                parser = parser.add_named_schema_from_str(name, &schema)?;
            }
        }
    }

    let schemas = parser.finish();

    if let Some(output) = config.debug_output {
        let debug = format!("{schemas:#?}");

        write(output, debug)?;
    }

    Ok(schemas)
}

/// Executes the [`Interpreter`] with the passed `config` and `schema`.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace", skip(schemas))]
pub fn exec_interpreter(config: InterpreterConfig, schemas: &Schemas) -> Result<MetaTypes, Error> {
    tracing::info!("Interpret Schema");

    let mut interpreter = Interpreter::new(schemas);

    if let Some(naming) = config.naming {
        interpreter = interpreter.with_naming_boxed(naming);
    }

    if config.flags.contains(InterpreterFlags::BUILDIN_TYPES) {
        interpreter = interpreter.with_buildin_types()?;
    }

    if config.flags.contains(InterpreterFlags::DEFAULT_TYPEDEFS) {
        interpreter = interpreter.with_default_typedefs()?;
    }

    if config.flags.contains(InterpreterFlags::WITH_XS_ANY_TYPE) {
        interpreter = interpreter.with_xs_any_type()?;
    }

    if config.flags.contains(InterpreterFlags::WITH_NUM_BIG_INT) {
        interpreter = interpreter.with_num_big_int()?;
    }

    for (ident, ty) in config.types {
        let ident = ident.resolve(schemas)?;
        interpreter = interpreter.with_type(ident, ty)?;
    }

    let types = interpreter.finish()?;

    if let Some(output) = config.debug_output {
        let printer = MetaTypesPrinter::new(&types);
        let debug = format!("{printer}");

        write(output, debug)?;
    }

    Ok(types)
}

/// Executes the [`Optimizer`] with the passed `config` and `types`.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace", skip(types))]
pub fn exec_optimizer(config: OptimizerConfig, types: MetaTypes) -> Result<MetaTypes, Error> {
    tracing::info!("Optimize Types");

    let mut optimizer = Optimizer::new(types);
    let mut unrestricted_base = UnrestrictedBaseFlags::empty();

    macro_rules! exec {
        ($flag:ident, $method:ident) => {
            if config.flags.contains(OptimizerFlags::$flag) {
                optimizer = optimizer.$method();
            }
        };
    }

    macro_rules! unrestricted_base {
        ($a:ident, $b:ident) => {
            if config.flags.contains(OptimizerFlags::$a) {
                unrestricted_base |= UnrestrictedBaseFlags::$b;
            }
        };
    }

    unrestricted_base!(USE_UNRESTRICTED_BASE_TYPE_COMPLEX, COMPLEX);
    unrestricted_base!(USE_UNRESTRICTED_BASE_TYPE_SIMPLE, SIMPLE);
    unrestricted_base!(USE_UNRESTRICTED_BASE_TYPE_ENUM, ENUM);
    unrestricted_base!(USE_UNRESTRICTED_BASE_TYPE_UNION, UNION);

    if !unrestricted_base.is_empty() {
        optimizer = optimizer.use_unrestricted_base_type(unrestricted_base);
    }

    exec!(REMOVE_EMPTY_ENUM_VARIANTS, remove_empty_enum_variants);
    exec!(REMOVE_EMPTY_ENUMS, remove_empty_enums);
    exec!(
        REMOVE_DUPLICATE_UNION_VARIANTS,
        remove_duplicate_union_variants
    );
    exec!(REMOVE_EMPTY_UNIONS, remove_empty_unions);
    exec!(CONVERT_DYNAMIC_TO_CHOICE, convert_dynamic_to_choice);
    exec!(FLATTEN_COMPLEX_TYPES, flatten_complex_types);
    exec!(FLATTEN_UNIONS, flatten_unions);
    exec!(MERGE_ENUM_UNIONS, merge_enum_unions);
    exec!(RESOLVE_TYPEDEFS, resolve_typedefs);
    exec!(REMOVE_DUPLICATES, remove_duplicates);
    exec!(RESOLVE_TYPEDEFS, resolve_typedefs);
    exec!(MERGE_CHOICE_CARDINALITIES, merge_choice_cardinalities);
    exec!(SIMPLIFY_MIXED_TYPES, simplify_mixed_types);

    let types = optimizer.finish();

    if let Some(output) = config.debug_output {
        let printer = MetaTypesPrinter::new(&types);
        let debug = format!("{printer}");

        write(output, debug)?;
    }

    Ok(types)
}

/// Executes the [`Generator`] with the passed `config`, `schema` and `types` to
/// generate a [`DataTypes`] for further processing.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace", skip(schemas, types))]
pub fn exec_generator<'types>(
    config: GeneratorConfig,
    schemas: &Schemas,
    types: &'types MetaTypes,
) -> Result<DataTypes<'types>, Error> {
    tracing::info!("Generate Module");

    let mut generator = Generator::new(types)
        .flags(config.flags)
        .box_flags(config.box_flags)
        .typedef_mode(config.typedef_mode);

    if let Some(any_type) = config.any_type {
        generator = generator.any_type(any_type).map_err(GeneratorError::from)?;
    }

    if let Some(any_attribute_type) = config.any_attribute_type {
        generator = generator
            .any_attribute_type(any_attribute_type)
            .map_err(GeneratorError::from)?;
    }

    generator = generator.with_type_postfix(IdentType::Type, config.type_postfix.type_);
    generator = generator.with_type_postfix(IdentType::Element, config.type_postfix.element);
    generator =
        generator.with_type_postfix(IdentType::ElementType, config.type_postfix.element_type);
    generator = generator.with_type_postfix(
        IdentType::NillableContent,
        config.type_postfix.nillable_content,
    );
    generator = generator.with_type_postfix(
        IdentType::DynamicElement,
        config.type_postfix.dynamic_element,
    );

    for triple in config.types {
        let ident = triple.resolve(schemas)?;

        generator = generator.with_type(ident)?;
    }

    let mut generator = generator.into_fixed();
    match config.generate {
        Generate::All => generator = generator.generate_all_types()?,
        Generate::Named => generator = generator.generate_named_types()?,
        Generate::Types(idents) => {
            for triple in idents {
                let ident = triple.resolve(schemas)?;

                generator = generator.generate_type(ident)?;
            }
        }
    }

    let data_types = generator.finish();

    Ok(data_types)
}

/// Executes the rendering process using the passed `config` and the `types`
/// created by the [`Generator`].
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
pub fn exec_render(config: RendererConfig, types: &DataTypes<'_>) -> Result<Module, RendererError> {
    tracing::info!("Render Module");

    let mut renderer = Renderer::new(types)
        .flags(config.flags)
        .xsd_parser_crate(config.xsd_parser);

    if let Some(derive) = config.derive {
        renderer = renderer.derive(derive);
    }

    if let Some(traits) = config.dyn_type_traits {
        renderer = renderer.dyn_type_traits(traits)?;
    }

    for step in config.steps {
        renderer = renderer.with_step_boxed(step.into_render_step());
    }

    let module = renderer.finish();

    Ok(module)
}

/// Error emitted by the [`generate`] function.
#[derive(Debug, ThisError)]
pub enum Error {
    /// IO Error.
    #[error("IO Error: {0}")]
    IoError(
        #[from]
        #[source]
        IoError,
    ),

    /// Parser error.
    #[error("Parser error: {0}")]
    ParserError(#[source] ParserError<AnyError>),

    /// Interpreter error.
    #[error("Interpreter error: {0}")]
    InterpreterError(
        #[from]
        #[source]
        InterpreterError,
    ),

    /// Generator error.
    #[error("Generator error: {0}")]
    GeneratorError(
        #[from]
        #[source]
        GeneratorError,
    ),

    /// Renderer error.
    #[error("Renderer error: {0}")]
    RendererError(
        #[from]
        #[source]
        RendererError,
    ),
}

impl<E> From<ParserError<E>> for Error
where
    AnyError: From<E>,
{
    fn from(value: ParserError<E>) -> Self {
        match value {
            ParserError::IoError(err) => Self::ParserError(ParserError::IoError(err)),
            ParserError::XmlError(err) => Self::ParserError(ParserError::XmlError(err)),
            ParserError::UrlParseError(err) => Self::ParserError(ParserError::UrlParseError(err)),
            ParserError::UnableToResolve(url) => {
                Self::ParserError(ParserError::UnableToResolve(url))
            }
            ParserError::Resolver(err) => {
                Self::ParserError(ParserError::Resolver(AnyError::from(err)))
            }
            ParserError::InvalidFilePath(path) => {
                Self::ParserError(ParserError::InvalidFilePath(path))
            }
        }
    }
}
