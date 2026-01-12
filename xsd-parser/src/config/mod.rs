//! Contains the [`Config`] structures for the [`generate`](super::generate) method.

mod generator;
mod interpreter;
mod optimizer;
mod parser;
mod renderer;

use url::Url;

pub use xsd_parser_types::misc::{Namespace, NamespacePrefix};

pub use crate::models::{meta::MetaType, schema::NamespaceId, IdentType, Name, TypeIdent};

use crate::models::schema::xs::SchemaContent;
use crate::pipeline::renderer::NamespaceSerialization;
use crate::traits::Naming;
use crate::InterpreterError;

pub use self::generator::{
    BoxFlags, Generate, GeneratorConfig, GeneratorFlags, TypePostfix, TypedefMode,
};
pub use self::interpreter::{InterpreterConfig, InterpreterFlags};
pub use self::optimizer::{OptimizerConfig, OptimizerFlags};
pub use self::parser::{ParserConfig, ParserFlags, Resolver, Schema};
pub use self::renderer::{
    DynTypeTraits, RenderStep, RenderStepConfig, RendererConfig, RendererFlags, SerdeXmlRsVersion,
};

use crate::models::schema::{SchemaId, Schemas};

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
    /// Adds the passed `schema` to the list of schemas to parse.
    pub fn with_schema<T>(mut self, schema: T) -> Self
    where
        T: Into<Schema>,
    {
        self.parser.schemas.push(schema.into());

        self
    }

    /// Adds the passed `schemas` to the list of schemas to parse.
    pub fn with_schemas<I>(mut self, schemas: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Schema>,
    {
        for schema in schemas {
            self = self.with_schema(schema.into());
        }

        self
    }

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
    pub fn with_render_step<T>(mut self, step: T) -> Self
    where
        T: RenderStepConfig + 'static,
    {
        let step = Box::new(step);
        let mut index = 0;
        let mut position = None;

        // Find the position to place the new step and remove any other mutual exclusive step
        self.renderer.steps.retain(|x| {
            let mut remove = x.is_mutual_exclusive_to(&*step) || step.is_mutual_exclusive_to(&**x);

            if remove && position.is_none() {
                remove = false;
                position = Some(index);
            }

            index += 1;

            !remove
        });

        // Insert at the found position or append
        if let Some(pos) = position {
            self.renderer.steps[pos] = step;
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
        I: IntoIterator,
        I::Item: RenderStepConfig + 'static,
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
            RenderStep::PrefixConstants,
            RenderStep::NamespaceConstants,
            RenderStep::QuickXmlSerialize {
                namespaces: NamespaceSerialization::Global,
                default_namespace: None,
            },
        ])
    }

    /// Enable code generation for [`quick_xml`] serialization.
    pub fn with_quick_xml_serialize_config(
        mut self,
        namespaces: NamespaceSerialization,
        default_namespace: Option<Namespace>,
    ) -> Self {
        self = self.with_render_steps([RenderStep::Types, RenderStep::Defaults]);

        if namespaces != NamespaceSerialization::None {
            self = self.with_render_step(RenderStep::PrefixConstants);
        }

        self.with_render_steps([
            RenderStep::NamespaceConstants,
            RenderStep::QuickXmlSerialize {
                namespaces,
                default_namespace,
            },
        ])
    }

    /// Enable code generation for [`quick_xml`] deserialization with the default settings.
    pub fn with_quick_xml_deserialize(self) -> Self {
        self.with_quick_xml_deserialize_config(false)
    }

    /// Enable render steps for [`quick_xml`] deserialization
    /// with the passed configuration.
    pub fn with_quick_xml_deserialize_config(self, boxed_deserializer: bool) -> Self {
        self.with_render_steps([
            RenderStep::Types,
            RenderStep::Defaults,
            RenderStep::NamespaceConstants,
            RenderStep::QuickXmlDeserialize { boxed_deserializer },
        ])
    }

    /// Enable render steps for [`quick_xml`] serialization and deserialization
    /// with the default settings.
    pub fn with_quick_xml(self) -> Self {
        self.with_quick_xml_serialize().with_quick_xml_deserialize()
    }

    /// Enable render steps for [`quick_xml`] serialization and deserialization
    /// with the passed configuration.
    pub fn with_quick_xml_config(
        self,
        namespace_serialization: NamespaceSerialization,
        default_serialize_namespace: Option<Namespace>,
        boxed_deserializer: bool,
    ) -> Self {
        self.with_quick_xml_serialize_config(namespace_serialization, default_serialize_namespace)
            .with_quick_xml_deserialize_config(boxed_deserializer)
    }

    /// Enable render steps for types with [`quick_xml`] serde support.
    pub fn with_serde_quick_xml(mut self) -> Self {
        self.optimizer.flags |= OptimizerFlags::SERDE;

        self.with_render_steps([RenderStep::TypesSerdeQuickXml, RenderStep::Defaults])
    }

    /// Enable render steps for types with [`quick_xml`] serde support.
    pub fn with_serde_xml_rs(mut self, version: SerdeXmlRsVersion) -> Self {
        self.optimizer.flags |= OptimizerFlags::SERDE;

        self.with_render_steps([
            RenderStep::TypesSerdeXmlRs { version },
            RenderStep::Defaults,
        ])
    }

    /// Set the types the code should be generated for.
    pub fn with_generate<I>(mut self, types: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<IdentQuadruple>,
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

    /// Enable support for `xs:any` types.
    pub fn with_any_type_support(self) -> Self {
        self.with_generator_flags(GeneratorFlags::ANY_TYPE_SUPPORT)
    }

    /// Set the types to use to handle `xs:any` and `xs:anyAttribute` elements.
    pub fn with_any_types<S, T>(mut self, any_type: S, any_attributes_type: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.generator.any_type = any_type.into();
        self.generator.any_attributes_type = any_attributes_type.into();

        self.with_any_type_support()
    }

    /// Enable support for mixed types.
    pub fn with_mixed_type_support(self) -> Self {
        self.with_generator_flags(GeneratorFlags::MIXED_TYPE_SUPPORT)
    }

    /// Set the types to use to handle mixed types and text data.
    pub fn with_mixed_types<S, T>(mut self, text_type: S, mixed_type: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.generator.text_type = text_type.into();
        self.generator.mixed_type = mixed_type.into();

        self.with_mixed_type_support()
    }

    /// Enable support for nillable types.
    pub fn with_nillable_type_support(self) -> Self {
        self.with_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT)
    }

    /// Set the type to use to handle nillable elements.
    pub fn with_nillable_type<S>(mut self, nillable_type: S) -> Self
    where
        S: Into<String>,
    {
        self.generator.nillable_type = nillable_type.into();

        self.with_nillable_type_support()
    }

    /// Set the [`Naming`] trait that is used to generated names in the interpreter.
    pub fn with_naming<X>(mut self, naming: X) -> Self
    where
        X: Naming + 'static,
    {
        self.interpreter.naming = Some(Box::new(naming));

        self
    }

    /// Set the postfix that should be applied to the name of types.
    ///
    /// For details please refer to [`GeneratorConfig::type_postfix`].
    pub fn with_type_postfix<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.generator.type_postfix.type_ = value.into();

        self
    }

    /// Set the postfix that should be applied to the name of elements.
    ///
    /// For details please refer to [`GeneratorConfig::type_postfix`].
    pub fn with_element_postfix<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.generator.type_postfix.element = value.into();

        self
    }

    /// Set the postfix that should be applied to the name of element types.
    ///
    /// For details please refer to [`GeneratorConfig::type_postfix`].
    pub fn with_element_type_postfix<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.generator.type_postfix.element = value.into();

        self
    }

    /// Set the postfix that should be applied to the name of nillable content types.
    ///
    /// For details please refer to [`GeneratorConfig::type_postfix`].
    pub fn with_nillable_content_postfix<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.generator.type_postfix.nillable_content = value.into();

        self
    }

    /// Set the postfix that should be applied to the name of dynamic elements.
    ///
    /// For details please refer to [`GeneratorConfig::type_postfix`].
    pub fn with_dynamic_element_postfix<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.generator.type_postfix.dynamic_element = value.into();

        self
    }
}

