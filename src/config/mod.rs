//! Contains the [`Config`] structures for the [`generate`](super::generate) method.

mod generator;
mod interpreter;
mod optimizer;
mod parser;
mod renderer;

pub use crate::models::{
    meta::MetaType,
    schema::{Namespace, NamespacePrefix},
    Ident, IdentType, Name,
};

use crate::models::schema::Schemas;
use crate::InterpreterError;

pub use self::generator::{
    BoxFlags, Generate, GeneratorConfig, GeneratorFlags, SerdeSupport, TypePostfix, TypedefMode,
};
pub use self::interpreter::{InterpreterConfig, InterpreterFlags};
pub use self::optimizer::{OptimizerConfig, OptimizerFlags};
pub use self::parser::{ParserConfig, ParserFlags, Resolver, Schema};
pub use self::renderer::{DynTypeTraits, RenderStep, RendererConfig, RendererFlags};

/// Configuration structure for the [`generate`](super::generate) method.
#[must_use]
#[derive(Default, Debug, Clone)]
pub struct Config {
    /// Configuration for the schema parser.
    pub parser: ParserConfig,

    /// Configuration for the schema interpreter.
    pub interpreter: InterpreterConfig,

    /// Configuration for the type information optimizer.
    pub optimizer: OptimizerConfig,

    /// Configuration for the code generator.
    pub generator: GeneratorConfig,

    /// Configuration for the code renderer.
    pub renderer: RendererConfig,
}

impl Config {
    /// Set parser flags to the config.
    pub fn set_parser_flags(mut self, flags: ParserFlags) -> Self {
        self.parser.flags = flags;

        self
    }

    /// Add parser flags to the config.
    pub fn with_parser_flags(mut self, flags: ParserFlags) -> Self {
        self.parser.flags.insert(flags);

        self
    }

    /// Remove parser flags to the config.
    pub fn without_parser_flags(mut self, flags: ParserFlags) -> Self {
        self.parser.flags.remove(flags);

        self
    }

    /// Set interpreter flags to the config.
    pub fn set_interpreter_flags(mut self, flags: InterpreterFlags) -> Self {
        self.interpreter.flags = flags;

        self
    }

    /// Add code interpreter flags to the config.
    pub fn with_interpreter_flags(mut self, flags: InterpreterFlags) -> Self {
        self.interpreter.flags.insert(flags);

        self
    }

    /// Remove code interpreter flags to the config.
    pub fn without_interpreter_flags(mut self, flags: InterpreterFlags) -> Self {
        self.interpreter.flags.remove(flags);

        self
    }

    /// Set optimizer flags to the config.
    pub fn set_optimizer_flags(mut self, flags: OptimizerFlags) -> Self {
        self.optimizer.flags = flags;

        self
    }

    /// Add optimizer flags to the config.
    pub fn with_optimizer_flags(mut self, flags: OptimizerFlags) -> Self {
        self.optimizer.flags.insert(flags);

        self
    }

    /// Remove optimizer flags to the config.
    pub fn without_optimizer_flags(mut self, flags: OptimizerFlags) -> Self {
        self.optimizer.flags.remove(flags);

        self
    }

    /// Set generator flags to the config.
    pub fn set_generator_flags(mut self, flags: GeneratorFlags) -> Self {
        self.generator.flags = flags;

        self
    }

    /// Add code generator flags to the config.
    pub fn with_generator_flags(mut self, flags: GeneratorFlags) -> Self {
        self.generator.flags.insert(flags);

        self
    }

    /// Remove code generator flags to the config.
    pub fn without_generator_flags(mut self, flags: GeneratorFlags) -> Self {
        self.generator.flags.remove(flags);

        self
    }

    /// Set renderer flags to the config.
    pub fn set_renderer_flags(mut self, flags: RendererFlags) -> Self {
        self.renderer.flags = flags;

        self
    }

