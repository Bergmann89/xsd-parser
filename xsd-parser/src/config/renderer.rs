use std::any::{Any, TypeId};
use std::fmt::Debug;

use bitflags::bitflags;

use xsd_parser_types::misc::Namespace;

use crate::models::code::IdentPath;
use crate::pipeline::renderer::{RenderStep as RenderStepTrait, RenderStepType};

/// Configuration for the actual code rendering.
#[derive(Debug)]
pub struct RendererConfig {
    /// List of renderers to use for code rendering.
    pub steps: Vec<Box<dyn RenderStepConfig>>,

    /// Additional flags to control the renderer.
    pub flags: RendererFlags,

    /// Sets the traits the generated types should derive from.
    ///
    /// See [`derive`](crate::Renderer::derive) for more details.
    pub derive: Option<Vec<String>>,

    /// Name of the `alloc` crate that is used for the generated code.
    pub alloc: String,

    /// Set the traits that should be implemented by dynamic types.
    ///
    /// See [`dyn_type_traits`](crate::Renderer::dyn_type_traits) for more details.
    pub dyn_type_traits: Option<Vec<String>>,

    /// Name of the `xsd-parser-types` crate that is used for the generated code.
    pub xsd_parser_types: String,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            steps: vec![Box::new(RenderStep::Types)],
            flags: RendererFlags::empty(),
            derive: None,
            alloc: "std".into(),
            dyn_type_traits: None,
            xsd_parser_types: "xsd_parser_types".into(),
        }
    }
}

impl Clone for RendererConfig {
    fn clone(&self) -> Self {
        Self {
            steps: self.steps.iter().map(|x| x.boxed_clone()).collect(),
            flags: self.flags,
            derive: self.derive.clone(),
            alloc: self.alloc.clone(),
            dyn_type_traits: self.dyn_type_traits.clone(),
            xsd_parser_types: self.xsd_parser_types.clone(),
        }
    }
}

bitflags! {
    /// Different flags that control what code the [`Renderer`](super::Renderer)
    /// is generating.
    #[derive(Debug, Clone, Copy)]
    pub struct RendererFlags: u32 {
        /// None of the features are enabled.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/renderer/renderer_flags/schema.xsd")]
        /// ```
        ///
        /// Setting none of the flags will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/renderer/renderer_flags/expected/empty.rs")]
        /// ```
        const NONE = 0;

        /// The renderer adds documentation to the rendered code if a
        /// `xs:documentation` element was placed in the schema.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/renderer/renderer_flags/schema_with_docs.xsd")]
        /// ```
        ///
        /// Enable the `RENDER_DOCS` feature only will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/renderer/renderer_flags/expected/render_docs.rs")]
        /// ```
        const RENDER_DOCS = Self::RENDER_TYPE_DOCS.bits()
            | Self::RENDER_ELEMENT_DOCS.bits()
            | Self::RENDER_ATTRIBUTE_DOCS.bits()
            | Self::RENDER_VARIANT_DOCS.bits();

        /// The renderer adds documentation to the rendered types if a
        /// `xs:documentation` element was placed in the schema.
        ///
        /// See [`RENDER_DOCS`](Self::RENDER_DOCS) for details.
        const RENDER_TYPE_DOCS = 1 << 0;

        /// The renderer adds documentation to the rendered elements if a
        /// `xs:documentation` element was placed in the schema.
        ///
        /// See [`RENDER_DOCS`](Self::RENDER_DOCS) for details.
        const RENDER_ELEMENT_DOCS = 1 << 1;

        /// The renderer adds documentation to the rendered attributes if a
        /// `xs:documentation` element was placed in the schema.
        ///
        /// See [`RENDER_DOCS`](Self::RENDER_DOCS) for details.
        const RENDER_ATTRIBUTE_DOCS = 1 << 2;

        /// The renderer adds documentation to the rendered enum variants for
        /// `xs:enumeration` types if a `xs:documentation` element was placed
        /// in the schema.
        ///
        /// See [`RENDER_DOCS`](Self::RENDER_DOCS) for details.
        const RENDER_VARIANT_DOCS = 1 << 3;
    }
}

/// Configuration for the [`RenderSteps`](crate::pipeline::renderer::RenderStep)s
/// the [`Renderer`](crate::Renderer) should use for rendering the code.
///
/// Caution: Some render steps are incompatible to each other (e.g. only
/// one `TypesXXX` step should be used, because they render the general type
/// structure). While other render steps depend on each other (e.g. `QuickXmlXXX`
/// depends on `Types` and `NamespaceConstants`).
#[derive(Debug, Clone)]
pub enum RenderStep {
    /// Step to render the pure types.
    Types,