/// Convenient type to not break the public interface.
///
/// The type was renamed to [`IdentQuadruple`].
#[deprecated(note = "Use IdentQuadruple instead")]
pub type IdentTriple = IdentQuadruple;

/// Identifier that is used inside the config.
///
/// Each element in a XML schema is uniquely identified by the following four
/// values:
///     - The namespace of the element (or no namespace at all).
///     - The schema the element was defined in.
///     - The name of the element.
///     - The type of element.
///
/// This struct is used to bundle these three information to a unique identifier
/// for an element.
#[derive(Debug, Clone)]
pub struct IdentQuadruple {
    /// Namespace where the type is defined in.
    pub ns: Option<NamespaceIdent>,

    /// Id of the schema the type is defined in.
    pub schema: Option<SchemaIdent>,

    /// Name of the type.
    pub name: String,

    /// Type of the identifier (because pure names are not unique in XSD).
    pub type_: IdentType,
}

impl IdentQuadruple {
    /// Create a new [`IdentQuadruple`] instance from the passed `name` and `type_`.
    #[inline]
    #[must_use]
    pub fn new<S>(name: S, type_: IdentType) -> Self
    where
        S: Into<String>,
    {
        Self {
            ns: None,
            schema: None,
            name: name.into(),
            type_,
        }
    }

