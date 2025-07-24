use std::any::Any;

use bitflags::bitflags;

use crate::models::code::IdentPath;
use crate::pipeline::renderer::RenderStep as RenderStepTrait;

/// Configuration for the actual code rendering.
#[derive(Debug, Clone)]
pub struct RendererConfig {
    /// List of renderers to use for code rendering.
    pub steps: Vec<RenderStep>,

    /// Additional flags to control the renderer.
    pub flags: RendererFlags,

    /// Sets the traits the generated types should derive from.
    ///
    /// See [`derive`](crate::Renderer::derive) for more details.
    pub derive: Option<Vec<String>>,

    /// Name of the `xsd-parser` crate that is used for the generated code.
    pub xsd_parser: String,

    /// Set the traits that should be implemented by dynamic types.
    ///
    /// See [`dyn_type_traits`](crate::Renderer::dyn_type_traits) for more details.
    pub dyn_type_traits: Option<Vec<String>>,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            steps: vec![RenderStep::Types],
            flags: RendererFlags::empty(),
            derive: None,
            xsd_parser: "xsd_parser".into(),
            dyn_type_traits: None,
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
#[derive(Debug, Clone, Eq, PartialEq)]
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

    /// Renderer that adds the [`WithNamespace`](crate::WithNamespace) trait to
    /// the generated types.
    WithNamespaceTrait,

    /// Renderer that renders code for the `quick_xml` serializer of the
    /// different types.
    QuickXmlSerialize,

    /// Renderer that renders code for the `quick_xml` deserializer of the
    /// different types.
    QuickXmlDeserialize {
        /// Whether to box the deserializer or not.
        ///
        /// For more details have a look at [`QuickXmlDeserializeRenderer::boxed_deserializer`](crate::pipeline::renderer::QuickXmlDeserializeRenderStep::boxed_deserializer).
        boxed_deserializer: bool,
    },

    /// Custom defined render step that should be added to the renderer.
    Custom(CustomRenderStep),
}

/// Wrapper for a type that implements [`CustomRenderStepImpl`] to be added
/// to [`RenderStep`] as [`Custom`](RenderStep::Custom) render step.
#[derive(Debug)]
pub struct CustomRenderStep(Box<dyn CustomRenderStepImpl>);

/// Helper trait to deal with custom render steps.
pub trait CustomRenderStepImpl: RenderStepTrait + Any + 'static {
    /// Returns `true`, if `other` is equal to `self`, false otherwise.
    fn eq(&self, other: &dyn CustomRenderStepImpl) -> bool;

    /// Returns a boxed clone of the current object.
    fn clone(&self) -> Box<dyn CustomRenderStepImpl>;
}

impl CustomRenderStep {
    /// Converts this instance into a boxed [`RenderStepTrait`].
    #[must_use]
    pub fn into_boxed(self) -> Box<dyn RenderStepTrait + 'static> {
        self.0
    }
}

impl Clone for CustomRenderStep {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for CustomRenderStep {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other.0.as_ref())
    }
}

impl Eq for CustomRenderStep {}

impl<X> CustomRenderStepImpl for X
where
    X: RenderStepTrait + Clone + Eq + Any + 'static,
{
    fn eq(&self, other: &dyn CustomRenderStepImpl) -> bool {
        if let Some(other) = (other as &dyn Any).downcast_ref() {
            PartialEq::eq(self, other)
        } else {
            false
        }
    }

    fn clone(&self) -> Box<dyn CustomRenderStepImpl> {
        Box::new(self.clone())
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
            | (Self::QuickXmlSerialize, Self::QuickXmlSerialize)
            | (Self::QuickXmlDeserialize { .. }, Self::QuickXmlDeserialize { .. }) => true,
            (_, _) => false,
        }
    }

    /// Create a custom render step ([`RenderStep::Custom`]) from the passed `step`.
    pub fn custom<T>(step: T) -> Self
    where
        T: CustomRenderStepImpl,
    {
        Self::Custom(CustomRenderStep(Box::new(step)))
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
