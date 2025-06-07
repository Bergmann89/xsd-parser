use bitflags::bitflags;

use crate::models::code::IdentPath;

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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RenderStep {
    /// Render to render the pure types.
    Types,

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