    /// Step to render the types with `serde-xml-rs` support.
    TypesSerdeXmlRs {
        /// Version of `serde-xml-rs` to render the code for.
        version: SerdeXmlRsVersion,
    },

    /// Step to render the types with `quick_xml` serde support.
    TypesSerdeQuickXml,

    /// Renderer to render associated methods that return the default values
    /// of the different fields of a struct.
    Defaults,

    /// Renderer to add constants for the namespaces to the generated code.
    NamespaceConstants,

    /// Renderer that adds the [`WithNamespace`](xsd_parser_types::WithNamespace) trait to
    /// the generated types.
    WithNamespaceTrait,

    /// Renderer that renders code for the `quick_xml` serializer of the
    /// different types.
    QuickXmlSerialize {
        /// Whether to add namespaces to the root element during serialization or not.
        with_namespaces: bool,

        /// Default namespace to use for the serialization.
        default_namespace: Option<Namespace>,
    },

    /// Renderer that renders code for the `quick_xml` deserializer of the
    /// different types.
    QuickXmlDeserialize {
        /// Whether to box the deserializer or not.
        ///
        /// For more details have a look at [`QuickXmlDeserializeRenderer::boxed_deserializer`](crate::pipeline::renderer::QuickXmlDeserializeRenderStep::boxed_deserializer).
        boxed_deserializer: bool,
    },
}

/// Helper trait to deal with custom render steps.
pub trait RenderStepConfig: Debug + Any {
    /// Returns a boxed clone of the current object.
    fn boxed_clone(&self) -> Box<dyn RenderStepConfig>;

    /// Creates the actual render step and returned it as a box.
    fn into_render_step(self: Box<Self>) -> Box<dyn RenderStepTrait>;

    /// Returns the type of this render step.
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::Undefined
    }

    /// Returns `true` if `self` is mutual exclusive to `other`, `false` otherwise.
    fn is_mutual_exclusive_to(&self, other: &dyn RenderStepConfig) -> bool {
        self.type_id() == other.type_id()
            || self
                .render_step_type()
                .is_mutual_exclusive_to(other.render_step_type())
    }
}

impl<X> RenderStepConfig for X
where
    X: RenderStepTrait + Clone + Any + 'static,
{
    fn render_step_type(&self) -> RenderStepType {
        X::render_step_type(self)
    }

    fn boxed_clone(&self) -> Box<dyn RenderStepConfig> {
        Box::new(self.clone())
    }

    fn into_render_step(self: Box<Self>) -> Box<dyn RenderStepTrait> {
        self
    }
}

impl RenderStepConfig for RenderStep {
    fn render_step_type(&self) -> RenderStepType {
        match self {
            Self::Types => RenderStepType::Types,
            Self::TypesSerdeXmlRs { .. } => RenderStepType::Types,
            Self::TypesSerdeQuickXml => RenderStepType::Types,
            Self::Defaults => RenderStepType::ExtraImpls,
            Self::NamespaceConstants => RenderStepType::ExtraImpls,
            Self::WithNamespaceTrait => RenderStepType::ExtraImpls,
            Self::QuickXmlSerialize { .. } => RenderStepType::ExtraImpls,
            Self::QuickXmlDeserialize { .. } => RenderStepType::ExtraImpls,
        }
    }

    fn boxed_clone(&self) -> Box<dyn RenderStepConfig> {
        Box::new(self.clone())
    }

    fn into_render_step(self: Box<Self>) -> Box<dyn RenderStepTrait> {
        use crate::pipeline::renderer::{
            DefaultsRenderStep, NamespaceConstantsRenderStep, QuickXmlDeserializeRenderStep,
            QuickXmlSerializeRenderStep, SerdeQuickXmlTypesRenderStep, SerdeXmlRsV7TypesRenderStep,
            SerdeXmlRsV8TypesRenderStep, TypesRenderStep, WithNamespaceTraitRenderStep,
        };

        match *self {
            Self::Types => Box::new(TypesRenderStep),
            Self::TypesSerdeXmlRs {
                version: SerdeXmlRsVersion::Version07AndBelow,
            } => Box::new(SerdeXmlRsV7TypesRenderStep),
            Self::TypesSerdeXmlRs {
                version: SerdeXmlRsVersion::Version08AndAbove,
            } => Box::new(SerdeXmlRsV8TypesRenderStep),
            Self::TypesSerdeQuickXml => Box::new(SerdeQuickXmlTypesRenderStep),
            Self::Defaults => Box::new(DefaultsRenderStep),
            Self::NamespaceConstants => Box::new(NamespaceConstantsRenderStep::default()),
            Self::WithNamespaceTrait => Box::new(WithNamespaceTraitRenderStep),
            Self::QuickXmlSerialize {
                with_namespaces,
                default_namespace,
            } => Box::new(QuickXmlSerializeRenderStep {
                with_namespaces,
                default_namespace,
            }),
            Self::QuickXmlDeserialize { boxed_deserializer } => {
                Box::new(QuickXmlDeserializeRenderStep { boxed_deserializer })
            }
        }
    }

