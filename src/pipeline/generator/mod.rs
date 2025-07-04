//! Code generation pipeline for transforming resolved schema models into Rust data structures.
//!
//! This module defines the [`Generator`] and [`GeneratorFixed`] types, along with supporting
//! logic and configuration mechanisms for converting fully interpreted [`MetaTypes`] into
//! concrete Rust representations [`DataTypes`].
//!
//! The [`Generator`] allows fine-grained configuration such as boxing strategy, type naming,
//! serde support, and handling of `xs:any`/`xs:anyAttribute`. Once configured, the generator
//! can be "fixed" into a [`GeneratorFixed`] state to emit type definitions in a controlled,
//! deterministic fashion.
//!
//! Code generation is performed by walking the dependency graph of types, resolving references,
//! and emitting type-safe Rust structures including enums, structs, and aliases.
//!
//! # Example
//! ```rust,ignore
//! let data_types = Generator::new(meta_types)
//!     .with_flags(GeneratorFlags::USE_MODULES)
//!     .generate_named_types()?
//!     .finish();
//! ```

mod context;
mod data;
mod error;
mod meta;
mod state;
mod walk;

use std::collections::{BTreeMap, HashSet, VecDeque};

use proc_macro2::Ident as Ident2;
use quote::format_ident;
use tracing::instrument;

use crate::config::{BoxFlags, GeneratorFlags, TypedefMode};
use crate::models::{
    code::{format_module_ident, format_type_ident, IdentPath},
    data::{DataType, DataTypes},
    meta::{ElementMetaVariant, MetaType, MetaTypeVariant, MetaTypes},
    schema::{MaxOccurs, NamespaceId},
    Ident, IdentType, Name,
};

pub use self::context::Context;
pub use self::error::Error;
pub use self::meta::MetaData;

use self::state::{PendingType, State, TraitInfos, TypeRef};
use self::walk::Walk;

/// Configurable Rust code generator for schema-derived type information.
///
/// The [`Generator`] type provides a flexible interface to customize how Rust code
/// structures are generated from XSD-like schema models represented as [`MetaTypes`].
/// It supports configuration of type postfixes, boxing rules, serde integration, and more.
///
/// Once all configuration is applied, the generator can be "sealed" into a
/// [`GeneratorFixed`] instance using [`into_fixed`](Self::into_fixed),
/// after which only code generation (not configuration) is permitted.
#[must_use]
#[derive(Debug)]
pub struct Generator<'types> {
    meta: MetaData<'types>,
    state: State<'types>,
}

/// Finalized code generator that emits Rust types from resolved schema definitions.
///
/// [`GeneratorFixed`] is produced by sealing a [`Generator`] with [`Generator::into_fixed()`],
/// locking its configuration and enabling deterministic generation of all required types.
///
/// It offers methods to:
/// - generate a specific type
/// - generate all named types
/// - generate all available types
///
/// Once generation is complete, use [`finish`](Self::finish) to retrieve the generated
/// [`DataTypes`] output for rendering.
#[must_use]
#[derive(Debug)]
pub struct GeneratorFixed<'types> {
    state: State<'types>,
    data_types: DataTypes<'types>,
}

/* Generator */

impl<'types> Generator<'types> {
    /// Create a new code generator from the passed `types`.
    pub fn new(types: &'types MetaTypes) -> Self {
        let meta = MetaData {
            types,
            flags: GeneratorFlags::empty(),
            postfixes: [
                String::from("Type"),        // Type = 0
                String::new(),               // Group = 1
                String::from("ElementType"), // Element = 2
                String::new(),               // ElementType = 3
                String::new(),               // Attribute = 4
                String::new(),               // AttributeGroup = 5
                String::new(),               // BuildIn = 6
                String::new(),               // Enumeration = 7
            ],
            box_flags: BoxFlags::AUTO,
            typedef_mode: TypedefMode::Auto,
            any_type: None,
            any_attribute_type: None,
        };
        let state = State {
            cache: BTreeMap::new(),
            pending: VecDeque::new(),
            trait_infos: None,
        };

        Self { meta, state }
    }

