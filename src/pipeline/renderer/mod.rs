//! Code rendering infrastructure for Rust type generation.
//!
//! This module defines the [`Renderer`] and supporting components responsible
//! for converting fully resolved [`DataTypes`] into structured Rust code modules.
//! It provides a flexible, composable rendering pipeline through the
//! [`RenderStep`] trait, allowing each rendering step to be added, removed, or
//! customized as needed.
//!
//! The [`Renderer`] can be extended with custom [`RenderStep`] implementations
//! or modified using configuration methods such as [`flags()`](Renderer::flags),
//! [`derive()`](Renderer::derive), and [`dyn_type_traits()`](Renderer::dyn_type_traits).
//!
//! Example usage:
//! ```rust,ignore
//! let module = Renderer::new(&types)
//!     .with_default_steps()
//!     .derive(["Debug", "Clone"])
//!     .finish();
//! ```

mod context;
mod error;
mod meta;
mod steps;

use std::fmt::{Debug, Display};
use std::str::FromStr;

use inflector::Inflector;
use quote::format_ident;

use crate::config::{DynTypeTraits, RendererFlags};
use crate::models::{
    code::{IdentPath, Module},
    data::{DataTypeVariant, DataTypes},
    meta::ModuleMeta as TypesModule,
};

pub use self::context::Context;
pub use self::error::Error;
pub use self::meta::MetaData;
pub use self::steps::{
    DefaultsRenderStep, NamespaceConstantsRenderStep, QuickXmlDeserializeRenderStep,
    QuickXmlSerializeRenderStep, SerdeQuickXmlTypesRenderStep, SerdeXmlRsV7TypesRenderStep,
    SerdeXmlRsV8TypesRenderStep, TypesRenderStep, WithNamespaceTraitRenderStep,
};

/// The [`Renderer`] is the central orchestrator for Rust code generation from
/// resolved schema types.
///
/// It allows the user to define a rendering pipeline using modular [`RenderStep`]s.
/// Each step contributes part of the code output - such as type definitions,
/// trait impls, constants, or serialization logic.
///
/// The [`Renderer`] holds configuration, shared metadata, and controls the execution
/// of rendering steps over the input [`DataTypes`].
///
/// You can chain configuration methods to adjust derive traits, dynamic trait
/// injection, and serialization support, and then call [`finish`](Renderer::finish)
/// to produce a [`Module`] ready for rendering as Rust source.
#[must_use]
#[derive(Debug)]
pub struct Renderer<'types> {
    meta: MetaData<'types>,
    steps: Vec<Box<dyn RenderStep + 'types>>,
    dyn_type_traits: DynTypeTraits,
}

impl<'types> Renderer<'types> {
    /// Create a new [`Renderer`] instance from the passed `types`.
    pub fn new(types: &'types DataTypes<'types>) -> Self {
        let meta = MetaData {
            types,
            flags: RendererFlags::empty(),
            derive: vec![format_ident!("Debug")],
            dyn_type_traits: Vec::new(),
            xsd_parser_crate: format_ident!("xsd_parser"),
        };

        Self {
            meta,
            steps: Vec::new(),
            dyn_type_traits: DynTypeTraits::Auto,
        }
    }

    /// Add a [`RenderStep`] to the renderer.
    pub fn with_step<X>(mut self, step: X) -> Self
    where
        X: RenderStep + 'types,
    {
        self.steps.push(Box::new(step));

        self
    }

    /// Add the default renderers to the generator.
    pub fn with_default_steps(self) -> Self {
        self.with_step(TypesRenderStep)
    }

    /// Remove all [`Renderer`]s from the generator.
    pub fn clear_steps(mut self) -> Self {
        self.steps.clear();

        self
    }

    /// Set the [`RendererFlags`] flags the renderer should use for rendering the code.
    pub fn flags(mut self, value: RendererFlags) -> Self {
        self.meta.flags = value;

        self
    }