    /// Add code renderer flags to the config.
    pub fn with_renderer_flags(mut self, flags: RendererFlags) -> Self {
        self.renderer.flags.insert(flags);

        self
    }

    /// Remove code renderer flags to the config.
    pub fn without_renderer_flags(mut self, flags: RendererFlags) -> Self {
        self.renderer.flags.remove(flags);

        self
    }

    /// Set boxing flags to the code generator config.
    pub fn set_box_flags(mut self, flags: BoxFlags) -> Self {
        self.generator.box_flags = flags;

        self
    }

    /// Add boxing flags to the code generator config.
    pub fn with_box_flags(mut self, flags: BoxFlags) -> Self {
        self.generator.box_flags.insert(flags);

        self
    }

    /// Remove boxing flags to the code generator config.
    pub fn without_box_flags(mut self, flags: BoxFlags) -> Self {
        self.generator.box_flags.remove(flags);

        self
    }

    /// Adds the passed `step` to the config.
    ///
    /// If the same type of renderer was already added,
    /// it is replaced by the new one.
    pub fn with_render_step(mut self, step: RenderStep) -> Self {
        if let Some(index) = self.renderer.steps.iter().position(|x| x == &step) {
            self.renderer.steps[index] = step;
        } else {
            self.renderer.steps.push(step);
        }

        self
    }

    /// Add multiple renderers to the generator config.
    ///
    /// See [`with_render_step`](Self::with_render_step) for details.
    pub fn with_render_steps<I>(mut self, steps: I) -> Self
    where
        I: IntoIterator<Item = RenderStep>,
    {
        for step in steps {
            self = self.with_render_step(step);
        }

        self
    }

    /// Enable code generation for [`quick_xml`] serialization.
    pub fn with_quick_xml_serialize(self) -> Self {
        self.with_render_steps([
            RenderStep::Types,
            RenderStep::Defaults,
            RenderStep::NamespaceConstants,
            RenderStep::QuickXmlSerialize,
        ])
    }

    /// Enable code generation for [`quick_xml`] deserialization with the default settings.
    pub fn with_quick_xml_deserialize(self) -> Self {
        self.with_quick_xml_deserialize_config(false)
    }

    /// Enable code generation for [`quick_xml`] deserialization
    /// with the passed configuration.
    pub fn with_quick_xml_deserialize_config(self, boxed_deserializer: bool) -> Self {
        self.with_render_steps([
            RenderStep::Types,
            RenderStep::Defaults,
            RenderStep::NamespaceConstants,
            RenderStep::QuickXmlDeserialize { boxed_deserializer },
        ])
    }

    /// Enable code generation for [`quick_xml`] serialization and deserialization
    /// with the default settings.
    pub fn with_quick_xml(self) -> Self {
        self.with_quick_xml_serialize().with_quick_xml_deserialize()
    }

    /// Enable code generation for [`quick_xml`] serialization and deserialization
    /// with the passed configuration.
    pub fn with_quick_xml_config(self, boxed_deserializer: bool) -> Self {
        self.with_quick_xml_serialize()
            .with_quick_xml_deserialize_config(boxed_deserializer)
    }

    /// Set the [`serde`] support.
    pub fn with_serde_support(mut self, serde_support: SerdeSupport) -> Self {
        self.generator.serde_support = serde_support;

        if self.generator.serde_support == SerdeSupport::None {
            self
        } else {
            self.optimizer.flags |= OptimizerFlags::SERDE;

            self.with_render_steps([RenderStep::Types, RenderStep::Defaults])
        }
    }