    /// Set the [`BoxFlags`] flags the generator should use for generating the code.
    pub fn box_flags(mut self, value: BoxFlags) -> Self {
        self.meta.box_flags = value;

        self
    }

    /// Set the [`TypedefMode`] value the generator should use for generating the code.
    pub fn typedef_mode(mut self, value: TypedefMode) -> Self {
        self.meta.typedef_mode = value;

        self
    }

    /// Set the [`GeneratorFlags`] flags the generator should use for generating the code.
    pub fn flags(mut self, value: GeneratorFlags) -> Self {
        self.meta.flags = value;

        self
    }

    /// Set the type to use to store unstructured `xs:any` elements.
    ///
    /// If this is set, the generator will create additional fields to store
    /// unstructured XML data for elements that has `xs:any` set.
    ///
    /// # Errors
    ///
    /// Forwards the error that is thrown, if `path` could not be converted.
    pub fn any_type<P>(mut self, path: P) -> Result<Self, P::Error>
    where
        P: TryInto<IdentPath>,
    {
        self.meta.any_type = Some(path.try_into()?);

        Ok(self)
    }

    /// Set the type to use to store unstructured `xs:anyAttribute` attributes.
    ///
    /// If this is set, the generator will create additional fields to store
    /// unstructured XML attributes for elements that has `xs:anyAttribute` set.
    ///
    /// # Errors
    ///
    /// Forwards the error that is thrown, if `path` could not be converted.
    pub fn any_attribute_type<P>(mut self, path: P) -> Result<Self, P::Error>
    where
        P: TryInto<IdentPath>,
    {
        self.meta.any_attribute_type = Some(path.try_into()?);

        Ok(self)
    }

    /// Add the passed [`GeneratorFlags`] flags the generator should use for generating the code.
    pub fn with_flags(mut self, value: GeneratorFlags) -> Self {
        self.meta.flags |= value;

        self
    }

    /// Set the postfixes the generator should use for the different types.
    ///
    /// Default is `"Type"` for the [`IdentType::Type`] type and `""` for the other types.
    pub fn with_type_postfix<S: Into<String>>(mut self, type_: IdentType, postfix: S) -> Self {
        self.meta.postfixes[type_ as usize] = postfix.into();

        self
    }

    /// Add a custom implemented type to the generator.
    ///
    /// This will add a custom implemented type to the generator. These types are
    /// usually implemented and provided by the user of the generated code. The
    /// generator will just reference to the type definition and will not generate
    /// any code related to this type.
    ///
    /// # Errors
    ///
    /// Returns an error if the namespace of the passed identifier is unknown.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .with_type(Ident::type_("UserDefinedType"));
    /// ```
    pub fn with_type(mut self, ident: Ident) -> Result<Self, Error> {
        let module_ident = format_module(self.meta.types, ident.ns)?;
        let type_ident = format_ident!("{}", ident.name.to_string());

        let type_ref = TypeRef {
            ident: ident.clone(),
            module_ident,
            type_ident,
            boxed_elements: HashSet::new(),
        };
        self.state.cache.insert(ident, type_ref);

        Ok(self)
    }