    /// Adds a [`NamespaceIdent`] to this identifier quadruple.
    #[inline]
    #[must_use]
    pub fn with_ns<X>(mut self, ns: X) -> Self
    where
        X: Into<NamespaceIdent>,
    {
        self.ns = Some(ns.into());

        self
    }

    /// Adds a [`SchemaIdent`] to this identifier quadruple.
    #[inline]
    #[must_use]
    pub fn with_schema<X>(mut self, schema: X) -> Self
    where
        X: Into<SchemaIdent>,
    {
        self.schema = Some(schema.into());

        self
    }

    /// Sets the name of the type that is identified by this identifier quadruple.
    #[inline]
    #[must_use]
    pub fn with_name<X>(mut self, name: X) -> Self
    where
        X: Into<String>,
    {
        self.name = name.into();

        self
    }

    /// Sets the identifier type of this identifier quadruple.
    #[inline]
    #[must_use]
    pub fn with_type(mut self, type_: IdentType) -> Self {
        self.type_ = type_;

        self
    }

    /// Resolve the quadruple to an actual type identifier that is available in
    /// the schema.
    ///
    /// /// <div class="warning">
    /// *Caution*
    /// This may end up in a type with the [`schema`](TypeIdent::schema) not fully
    /// resolved. This can happen if you specified the wrong schema name, or schema
    /// location. If you didn't provide a [`SchemaIdent`] at all, the type is resolved
    /// by it's name, which is also not always successful, if the type is not defined
    /// in the root of the available schemas.
    ///
    /// If you use this to get suitable identifiers to define types for the interpreter
    /// (see [`with_type`](crate::Interpreter::with_type)), then you are fine, because
    /// the interpreter will resolve unknown schema IDs by it's own.
    ///
    /// If you want to use the resolved identifier for selecting a [`MetaType`]
    /// from the resulting [`MetaTypes`](crate::MetaTypes) structure created by
    /// the interpreted, you have to resolve the type additionally using the
    /// [`IdentCache`](crate::IdentCache), which is also returned by the
    /// [`Interpreter`](crate::Interpreter)
    /// (see [`exec_interpreter_with_ident_cache`](crate::exec_interpreter_with_ident_cache)).
    /// </div>
    ///
    /// # Errors
    ///
    /// Returns an error if the namespace or the namespace prefix could not be
    /// resolved.
    pub fn resolve(self, schemas: &Schemas) -> Result<TypeIdent, InterpreterError> {
        let Self {
            ns,
            schema,
            name,
            type_,
        } = self;

        let name = Name::new_named(name);

        let ns = match ns {
            None | Some(NamespaceIdent::Anonymous) => schemas
                .resolve_namespace(&None)
                .ok_or(InterpreterError::AnonymousNamespaceIsUndefined)?,
            Some(NamespaceIdent::Id(ns)) => ns,
            Some(NamespaceIdent::Prefix(prefix)) => schemas
                .resolve_prefix(&prefix)
                .ok_or(InterpreterError::UnknownNamespacePrefix(prefix))?,
            #[allow(clippy::unnecessary_literal_unwrap)]
            Some(NamespaceIdent::Namespace(ns)) => {
                let ns = Some(ns);

                schemas
                    .resolve_namespace(&ns)
                    .ok_or_else(|| InterpreterError::UnknownNamespace(ns.unwrap()))?
            }
        };

        let schema = match schema {
            None => schemas
                .get_namespace_info(&ns)
                .and_then(|x| {
                    if x.schemas.len() == 1 {
                        Some(x.schemas[0])
                    } else {
                        None
                    }
                })
                .or_else(|| schemas.resolve_schema(ns, name.as_str(), type_))
                .unwrap_or(SchemaId::UNKNOWN),
            Some(SchemaIdent::Id(schema)) => schema,
            Some(SchemaIdent::Name(s)) => schemas
                .schemas()
                .find(|(_, info)| matches!(&info.name, Some(name) if *name == s))
                .map_or(SchemaId::UNKNOWN, |(id, _)| *id),
            Some(SchemaIdent::Location(url)) => schemas
                .schemas()
                .find(|(_, info)| matches!(&info.location, Some(location) if *location == url))
                .map_or(SchemaId::UNKNOWN, |(id, _)| *id),
        };

        Ok(TypeIdent {
            ns,
            schema,
            name,
            type_,
        })
    }
}