    /// Set the types the code should be generated for.
    pub fn with_generate<I>(mut self, types: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<IdentTriple>,
    {
        self.generator.generate = Generate::Types(types.into_iter().map(Into::into).collect());

        self
    }

    /// Set the typedef mode for the generator.
    pub fn with_typedef_mode(mut self, mode: TypedefMode) -> Self {
        self.generator.typedef_mode = mode;

        self
    }

    /// Set the traits the generated types should derive from.
    pub fn with_derive<I>(mut self, derive: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.renderer.derive = Some(
            derive
                .into_iter()
                .map(Into::into)
                .filter(|s| !s.is_empty())
                .collect(),
        );

        self
    }
}

/// Identifier that is used inside the config.
///
/// Each element in a XML schema is uniquely identified by the following three
/// values:
///     - The namespace of the element (or no namespace at all).
///     - The name of the element.
///     - The type of element.
///
/// This struct is used to bundle these three information to a unique identifier
/// for an element.
#[derive(Debug, Clone)]
pub struct IdentTriple {
    /// Namespace where the type is defined in.
    pub ns: Option<NamespaceIdent>,

    /// Name of the type.
    pub name: String,

    /// Type of the identifier (because pure names are not unique in XSD).
    pub type_: IdentType,
}

impl IdentTriple {
    /// Resolve the triple to an actual type that is available in the schema.
    ///
    /// # Errors
    ///
    /// Returns an error if the namespace or the namespace prefix could not be
    /// resolved.
    pub fn resolve(self, schemas: &Schemas) -> Result<Ident, InterpreterError> {
        let ns = match self.ns {
            None => None,
            Some(NamespaceIdent::Prefix(prefix)) => Some(
                schemas
                    .resolve_prefix(&prefix)
                    .ok_or(InterpreterError::UnknownNamespacePrefix(prefix))?,
            ),
            #[allow(clippy::unnecessary_literal_unwrap)]
            Some(NamespaceIdent::Namespace(ns)) => {
                let ns = Some(ns);
                Some(
                    schemas
                        .resolve_namespace(&ns)
                        .ok_or_else(|| InterpreterError::UnknownNamespace(ns.unwrap()))?,
                )
            }
        };

        Ok(Ident {
            ns,
            name: Name::new_named(self.name),
            type_: self.type_,
        })
    }
}

impl<X> From<(IdentType, X)> for IdentTriple
where
    X: AsRef<str>,
{
    fn from((type_, ident): (IdentType, X)) -> Self {
        let ident = ident.as_ref();
        let (prefix, name) = ident
            .split_once(':')
            .map_or((None, ident), |(a, b)| (Some(a), b));
        let ns = prefix.map(|x| NamespaceIdent::prefix(x.as_bytes().to_owned()));
        let name = name.to_owned();

        Self { ns, name, type_ }
    }
}

impl<N, X> From<(IdentType, N, X)> for IdentTriple
where
    N: Into<Option<NamespaceIdent>>,
    X: Into<String>,
{
    fn from((type_, ns, name): (IdentType, N, X)) -> Self {
        let ns = ns.into();
        let name = name.into();

        Self { ns, name, type_ }
    }
}

/// Identifies a namespace by either it's known prefix or by the namespace directly.
#[derive(Debug, Clone)]
pub enum NamespaceIdent {
    /// Uses a namespace prefix to refer to a specific namespace in the schema.
    Prefix(NamespacePrefix),

    /// Uses the full namespace to refer to a specific namespace in the schema.
    Namespace(Namespace),
}

impl NamespaceIdent {
    /// Creates a new [`NamespaceIdent::Prefix`] instance from the passed `value`.
    pub fn prefix<X>(value: X) -> Self
    where
        NamespacePrefix: From<X>,
    {
        Self::Prefix(NamespacePrefix::from(value))
    }

    /// Creates a new [`NamespaceIdent::Namespace`] instance from the passed `value`.
    pub fn namespace<X>(value: X) -> Self
    where
        Namespace: From<X>,
    {
        Self::Namespace(Namespace::from(value))
    }
}

impl From<Namespace> for NamespaceIdent {
    fn from(value: Namespace) -> Self {
        Self::Namespace(value)
    }
}

impl From<NamespacePrefix> for NamespaceIdent {
    fn from(value: NamespacePrefix) -> Self {
        Self::Prefix(value)
    }
}