    /// Will fix the generator by call [`into_fixed`](Self::into_fixed) and then
    /// [`generate_type`](GeneratorFixed::generate_type).
    ///
    /// # Errors
    ///
    /// Raises an [`Error`] if the type generation failed.
    #[instrument(err, level = "trace", skip(self))]
    pub fn generate_type(self, ident: Ident) -> Result<GeneratorFixed<'types>, Error> {
        self.into_fixed().generate_type(ident)
    }

    /// Will fix the generator by call [`into_fixed`](Self::into_fixed) and then
    /// [`generate_named_types`](GeneratorFixed::generate_named_types).
    ///
    /// # Errors
    ///
    /// Will just forward the errors from [`generate_named_types`](GeneratorFixed::generate_named_types).
    #[instrument(err, level = "trace", skip(self))]
    pub fn generate_named_types(self) -> Result<GeneratorFixed<'types>, Error> {
        self.into_fixed().generate_named_types()
    }

    /// Will fix the generator by call [`into_fixed`](Self::into_fixed) and then
    /// [`generate_all_types`](GeneratorFixed::generate_all_types).
    ///
    /// # Errors
    ///
    /// Will just forward the errors from [`generate_all_types`](GeneratorFixed::generate_all_types).
    pub fn generate_all_types(self) -> Result<GeneratorFixed<'types>, Error> {
        self.into_fixed().generate_all_types()
    }

    /// Will convert the generator into a [`GeneratorFixed`].
    ///
    /// You need to call this method if the general configuration of the generator
    /// is finished. The resulting [`GeneratorFixed`] type will only provide methods
    /// to generate data types for specific types. The underlying configuration can
    /// not be changed anymore.
    pub fn into_fixed(self) -> GeneratorFixed<'types> {
        let Self { meta, state } = self;

        let data_types = DataTypes::new(meta);

        GeneratorFixed { state, data_types }
    }
}

impl<'types> GeneratorFixed<'types> {
    /// Generate the code for the given type.
    ///
    /// This will generate the code for the passed type identifier and all
    /// dependencies of this type.
    ///
    /// # Errors
    ///
    /// Raises an [`Error`] if the type generation failed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .generate_type(Ident::type_("Root"));
    /// ```
    #[instrument(err, level = "trace", skip(self))]
    pub fn generate_type(mut self, ident: Ident) -> Result<GeneratorFixed<'types>, Error> {
        self.state
            .get_or_create_type_ref(&self.data_types.meta, &ident)?;
        self.generate_pending()?;