impl<X> From<(IdentType, X)> for IdentQuadruple
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
        let schema = None;

        Self {
            ns,
            schema,
            name,
            type_,
        }
    }
}

impl<N, X> From<(IdentType, N, X)> for IdentQuadruple
where
    N: Into<NamespaceIdent>,
    X: Into<String>,
{
    fn from((type_, ns, name): (IdentType, N, X)) -> Self {
        Self::from((type_, Some(ns), name))
    }
}

impl<N, X> From<(IdentType, Option<N>, X)> for IdentQuadruple
where
    N: Into<NamespaceIdent>,
    X: Into<String>,
{
    fn from((type_, ns, name): (IdentType, Option<N>, X)) -> Self {
        let ns = ns.map(Into::into);
        let name = name.into();
        let schema = None;

        Self {
            ns,
            schema,
            name,
            type_,
        }
    }
}

impl<N, S, X> From<(IdentType, N, S, X)> for IdentQuadruple
where
    N: Into<NamespaceIdent>,
    S: Into<SchemaIdent>,
    X: Into<String>,
{
    fn from((type_, ns, schema, name): (IdentType, N, S, X)) -> Self {
        Self::from((type_, Some(ns), Some(schema), name))
    }
}

impl<N, S, X> From<(IdentType, Option<N>, S, X)> for IdentQuadruple
where
    N: Into<NamespaceIdent>,
    S: Into<SchemaIdent>,
    X: Into<String>,
{
    fn from((type_, ns, schema, name): (IdentType, Option<N>, S, X)) -> Self {
        Self::from((type_, ns, Some(schema), name))
    }
}

impl<N, S, X> From<(IdentType, N, Option<S>, X)> for IdentQuadruple
where
    N: Into<NamespaceIdent>,
    S: Into<SchemaIdent>,
    X: Into<String>,
{
    fn from((type_, ns, schema, name): (IdentType, N, Option<S>, X)) -> Self {
        Self::from((type_, Some(ns), schema, name))
    }
}

impl<N, S, X> From<(IdentType, Option<N>, Option<S>, X)> for IdentQuadruple
where
    N: Into<NamespaceIdent>,
    S: Into<SchemaIdent>,
    X: Into<String>,
{
    fn from((type_, ns, schema, name): (IdentType, Option<N>, Option<S>, X)) -> Self {
        let ns = ns.map(Into::into);
        let schema = schema.map(Into::into);
        let name = name.into();

        Self {
            ns,
            schema,
            name,
            type_,
        }
    }
}

/// Identifies a namespace by either it's id, it's known prefix or it's namespace.
///
/// Used in [`IdentQuadruple`].
#[derive(Debug, Clone)]
pub enum NamespaceIdent {
    /// Identifies the anonymous namespace.
    Anonymous,

