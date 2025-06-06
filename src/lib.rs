#![recursion_limit = "256"]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/README.md"))]

pub mod config;
pub mod models;
pub mod pipeline;
pub mod quick_xml;
pub mod xml;

mod macros;
mod traits;

/// Type alias for [`generator::Error`].
pub type GeneratorError = self::pipeline::generator::Error;

/// Type alias for [`interpreter::Error`].
pub type InterpreterError = self::pipeline::interpreter::Error;

/// Type alias for [`parser::Error`].
pub type ParserError<E> = self::pipeline::parser::Error<E>;

use std::fmt::Debug;
use std::fs::write;
use std::io::Error as IoError;

pub use self::config::Config;
pub use self::models::code::{Module, SubModules};
pub use self::models::{Ident, IdentType, Name};
pub use self::pipeline::{
    generator::Generator, interpreter::Interpreter, optimizer::Optimizer, parser::Parser,
};
pub use self::traits::{AsAny, VecHelper, WithIdent, WithNamespace};

use anyhow::Error as AnyError;
use macros::{assert_eq, unreachable};
use proc_macro2::TokenStream;
use quote::ToTokens;
use thiserror::Error as ThisError;
use tracing::instrument;

use self::config::{
    Generate, GeneratorConfig, InterpreterConfig, InterpreterFlags, OptimizerConfig,
    OptimizerFlags, ParserConfig, ParserFlags, Renderer, Resolver, Schema,
};
use self::models::{meta::MetaTypes, schema::Schemas};
use self::pipeline::{
    generator::renderer::{
        DefaultsRenderer, NamespaceConstantsRenderer, QuickXmlDeserializeRenderer,
        QuickXmlSerializeRenderer, TypesRenderer, WithNamespaceTraitRenderer,
    },
    parser::resolver::{FileResolver, ManyResolver},
    TypesPrinter,
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
    let types = exec_interpreter(config.interpreter, &schemas)?;
    let types = exec_optimizer(config.optimizer, types)?;
    let module = exec_generator_module(config.generator, &schemas, &types)?;

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
        let printer = TypesPrinter::new(&types);
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

    macro_rules! exec {
        ($flag:ident, $method:ident) => {
            if config.flags.contains(OptimizerFlags::$flag) {
                optimizer = optimizer.$method();
            }
        };
    }

    exec!(REMOVE_EMPTY_ENUM_VARIANTS, remove_empty_enum_variants);
    exec!(REMOVE_EMPTY_ENUMS, remove_empty_enums);
    exec!(
        REMOVE_DUPLICATE_UNION_VARIANTS,
        remove_duplicate_union_variants
    );
    exec!(REMOVE_EMPTY_UNIONS, remove_empty_unions);
    exec!(USE_UNRESTRICTED_BASE_TYPE, use_unrestricted_base_type);
    exec!(CONVERT_DYNAMIC_TO_CHOICE, convert_dynamic_to_choice);
    exec!(FLATTEN_COMPLEX_TYPES, flatten_complex_types);
    exec!(FLATTEN_UNIONS, flatten_unions);
    exec!(MERGE_ENUM_UNIONS, merge_enum_unions);
    exec!(RESOLVE_TYPEDEFS, resolve_typedefs);
    exec!(REMOVE_DUPLICATES, remove_duplicates);
    exec!(RESOLVE_TYPEDEFS, resolve_typedefs);
    exec!(MERGE_CHOICE_CARDINALITIES, merge_choice_cardinalities);

    let types = optimizer.finish();

    if let Some(output) = config.debug_output {
        let printer = TypesPrinter::new(&types);
        let debug = format!("{printer}");

        write(output, debug)?;
    }

    Ok(types)
}

/// Executes the [`Generator`] with the passed `config`, `schema` and `types` to
/// generate the whole code as token stream.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace", skip(schemas, types))]
pub fn exec_generator(
    config: GeneratorConfig,
    schemas: &Schemas,
    types: &MetaTypes,
) -> Result<TokenStream, Error> {
    let module = exec_generator_module(config, schemas, types)?;
    let code = module.to_token_stream();

    Ok(code)
}

/// Executes the [`Generator`] with the passed `config`, `schema` and `types` to
/// generate a [`Module`] for further processing.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace", skip(schemas, types))]
pub fn exec_generator_module(
    config: GeneratorConfig,
    schemas: &Schemas,
    types: &MetaTypes,
) -> Result<Module, Error> {
    tracing::info!("Generate Module");

    let mut generator = Generator::new(types)
        .flags(config.flags)
        .box_flags(config.box_flags)
        .typedef_mode(config.typedef_mode)
        .serde_support(config.serde_support)
        .xsd_parser_crate(config.xsd_parser);

    if let Some(derive) = config.derive {
        generator = generator.derive(derive);
    }

    if let Some(traits) = config.dyn_type_traits {
        generator = generator.dyn_type_traits(traits)?;
    }

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

    for triple in config.types {
        let ident = triple.resolve(schemas)?;

        generator = generator.with_type(ident)?;
    }

    for renderer in config.renderers {
        match renderer {
            Renderer::Types => generator = generator.with_renderer(TypesRenderer),
            Renderer::Defaults => generator = generator.with_renderer(DefaultsRenderer),
            Renderer::NamespaceConstants => {
                generator = generator.with_renderer(NamespaceConstantsRenderer);
            }
            Renderer::WithNamespaceTrait => {
                generator = generator.with_renderer(WithNamespaceTraitRenderer);
            }
            Renderer::QuickXmlSerialize => {
                generator = generator.with_renderer(QuickXmlSerializeRenderer);
            }
            Renderer::QuickXmlDeserialize { boxed_deserializer } => {
                generator =
                    generator.with_renderer(QuickXmlDeserializeRenderer { boxed_deserializer });
            }
        }
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

    let module = generator.into_module();

    Ok(module)
}

/// Error emitted by the [`generate`] function.
#[derive(Debug, ThisError)]
pub enum Error {
    /// IO Error.
    #[error("IO Error: {0}")]
    IoError(#[from] IoError),

    /// Parser error.
    #[error("Parser error: {0}")]
    ParserError(ParserError<AnyError>),

    /// Interpreter error.
    #[error("Interpreter error: {0}")]
    InterpreterError(#[from] InterpreterError),

    /// Generator error.
    #[error("Generator error: {0}")]
    GeneratorError(#[from] GeneratorError),
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
