#![doc = include_str!(concat!(env!("OUT_DIR"), "/README.md"))]

pub mod code;
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

/// Type alias for [`optimizer::Error`].
pub type OptimizerError = optimizer::Error;

use std::fs::write;

pub use config::Config;
pub use generator::Generator;
pub use interpreter::Interpreter;
pub use misc::{AsAny, Error, WithNamespace};
pub use optimizer::{
    ConvertDynamicToChoice, FlattenComplexTypes, FlattenUnions, MergeChoiceCardinalities,
    MergeEnumUnions, RemoveDuplicateUnionVariants, RemoveDuplicates, RemoveEmptyEnumVariants,
    RemoveEmptyEnums, RemoveEmptyUnions, ResolveTypedefs, TypeTransformer, UseUnrestrictedBaseType,
};
pub use parser::Parser;

use macros::{assert_eq, unreachable};
use proc_macro2::TokenStream;
use tracing::instrument;

use self::config::{
    Generate, GeneratorConfig, InterpreterConfig, InterpreterFlags, OptimizerConfig,
    OptimizerFlags, ParserConfig, ParserFlags, Renderer, Resolver, Schema,
};
use self::generator::renderer::{
    DefaultsRenderer, NamespaceConstantsRenderer, QuickXmlDeserializeRenderer,
    QuickXmlSerializeRenderer, TypesRenderer, WithNamespaceTraitRenderer,
};
use self::misc::TypesPrinter;
use self::parser::resolver::{FileResolver, ManyResolver};
use self::schema::Schemas;
use self::types::{IdentType, Types};

/// Generates rust code from a XML schema using the passed `config`.
///
/// This is the most easiest way to use the `xsd-parser` crate. The `generate`
/// function provides a simple way to generate rust code from XML schemas using
/// the passed configuration.
///
/// If you need more detailed control over the generation process or only a part
/// of it, use the [`Parser`], [`Interpreter`], [`TypeTransformer`] or [`Generator`]
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

/// Executes the [`Interpreter`] with the passed `config` and `schema`.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace", skip(schemas))]
pub fn exec_interpreter(config: InterpreterConfig, schemas: &Schemas) -> Result<Types, Error> {
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

/// Executes various [`TypeTransformer`]s with the passed `config` and `types`.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace", skip(types))]
pub fn exec_optimizer(config: OptimizerConfig, types: Types) -> Result<Types, Error> {
    use OptimizerFlags as F;

    tracing::info!("Optimize Types");

    let fa = |flag: F| config.flags.contains(flag);

    let types = types
        .apply_transformer_if(RemoveEmptyEnumVariants, fa(F::REMOVE_EMPTY_ENUM_VARIANTS))?
        .apply_transformer_if(RemoveEmptyEnums, fa(F::REMOVE_EMPTY_ENUMS))?
        .apply_transformer_if(
            RemoveDuplicateUnionVariants,
            fa(F::REMOVE_DUPLICATE_UNION_VARIANTS),
        )?
        .apply_transformer_if(RemoveEmptyUnions, fa(F::REMOVE_EMPTY_UNIONS))?
        .apply_transformer_if(UseUnrestrictedBaseType, fa(F::USE_UNRESTRICTED_BASE_TYPE))?
        .apply_transformer_if(ConvertDynamicToChoice, fa(F::CONVERT_DYNAMIC_TO_CHOICE))?
        .apply_transformer_if(FlattenComplexTypes, fa(F::FLATTEN_COMPLEX_TYPES))?
        .apply_transformer_if(FlattenUnions, fa(F::FLATTEN_UNIONS))?
        .apply_transformer_if(MergeEnumUnions, fa(F::MERGE_ENUM_UNIONS))?
        .apply_transformer_if(ResolveTypedefs, fa(F::RESOLVE_TYPEDEFS))?
        .apply_transformer_if(RemoveDuplicates, fa(F::REMOVE_DUPLICATES))?
        .apply_transformer_if(ResolveTypedefs, fa(F::RESOLVE_TYPEDEFS))?
        .apply_transformer_if(MergeChoiceCardinalities, fa(F::MERGE_CHOICE_CARDINALITIES))?;

    if let Some(output) = config.debug_output {
        let printer = TypesPrinter::new(&types);
        let debug = format!("{printer}");

        write(output, debug)?;
    }

    Ok(types)
}

/// Executes the [`Generator`] with the passed `config`, `schema` and `types`.
///
/// # Errors
///
/// Returns a suitable [`Error`] type if the process was not successful.
#[instrument(err, level = "trace", skip(schemas, types))]
pub fn exec_generator(
    config: GeneratorConfig,
    schemas: &Schemas,
    types: &Types,
) -> Result<TokenStream, Error> {
    tracing::info!("Generate Code");

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
            Renderer::QuickXmlDeserialize => {
                generator = generator.with_renderer(QuickXmlDeserializeRenderer);
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

    let code = generator.finish();

    Ok(code)
}