    /// Use the actual id the namespace is identified with.
    Id(NamespaceId),

    /// Uses a namespace prefix to refer to a specific namespace in the schema.
    Prefix(NamespacePrefix),

    /// Uses the full namespace to refer to a specific namespace in the schema.
    Namespace(Namespace),
}

impl NamespaceIdent {
    /// Creates a new [`NamespaceIdent::Id`] instance from the passed `value`.
    #[inline]
    #[must_use]
    pub fn id(value: NamespaceId) -> Self {
        Self::Id(value)
    }

    /// Creates a new [`NamespaceIdent::Prefix`] instance from the passed `value`.
    #[inline]
    #[must_use]
    pub fn prefix<X>(value: X) -> Self
    where
        NamespacePrefix: From<X>,
    {
        Self::Prefix(NamespacePrefix::from(value))
    }

    /// Creates a new [`NamespaceIdent::Namespace`] instance from the passed `value`.
    #[inline]
    #[must_use]
    pub fn namespace<X>(value: X) -> Self
    where
        Namespace: From<X>,
    {
        Self::Namespace(Namespace::from(value))
    }
}

impl From<NamespaceId> for NamespaceIdent {
    #[inline]
    fn from(value: NamespaceId) -> Self {
        Self::Id(value)
    }
}

impl From<Namespace> for NamespaceIdent {
    #[inline]
    fn from(value: Namespace) -> Self {
        Self::Namespace(value)
    }
}

impl From<NamespacePrefix> for NamespaceIdent {
    #[inline]
    fn from(value: NamespacePrefix) -> Self {
        Self::Prefix(value)
    }
}

/// Identifies a schema by either it's id, it's name or it's location.
///
/// Used in [`IdentQuadruple`].
#[derive(Debug, Clone)]
pub enum SchemaIdent {
    /// Identify the schema by it's [`SchemaId`].
    Id(SchemaId),

    /// Identify the schema by it's name.
    Name(String),

    /// Identify the schema by it's location.
    Location(Url),
}

impl SchemaIdent {
    /// Creates a new [`SchemaIdent::Id`] instance from the passed `value`.
    #[inline]
    #[must_use]
    pub fn id(value: SchemaId) -> Self {
        Self::Id(value)
    }

    /// Creates a new [`SchemaIdent::Name`] instance from the passed `value`.
    #[inline]
    #[must_use]
    pub fn name<X>(value: X) -> Self
    where
        X: Into<String>,
    {
        Self::Name(value.into())
    }

    /// Creates a new [`SchemaIdent::Location`] instance from the passed `value`.
    #[inline]
    #[must_use]
    pub fn location<X>(value: X) -> Self
    where
        X: Into<Url>,
    {
        Self::Location(value.into())
    }
}

impl From<SchemaId> for SchemaIdent {
    #[inline]
    fn from(value: SchemaId) -> Self {
        Self::Id(value)
    }
}

impl From<String> for SchemaIdent {
    #[inline]
    fn from(value: String) -> Self {
        Self::Name(value)
    }
}

impl From<Url> for SchemaIdent {
    #[inline]
    fn from(value: Url) -> Self {
        Self::Location(value)
    }
}

impl Schemas {
    fn resolve_schema(&self, ns: NamespaceId, name: &str, type_: IdentType) -> Option<SchemaId> {
        let ns_info = self.get_namespace_info(&ns)?;

        for schema in &ns_info.schemas {
            let Some(schema_info) = self.get_schema(schema) else {
                continue;
            };

            for c in &schema_info.schema.content {
                match (type_, c) {
                    (IdentType::Element, SchemaContent::Element(x)) if matches!(&x.name, Some(n) if n == name) => {
                        return Some(*schema)
                    }
                    (IdentType::Type, SchemaContent::SimpleType(x)) if matches!(&x.name, Some(n) if n == name) => {
                        return Some(*schema)
                    }
                    (IdentType::Type, SchemaContent::ComplexType(x)) if matches!(&x.name, Some(n) if n == name) => {
                        return Some(*schema)
                    }
                    (_, _) => (),
                }
            }
        }

        None
    }
}
