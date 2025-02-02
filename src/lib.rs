#![doc = include_str!(concat!(env!("OUT_DIR"), "/README.md"))]

pub mod config;
pub mod generator;
pub mod interpreter;
pub mod optimizer;
pub mod parser;
pub mod quick_xml;
pub mod schema;
pub mod types;

mod macros;
mod misc;

/// Type alias for [`generator::Error`].
pub type GeneratorError = generator::Error;

/// Type alias for [`interpreter::Error`].
pub type InterpreterError = interpreter::Error;

/// Type alias for [`parser::Error`].
pub type ParserError<E> = parser::Error<E>;

use std::fs::write;

pub use config::Config;
pub use generator::Generator;
pub use interpreter::Interpreter;
pub use misc::{AsAny, Error, WithNamespace};
pub use optimizer::Optimizer;
pub use parser::Parser;

use macros::{assert_eq, unreachable};
use proc_macro2::TokenStream;
use schema::{NamespacePrefix, Schemas};
use tracing::instrument;

use self::config::{
    Generate, GeneratorConfig, InterpreterConfig, InterpreterFlags, OptimizerConfig,
    OptimizerFlags, ParserConfig, ParserFlags, Resolver, Schema,
};
use self::misc::TypesPrinter;
use self::parser::resolver::{FileResolver, ManyResolver};
use self::types::{Ident, IdentType, Name, Types};

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
    let schemas = exec_parser(config.parser)?;
    let types = exec_interpreter(config.interpreter, &schemas)?;
    let types = exec_optimizer(config.optimizer, types)?;
    let code = exec_generator(config.generator, &schemas, &types)?;

    Ok(code)
}

#[instrument(err, level = "trace")]
fn exec_parser(config: ParserConfig) -> Result<Schemas, Error> {
    tracing::info!("Parse Schemas");

    let mut resolver = ManyResolver::new();
    for r in config.resolver {
        match r {
            #[cfg(feature = "web-resolver")]
            Resolver::Web => {
                let web_resolver = self::parser::resolver::WebResolver::new();

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

#[instrument(err, level = "trace", skip(schemas))]
fn exec_interpreter(config: InterpreterConfig, schemas: &Schemas) -> Result<Types, Error> {
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

    for (x, ident, type_) in config.types {
        let ident = resolve_ident(schemas, &ident, x)?;
        interpreter = interpreter.with_type(ident, type_)?;
    }

    let types = interpreter.finish()?;

    if let Some(output) = config.debug_output {
        let printer = TypesPrinter::new(&types);
        let debug = format!("{printer}");

        write(output, debug)?;
    }

    Ok(types)
}

#[instrument(err, level = "trace", skip(types))]
fn exec_optimizer(config: OptimizerConfig, types: Types) -> Result<Types, Error> {
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
    exec!(FLATTEN_ELEMENT_CONTENT, flatten_element_content);
    exec!(FLATTEN_UNIONS, flatten_unions);
    exec!(MERGE_ENUM_UNIONS, merge_enum_unions);
    exec!(RESOLVE_TYPEDEFS, resolve_typedefs);
    exec!(REMOVE_DUPLICATES, remove_duplicates);
    exec!(RESOLVE_TYPEDEFS, resolve_typedefs);

    let types = optimizer.finish();

    if let Some(output) = config.debug_output {
        let printer = TypesPrinter::new(&types);
        let debug = format!("{printer}");

        write(output, debug)?;
    }

    Ok(types)
}

#[instrument(err, level = "trace", skip(schemas, types))]
fn exec_generator(
    config: GeneratorConfig,
    schemas: &Schemas,
    types: &Types,
) -> Result<TokenStream, Error> {
    tracing::info!("Generate Code");

    let mut generator = Generator::new(types)
        .box_flags(config.box_flags)
        .content_mode(config.content_mode)
        .typedef_mode(config.typedef_mode)
        .serde_support(config.serde_support)
        .xsd_parser_crate(config.xsd_parser)
        .generate_flags(config.flags);

    if let Some(derive) = config.derive {
        generator = generator.derive(derive);
    }

    if let Some(traits) = config.dyn_type_traits {
        generator = generator.dyn_type_traits(traits);
    }

    generator = generator.with_type_postfix(IdentType::Type, config.type_postfix.type_);
    generator = generator.with_type_postfix(IdentType::Element, config.type_postfix.element);
    generator =
        generator.with_type_postfix(IdentType::ElementType, config.type_postfix.element_type);

    for (type_, ident) in config.types {
        let ident = resolve_ident(schemas, &ident, type_)?;

        generator = generator.with_type(ident)?;
    }

    match config.generate {
        Generate::All => generator = generator.generate_all_types()?,
        Generate::Types(idents) => {
            for (type_, ident) in idents {
                let ident = resolve_ident(schemas, &ident, type_)?;

                generator = generator.generate_type(ident)?;
            }
        }
    }

    let code = generator.finish();

    Ok(code)
}

fn resolve_ident(schemas: &Schemas, ident: &str, type_: IdentType) -> Result<Ident, Error> {
    let (prefix, name) = ident
        .split_once(':')
        .map_or((None, ident), |(a, b)| (Some(a), b));
    let ns = prefix
        .map(|s| NamespacePrefix::new(s.as_bytes().to_owned()))
        .map(|prefix| {
            schemas
                .resolve_prefix(&prefix)
                .ok_or_else(|| InterpreterError::UnknownNamespacePrefix(prefix.clone()))
        })
        .transpose()?;

    Ok(Ident {
        name: Name::new(name),
        ns,
        type_,
    })
}