        Ok(self)
    }

    /// Generate the code for all types.
    ///
    /// This will generate the code for all types that are specified in
    /// the [`MetaTypes`] object passed to the generator.
    ///
    /// # Errors
    ///
    /// Raises an [`Error`] if the type generation failed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .generate_all_types();
    /// ```
    #[instrument(err, level = "trace", skip(self))]
    pub fn generate_all_types(mut self) -> Result<Self, Error> {
        for ident in self.data_types.meta.types.items.keys() {
            self.state
                .get_or_create_type_ref(&self.data_types.meta, ident)?;
        }
        self.generate_pending()?;

        Ok(self)
    }

    /// Generate the code for all named types.
    ///
    /// This will generate the code for all types with an explicit name and all
    /// dependencies of these types that are specified in the [`MetaTypes`] object
    /// passed to the generator.
    ///
    /// # Errors
    ///
    /// Raises an [`Error`] if the type generation failed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .generate_named_types();
    /// ```
    #[instrument(err, level = "trace", skip(self))]
    pub fn generate_named_types(mut self) -> Result<Self, Error> {
        for ident in self.data_types.meta.types.items.keys() {
            if ident.name.is_named() {
                self.state
                    .get_or_create_type_ref(&self.data_types.meta, ident)?;
            }
        }
        self.generate_pending()?;

        Ok(self)
    }

    /// Finish the code generation.
    ///
    /// This will return the generated data types as [`DataTypes`].
    #[instrument(level = "trace", skip(self))]
    pub fn finish(self) -> DataTypes<'types> {
        self.data_types
    }

    #[instrument(err, level = "trace", skip(self))]
    fn generate_pending(&mut self) -> Result<(), Error> {
        while let Some(args) = self.state.pending.pop_front() {
            self.generate_type_intern(args)?;
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn generate_type_intern(&mut self, data: PendingType<'types>) -> Result<(), Error> {
        let PendingType { ty, ident } = data;
        let Self { state, data_types } = self;

        let mut context = Context::new(&data_types.meta, &ident, state);
        let ty = DataType::new(ty, &mut context)?;

        data_types.items.insert(ident, ty);

        Ok(())
    }
}

impl<'types> State<'types> {
    #[instrument(level = "trace", skip(self, meta))]
    fn get_or_create_type_ref(
        &mut self,
        meta: &MetaData<'types>,
        ident: &Ident,
    ) -> Result<&TypeRef, Error> {
        if !self.cache.contains_key(ident) {
            let ty = meta
                .types
                .items
                .get(ident)
                .ok_or_else(|| Error::UnknownType(ident.clone()))?;
            let name = make_type_name(&meta.postfixes, ty, ident);
            let (module_ident, type_ident) = match &ty.variant {
                MetaTypeVariant::BuildIn(x) => (None, format_ident!("{x}")),
                MetaTypeVariant::Custom(x) => (None, format_ident!("{}", x.name())),
                _ => {
                    let use_modules = meta.flags.intersects(GeneratorFlags::USE_MODULES);
                    let module_ident =
                        format_module(meta.types, use_modules.then_some(ident.ns).flatten())?;
                    let type_ident = format_type_ident(&name, ty.display_name.as_deref());

                    (module_ident, type_ident)
                }
            };

            tracing::debug!("Queue new type generation: {ident}");

            let boxed_elements = get_boxed_elements(ident, ty, meta.types, &self.cache);
            self.pending.push_back(PendingType {
                ty,
                ident: ident.clone(),
            });

            let type_ref = TypeRef {
                ident: ident.clone(),
                type_ident,
                module_ident,
                boxed_elements,
            };

            assert!(self.cache.insert(ident.clone(), type_ref).is_none());
        }

        Ok(self.cache.get_mut(ident).unwrap())
    }
}

/* Helper */

fn get_boxed_elements<'a>(
    ident: &Ident,
    mut ty: &'a MetaType,
    types: &'a MetaTypes,
    cache: &BTreeMap<Ident, TypeRef>,
) -> HashSet<Ident> {
    if let MetaTypeVariant::ComplexType(ci) = &ty.variant {
        if let Some(type_) = ci.content.as_ref().and_then(|ident| types.items.get(ident)) {
            ty = type_;
        }
    }

    match &ty.variant {
        MetaTypeVariant::All(si) | MetaTypeVariant::Choice(si) | MetaTypeVariant::Sequence(si) => {
            si.elements
                .iter()
                .filter_map(|f| {
                    if let ElementMetaVariant::Type(type_) = &f.variant {
                        if Walk::new(types, cache).is_loop(ident, type_) {
                            return Some(f.ident.clone());
                        }
                    }

                    None
                })
                .collect()
        }
        _ => HashSet::new(),
    }
}

fn make_type_name(postfixes: &[String], ty: &MetaType, ident: &Ident) -> Name {
    if let MetaTypeVariant::Reference(ti) = &ty.variant {
        if ident.name.is_generated() && ti.type_.name.is_named() {
            let s = ti.type_.name.to_type_name();

            if ti.max_occurs > MaxOccurs::Bounded(1) {
                return Name::new_generated(format!("{s}List"));
            } else if ti.min_occurs == 0 {
                return Name::new_generated(format!("{s}Opt"));
            }
        }
    }

    let postfix = postfixes
        .get(ident.type_ as usize)
        .map_or("", |s| s.as_str());

    let s = ident.name.to_type_name();

    if s.ends_with(postfix) {
        ident.name.clone()
    } else {
        Name::new_generated(format!("{s}{postfix}"))
    }
}

fn format_module(types: &MetaTypes, ns: Option<NamespaceId>) -> Result<Option<Ident2>, Error> {
    let Some(ns) = ns else {
        return Ok(None);
    };

    let module = types.modules.get(&ns).ok_or(Error::UnknownNamespace(ns))?;
    let Some(name) = &module.name else {
        return Ok(None);
    };

    Ok(Some(format_module_ident(name)))
}