    /// Set the traits the generated types should derive from.
    ///
    /// Default is `Debug`.
    ///
    /// If you want to set the derive for a single value, please have a look to
    /// [`DataType::derive`](crate::models::data::DataType::derive).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let renderer = Renderer::new(types)
    ///     .derive(["Debug", "Clone", "Eq", "PartialEq", "Ord", "PartialOrd"]);
    /// ```
    pub fn derive<I>(mut self, value: I) -> Self
    where
        I: IntoIterator,
        I::Item: Display,
    {
        self.meta.derive = value.into_iter().map(|x| format_ident!("{x}")).collect();

        self
    }

    /// Set the traits that should be implemented by dynamic types.
    ///
    /// The passed values must be valid type paths.
    ///
    /// # Errors
    ///
    /// Will raise a [`InvalidIdentifier`](Error::InvalidIdentifier) error
    /// if the passed strings does not represent a valid identifier.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .dyn_type_traits(["core::fmt::Debug", "core::any::Any"]);
    /// ```
    pub fn dyn_type_traits<I>(mut self, value: I) -> Result<Self, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let traits = value
            .into_iter()
            .map(|x| {
                let s = x.as_ref();
                IdentPath::from_str(s)
            })
            .collect::<Result<Vec<_>, _>>()?;

        self.dyn_type_traits = DynTypeTraits::Custom(traits);

        Ok(self)
    }

    /// Set the name of the `xsd-parser` create that the generator should use for
    /// generating the code.
    pub fn xsd_parser_crate<S: Display>(mut self, value: S) -> Self {
        self.meta.xsd_parser_crate = format_ident!("{value}");

        self
    }

    /// Finish the rendering process and return the resulting [`Module`].
    #[must_use]
    pub fn finish(self) -> Module {
        let mut module = Module::default();
        let Self {
            mut meta,
            mut steps,
            dyn_type_traits,
        } = self;

        meta.dyn_type_traits = match dyn_type_traits {
            DynTypeTraits::Auto => {
                let traits = meta.derive.iter().map(|x| match x.to_string().as_ref() {
                    "Debug" => IdentPath::from_str("core::fmt::Debug").unwrap(),
                    "Hash" => IdentPath::from_str("core::hash::Hash").unwrap(),
                    _ => IdentPath::from_ident(x.clone()),
                });

                let as_any = IdentPath::from_parts(
                    Some(meta.xsd_parser_crate.clone()),
                    format_ident!("AsAny"),
                );

                traits.chain(Some(as_any)).collect()
            }
            DynTypeTraits::Custom(x) => x,
        };

        for step in &mut steps {
            step.initialize(&mut meta);
        }

        for (ident, data) in &meta.types.items {
            let mut ctx = Context::new(&meta, data, ident, &mut module);

            for step in &mut steps {
                step.render_type(&mut ctx);
            }
        }

        for step in &mut steps {
            step.finish(&meta, &mut module);
        }

        module
    }
}

/// Trait that is used to define a renderer.
///
/// A render step is used to generate the actual code of a specific
/// [`DataType`](crate::models::data::DataType).
/// The idea is that different render steps generate different code. This can be
/// used by the user to compose different render steps depending on his needs, or
/// he could even implement customized steps on his own.
pub trait RenderStep: Debug {
    /// Initialized the renderer.
    ///
    /// This is called once for each renderer when the generator is initialized.
    fn initialize(&mut self, meta: &mut MetaData<'_>) {
        let _meta = meta;
    }

    /// Renders the code for the given type.
    ///
    /// This is called once for each type that needs to be rendered.
    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        let _cxt = ctx;
    }

    /// Finishes the rendering process.
    ///
    /// This is called once for each renderer after the actual rendering of the
    /// types is finished and the code generation process is being finished.
    fn finish(&mut self, meta: &MetaData<'_>, module: &mut Module) {
        let _meta = meta;
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
