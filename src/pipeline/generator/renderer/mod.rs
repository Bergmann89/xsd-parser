//! The `renderer` module contains different types that implements the [`Renderer`] trait.

use std::fmt::Debug;

use inflector::Inflector;
use quote::format_ident;

use crate::models::{
    code::{IdentPath, Module},
    meta::ModuleMeta as TypesModule,
};

use super::{data::TypeData, Config, Context};

mod defaults;
mod namespace_const;
mod quick_xml;
mod types;
mod with_namespace_trait;

pub use self::defaults::DefaultsRenderer;
pub use self::namespace_const::NamespaceConstantsRenderer;
pub use self::quick_xml::{QuickXmlDeserializeRenderer, QuickXmlSerializeRenderer};
pub use self::types::TypesRenderer;
pub use self::with_namespace_trait::WithNamespaceTraitRenderer;

/// Trait that is used to define a renderer.
///
/// A renderer is used to generate the actual code of a specific [`TypeData`].
/// The idea is that different renderers generate different code. This can be
/// used by the user to compose different renderers depending on his needs, or
/// he could even implement customized renderers on his own.
pub trait Renderer: Debug {
    /// Initialized the renderer.
    ///
    /// This is called once for each renderer when the generator is initialized.
    fn initialize(&mut self, config: &mut Config<'_>) {
        let _config = config;
    }

    /// Renders the code for the given type.
    ///
    /// This is called once for each type that needs to be rendered.
    fn render_type(&mut self, ctx: &mut Context<'_, '_>, ty: &TypeData<'_>) {
        let _cxt = ctx;
        let _ty = ty;
    }

    /// Finishes the rendering process.
    ///
    /// This is called once for each renderer after the actual rendering of the
    /// types is finished and the code generation process is being finished.
    fn finish(&mut self, config: &Config<'_>, module: &mut Module) {
        let _config = config;
        let _module = module;
    }
}

impl TypesModule {
    pub(super) fn make_ns_const(&self) -> IdentPath {
        let ident = format_ident!(
            "NS_{}",
            self.name
                .as_ref()
                .map_or_else(|| String::from("DEFAULT"), ToString::to_string)
                .to_screaming_snake_case()
        );

        IdentPath::from_parts([], ident)
    }
}
