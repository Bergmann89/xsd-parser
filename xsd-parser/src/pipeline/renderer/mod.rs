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
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::config::{DynTypeTraits, RendererFlags};
use crate::models::{
    code::{IdentPath, Module},
    data::{DataTypeVariant, DataTypes, Occurs, PathData},
    meta::ModuleMeta,
};

pub use self::context::{Context, ValueKey, Values};
pub use self::error::Error;
pub use self::meta::MetaData;
pub use self::steps::{
    DefaultsRenderStep, NamespaceConstantsRenderStep, NamespaceSerialization,
    PrefixConstantsRenderStep, QuickXmlDeserializeRenderStep, QuickXmlSerializeRenderStep,
    SerdeQuickXmlTypesRenderStep, SerdeXmlRsV7TypesRenderStep, SerdeXmlRsV8TypesRenderStep,
    TypesRenderStep, WithNamespaceTraitRenderStep,
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
            derive: vec![IdentPath::from_ident(format_ident!("Debug"))],
            dyn_type_traits: Vec::new(),
            alloc_crate: format_ident!("std"),
            xsd_parser_types: format_ident!("xsd_parser_types"),
        };

        Self {
            meta,
            steps: Vec::new(),
            dyn_type_traits: DynTypeTraits::Auto,
        }
    }

    /// Add a [`RenderStep`] to the renderer.
    pub fn with_step<X>(self, step: X) -> Self
    where
        X: RenderStep + 'types,
    {
        self.with_step_boxed(Box::new(step))
    }

    /// Add an already boxed [`RenderStep`] to the renderer.
    pub fn with_step_boxed(mut self, step: Box<dyn RenderStep + 'types>) -> Self {
        self.steps.push(step);

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
        self.meta.derive = value
            .into_iter()
            .map(|x| IdentPath::from_str(&x.to_string()).expect("Invalid identifier path"))
            .collect();

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
    ///     .dyn_type_traits(["::core::fmt::Debug", "::core::any::Any"]);
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

    /// Set the name of the `alloc` create that the generator should use for
    /// generating the code.
    ///
    /// This is useful if the `alloc` create can not be resolved by the default
    /// name in your environment. You can just set a name that suites your needs.
    ///
    /// By default `std` is used as `alloc` crate.
    pub fn alloc_crate<S: Display>(mut self, value: S) -> Self {
        self.meta.alloc_crate = format_ident!("{value}");

        self
    }

    /// Set the name of the `xsd-parser-types` create that the generator should use for
    /// generating the code.
    ///
    /// This is useful if the `xsd-parser-types` create can not be resolved by the default
    /// name in your environment. You can just set a name that suites your needs.
    pub fn xsd_parser_types<S: Display>(mut self, value: S) -> Self {
        self.meta.xsd_parser_types = format_ident!("{value}");

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
                    "Debug" => IdentPath::from_str("::core::fmt::Debug").unwrap(),
                    "Hash" => IdentPath::from_str("::core::hash::Hash").unwrap(),
                    _ => x.clone(),
                });

                let as_any =
                    IdentPath::from_parts([meta.xsd_parser_types.clone()], format_ident!("AsAny"));

                traits.chain(Some(as_any)).collect()
            }
            DynTypeTraits::Custom(x) => x,
        };

        for step in &mut steps {
            step.initialize(&mut meta);
        }

        let mut values = Values::new();
        for (ident, data) in &meta.types.items {
            let mut ctx = Context::new(&meta, data, ident, &mut module, values);

            for step in &mut steps {
                step.render_type(&mut ctx);
            }

            values = ctx.values;
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
    /// Returns the type of the render step.
    fn render_step_type(&self) -> RenderStepType;

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

/// Defines the type of the render step.
///
/// This defines the type of a render steps and is used to manage mutually
/// exclusive render steps. For example the renderer pipeline should only
/// contain one single render step of type [`Types`](RenderStepType::Types).
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum RenderStepType {
    /// Render step that renders the actual types defined in the schema.
    /// This type of render step should only exists once in the whole renderer pipeline.
    Types,

    /// Render step that renders additional type definitions they are not conflicting
    /// with the actual types defined in the schema.
    ExtraTypes,

    /// Render step that renders additional implementation blocks or trait implementations
    /// for types defined in a different render step.
    ExtraImpls,

    /// The type of this render step is undefined.
    /// If you are implementing a custom render step and you are not sure what type
    /// to use, then use this one.
    #[default]
    Undefined,
}

impl RenderStepType {
    /// Returns `true` if the two types are mutual exclusive to each other, `false` otherwise.
    #[must_use]
    pub fn is_mutual_exclusive_to(&self, other: Self) -> bool {
        matches!((self, other), (Self::Types, Self::Types))
    }
}

impl ModuleMeta {
    pub(super) fn make_ns_const(&self) -> Option<PathData> {
        self.namespace.as_ref()?;
        let name = self.name().map_or_else(
            || format!("UNNAMED_{}", self.namespace_id.0),
            |name| name.as_str().to_screaming_snake_case(),
        );
        let ident = format_ident!("NS_{name}");
        let path = IdentPath::from_parts([], ident);

        Some(PathData::from_path(path))
    }

    pub(super) fn make_prefix_const(&self) -> Option<PathData> {
        self.namespace.as_ref()?;
        let name = self.name()?;
        let name = name.as_str().to_screaming_snake_case();
        let ident = format_ident!("PREFIX_{name}");
        let path = IdentPath::from_parts([], ident);

        Some(PathData::from_path(path))
    }
}

impl Occurs {
    /// Wrapped the passed type `ident` into a suitable rust type depending on
    /// the occurrence and the need of indirection (boxing).
    ///
    /// # Examples
    /// - `Occurs::Single` will return the type as is, or as `Box<T>`
    /// - `Occurs::Optional` will return the type as `Option<T>`
    /// - `Occurs::DynamicList` will return the type as `Vec<T>`
    /// - `Occurs::StaticList` will return the type as array `[T; SIZE]`
    #[must_use]
    pub fn make_type(
        self,
        ctx: &Context<'_, '_>,
        ident: &TokenStream,
        need_indirection: bool,
    ) -> Option<TokenStream> {
        match self {
            Self::None => None,
            Self::Single if need_indirection => {
                let box_ = ctx.resolve_build_in("::alloc::boxed::Box");

                Some(quote! { #box_<#ident> })
            }
            Self::Single => Some(quote! { #ident }),
            Self::Optional if need_indirection => {
                let box_ = ctx.resolve_build_in("::alloc::boxed::Box");
                let option = ctx.resolve_build_in("::core::option::Option");

                Some(quote! { #option<#box_<#ident>> })
            }
            Self::Optional => {
                let option = ctx.resolve_build_in("::core::option::Option");

                Some(quote! { #option<#ident> })
            }
            Self::DynamicList => {
                let vec = ctx.resolve_build_in("::alloc::vec::Vec");

                Some(quote! { #vec<#ident> })
            }
            Self::StaticList(sz) if need_indirection => {
                let box_ = ctx.resolve_build_in("::alloc::boxed::Box");

                Some(quote! { [#box_<#ident>; #sz] })
            }
            Self::StaticList(sz) => Some(quote! { [#ident; #sz] }),
        }
    }
}