    fn is_mutual_exclusive_to(&self, other: &dyn RenderStepConfig) -> bool {
        use crate::pipeline::renderer::{
            DefaultsRenderStep, NamespaceConstantsRenderStep, QuickXmlDeserializeRenderStep,
            QuickXmlSerializeRenderStep, SerdeQuickXmlTypesRenderStep, SerdeXmlRsV7TypesRenderStep,
            SerdeXmlRsV8TypesRenderStep, TypesRenderStep, WithNamespaceTraitRenderStep,
        };

        if self
            .render_step_type()
            .is_mutual_exclusive_to(other.render_step_type())
        {
            return true;
        }

        let other_id = other.type_id();
        let other = (other as &dyn Any).downcast_ref::<Self>();

        match (self, other) {
            (Self::Types, Some(Self::Types)) => true,
            (Self::TypesSerdeXmlRs { .. }, Some(Self::TypesSerdeXmlRs { .. })) => true,
            (Self::TypesSerdeQuickXml, Some(Self::TypesSerdeQuickXml)) => true,
            (Self::Defaults, Some(Self::Defaults)) => true,
            (Self::NamespaceConstants, Some(Self::NamespaceConstants)) => true,
            (Self::WithNamespaceTrait, Some(Self::WithNamespaceTrait)) => true,
            (Self::QuickXmlSerialize { .. }, Some(Self::QuickXmlSerialize { .. })) => true,
            (Self::QuickXmlDeserialize { .. }, Some(Self::QuickXmlDeserialize { .. })) => true,
            (Self::Types, None) => other_id == TypeId::of::<TypesRenderStep>(),
            (
                Self::TypesSerdeXmlRs {
                    version: SerdeXmlRsVersion::Version07AndBelow,
                },
                None,
            ) => other_id == TypeId::of::<SerdeXmlRsV7TypesRenderStep>(),
            (
                Self::TypesSerdeXmlRs {
                    version: SerdeXmlRsVersion::Version08AndAbove,
                },
                None,
            ) => other_id == TypeId::of::<SerdeXmlRsV8TypesRenderStep>(),
            (Self::TypesSerdeQuickXml, None) => {
                other_id == TypeId::of::<SerdeQuickXmlTypesRenderStep>()
            }
            (Self::Defaults, None) => other_id == TypeId::of::<DefaultsRenderStep>(),
            (Self::NamespaceConstants, None) => {
                other_id == TypeId::of::<NamespaceConstantsRenderStep>()
            }
            (Self::WithNamespaceTrait, None) => {
                other_id == TypeId::of::<WithNamespaceTraitRenderStep>()
            }
            (Self::QuickXmlSerialize { .. }, None) => {
                other_id == TypeId::of::<QuickXmlSerializeRenderStep>()
            }
            (Self::QuickXmlDeserialize { .. }, None) => {
                other_id == TypeId::of::<QuickXmlDeserializeRenderStep>()
            }
            _ => false,
        }
    }
}

impl RenderStep {
    /// Return `true` if the passed value identifies the same step, `false` otherwise.
    #[must_use]
    pub fn is_same(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Types | Self::TypesSerdeXmlRs { .. } | Self::TypesSerdeQuickXml,
                Self::Types | Self::TypesSerdeXmlRs { .. } | Self::TypesSerdeQuickXml,
            )
            | (Self::Defaults, Self::Defaults)
            | (Self::NamespaceConstants, Self::NamespaceConstants)
            | (Self::WithNamespaceTrait, Self::WithNamespaceTrait)
            | (Self::QuickXmlSerialize { .. }, Self::QuickXmlSerialize { .. })
            | (Self::QuickXmlDeserialize { .. }, Self::QuickXmlDeserialize { .. }) => true,
            (_, _) => false,
        }
    }
}

/// Version of `serde-xml-rs` to render the code for.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SerdeXmlRsVersion {
    /// Render code for `serde-xml-rs <= 0.7`.
    Version07AndBelow,

    /// Render code for `serde-xml-rs >= 0.8`.
    Version08AndAbove,
}

/// Defines which additional traits should be implemented by the generated traits
/// of dynamic types.
#[derive(Default, Debug)]
pub enum DynTypeTraits {
    /// The traits will be detected automatically.
    #[default]
    Auto,

    /// List of trait identifiers that will be implemented.
    Custom(Vec<IdentPath>),
}
