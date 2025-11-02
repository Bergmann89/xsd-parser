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

use std::collections::btree_map::{Entry, VacantEntry};
use std::collections::{BTreeMap, VecDeque};

use quote::format_ident;
use tracing::instrument;

use crate::config::{BoxFlags, GeneratorFlags, TypedefMode};
use crate::models::{
    code::{IdentPath, ModuleIdent, ModulePath},
    data::{DataType, DataTypes, PathData},
    meta::{MetaTypeVariant, MetaTypes},
    Ident, IdentType,
};
use crate::traits::Naming;

pub use self::context::Context;
pub use self::error::Error;
pub use self::meta::MetaData;

use self::state::{LoopDetection, PendingType, State, TraitInfos, TypeRef};

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
                String::from("NotNil"),      // NillableContent = 8
                String::from("Dyn"),         // DynamicElement = 9
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
            loop_detection: LoopDetection::default(),
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
        let module_ident = self
            .meta
            .types
            .naming
            .format_module(self.meta.types, ident.ns);
        let type_ident = format_ident!("{}", ident.name.to_string());
        let path = PathData::from_path(IdentPath::from_parts(module_ident, type_ident));

        let id = self.state.loop_detection.next_id(ident.clone());
        let type_ref = TypeRef::new_fixed(id, path);

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
            .get_or_create_type_ref_mut(&self.data_types.meta, &ident)?;
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
                .get_or_create_type_ref_mut(&self.data_types.meta, ident)?;
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
                    .get_or_create_type_ref_mut(&self.data_types.meta, ident)?;
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
    fn get_or_create_type_ref_mut(
        &mut self,
        meta: &MetaData<'types>,
        ident: &Ident,
    ) -> Result<&mut TypeRef, Error> {
        match self.cache.entry(ident.clone()) {
            Entry::Occupied(e) => Ok(e.into_mut()),
            Entry::Vacant(e) => {
                let id = self.loop_detection.next_id(e.key().clone());

                Self::create_type_ref(id, &*meta.naming, &mut self.pending, e, meta, ident)
            }
        }
    }

    fn create_type_ref<'a>(
        id: usize,
        naming: &dyn Naming,
        pending: &mut VecDeque<PendingType<'types>>,
        entry: VacantEntry<'a, Ident, TypeRef>,
        meta: &MetaData<'types>,
        ident: &Ident,
    ) -> Result<&'a mut TypeRef, Error> {
        let ty = meta
            .types
            .items
            .get(ident)
            .ok_or_else(|| Error::UnknownType(ident.clone()))?;
        let name = naming.make_type_name(&meta.postfixes, ty, ident);
        let path = match &ty.variant {
            MetaTypeVariant::BuildIn(x) => {
                let path = if meta.check_generator_flags(GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS) {
                    x.absolute_ident_path()
                } else {
                    x.ident_path()
                };

                PathData::from_path(path)
            }
            MetaTypeVariant::Custom(x) => {
                let path = IdentPath::from_ident(format_ident!("{}", x.name()));

                if let Some(using) = x.include() {
                    PathData::from_path(path).with_using(using)
                } else {
                    PathData::from_path(path.with_path(None))
                }
            }
            _ => {
                let module_ident = ModuleIdent::new(
                    meta.types,
                    ident,
                    meta.check_generator_flags(GeneratorFlags::USE_NAMESPACE_MODULES),
                    meta.check_generator_flags(GeneratorFlags::USE_SCHEMA_MODULES),
                );
                let module_path = ModulePath::from_ident(meta.types, module_ident);
                let type_ident = naming.format_type_ident(&name, ty.display_name.as_deref());

                let path = IdentPath::from_parts(module_path.0, type_ident);

                PathData::from_path(path)
            }
        };

        tracing::debug!("Queue new type generation: {ident}");

        pending.push_back(PendingType {
            ty,
            ident: ident.clone(),
        });

        Ok(entry.insert(TypeRef::new_pending(id, path)))
    }
}
