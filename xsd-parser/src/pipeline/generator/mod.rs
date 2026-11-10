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
mod custom;
mod data;
mod error;
mod layout;
mod meta;
mod state;

use std::collections::BTreeMap;
use std::str::FromStr;

use quote::format_ident;
use tracing::instrument;

use crate::config::{BoxFlags, GeneratorFlags, TypedefMode};
use crate::models::{
    code::{IdentPath, ModuleIdent, ModulePath},
    data::{DataType, DataTypes, PathData},
    meta::{MetaTypeVariant, MetaTypes},
    IdentType, TypeIdent,
};

pub use self::context::Context;
pub use self::custom::{
    CustomGenerator, CustomGeneratorBox, ValueGenerator, ValueGeneratorBox, ValueGeneratorMode,
};
pub use self::error::Error;
pub use self::meta::MetaData;
pub use self::state::TypeRef;

use self::state::{PendingType, State, TraitInfos};

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
                String::from("Dyn"),         // SubstitutionElement = 9
                String::from("Dyn"),         // DynamicVariant = 10
            ],
            box_flags: BoxFlags::AUTO,
            typedef_mode: TypedefMode::Auto,
            text_type: IdentPath::from_str("::xsd_parser_types::xml::Text").unwrap(),
            mixed_type: IdentPath::from_str("::xsd_parser_types::xml::Mixed").unwrap(),
            nillable_type: IdentPath::from_str("::xsd_parser_types::xml::Nillable").unwrap(),
            any_type: IdentPath::from_str("::xsd_parser_types::xml::AnyElement").unwrap(),
            any_attributes_type: IdentPath::from_str("::xsd_parser_types::xml::AnyAttributes")
                .unwrap(),
            content_display_name: String::from("content"),
        };
        let state = State::new(types);

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

    /// Set the type to use to store unformatted text.
    ///
    /// # Errors
    ///
    /// Forwards the error that is thrown, if `path` could not be converted.
    pub fn text_type<P>(mut self, path: P) -> Result<Self, P::Error>
    where
        P: TryInto<IdentPath>,
    {
        self.meta.text_type = path.try_into()?;

        Ok(self)
    }

    /// Set the type to use to store mixed types.
    ///
    /// # Errors
    ///
    /// Forwards the error that is thrown, if `path` could not be converted.
    pub fn mixed_type<P>(mut self, path: P) -> Result<Self, P::Error>
    where
        P: TryInto<IdentPath>,
    {
        self.meta.mixed_type = path.try_into()?;

        Ok(self)
    }

    /// Set the type to use to store nillable types.
    ///
    /// # Errors
    ///
    /// Forwards the error that is thrown, if `path` could not be converted.
    pub fn nillable_type<P>(mut self, path: P) -> Result<Self, P::Error>
    where
        P: TryInto<IdentPath>,
    {
        self.meta.nillable_type = path.try_into()?;

        Ok(self)
    }

    /// Set the type to use to store unstructured `xs:any` elements.
    ///
    /// # Errors
    ///
    /// Forwards the error that is thrown, if `path` could not be converted.
    pub fn any_type<P>(mut self, path: P) -> Result<Self, P::Error>
    where
        P: TryInto<IdentPath>,
    {
        self.meta.any_type = path.try_into()?;

        Ok(self)
    }

    /// Set the type to use to store unstructured `xs:anyAttribute` attributes.
    ///
    /// # Errors
    ///
    /// Forwards the error that is thrown, if `path` could not be converted.
    pub fn any_attributes_type<P>(mut self, path: P) -> Result<Self, P::Error>
    where
        P: TryInto<IdentPath>,
    {
        self.meta.any_attributes_type = path.try_into()?;

        Ok(self)
    }

    /// Set the name to use for the generated content field of the generated types.
    pub fn content_display_name<S: Into<String>>(mut self, name: S) -> Self {
        self.meta.content_display_name = name.into();

        self
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
    ///     .with_type(TypeIdent::type_("UserDefinedType"));
    /// ```
    pub fn with_type(mut self, ident: TypeIdent) -> Result<Self, Error> {
        let module_ident = self
            .meta
            .types
            .naming
            .format_module(self.meta.types, Some(ident.ns));
        let type_ident = format_ident!("{}", ident.name.to_string());
        let path = IdentPath::from_parts(module_ident, type_ident);

        let id = self.state.loop_detection.next_id(ident.clone());
        let type_ref = TypeRef::new_fixed(id, PathData::from_path(path.clone()));

        self.state.paths.insert(path, ident.clone());
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
    pub fn generate_type(self, ident: TypeIdent) -> Result<GeneratorFixed<'types>, Error> {
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
        let Self { meta, mut state } = self;
        state.type_modules = layout::TypeModules::new(&meta);

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
    pub fn generate_type(mut self, ident: TypeIdent) -> Result<GeneratorFixed<'types>, Error> {
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
        ident: &TypeIdent,
    ) -> Result<&mut TypeRef, Error> {
        if !self.cache.contains_key(ident) {
            self.requeue_derived_types(meta, ident);
            let type_ref = self.create_type_ref(meta, ident)?;
            self.cache.insert(ident.clone(), type_ref);
        }

        self.cache
            .get_mut(ident)
            .ok_or_else(|| Error::UnknownType(ident.clone()))
    }

    /// Regenerates already generated types that derive from the dynamic type
    /// `ident`, so they pick up the trait implementation of it.
    fn requeue_derived_types(&mut self, meta: &MetaData<'types>, ident: &TypeIdent) {
        for derived in self.trait_infos.update(meta.types, ident) {
            let is_generated = self.cache.get(&derived).is_some_and(|x| !x.is_fixed);
            if !is_generated || self.pending.iter().any(|x| x.ident == derived) {
                continue;
            }

            let Some(ty) = meta.types.items.get(&derived) else {
                continue;
            };

            self.pending.push_back(PendingType { ty, ident: derived });
        }
    }

    fn create_type_ref(
        &mut self,
        meta: &MetaData<'types>,
        ident: &TypeIdent,
    ) -> Result<TypeRef, Error> {
        let id = self.loop_detection.next_id(ident.clone());
        let ty = meta
            .types
            .items
            .get(ident)
            .ok_or_else(|| Error::UnknownType(ident.clone()))?;
        let naming = &*meta.naming;
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
            MetaTypeVariant::Custom(x) if !x.alias => {
                if let Some(using) = x.include() {
                    let path = IdentPath::from_str(using)?;

                    if meta.check_generator_flags(GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS) {
                        PathData::from_path(path)
                    } else {
                        let path = IdentPath::from_ident(format_ident!("{}", x.name()));

                        PathData::from_path(path).with_using(using)
                    }
                } else {
                    let path = IdentPath::from_ident(format_ident!("{}", x.name())).with_path(None);

                    PathData::from_path(path)
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

                let public_path = IdentPath::from_parts(module_path.0.clone(), type_ident.clone());
                Self::claim_path(&mut self.paths, meta, &public_path, ident)?;
                let module_path = if let Some(module) =
                    self.type_modules
                        .allocate(meta, ident, &module_path, &type_ident)
                {
                    module_path.join(module)
                } else {
                    module_path
                };
                let path = IdentPath::from_parts(module_path.0, type_ident);

                PathData::from_path(path)
            }
        };

        tracing::debug!("Queue new type generation: {ident}");

        self.pending.push_back(PendingType {
            ty,
            ident: ident.clone(),
        });

        Ok(TypeRef::new_pending(id, path))
    }

    /// Registers `path` for `ident` and fails if another type already uses it.
    fn claim_path(
        paths: &mut BTreeMap<IdentPath, TypeIdent>,
        meta: &MetaData<'types>,
        path: &IdentPath,
        ident: &TypeIdent,
    ) -> Result<(), Error> {
        match paths.insert(path.clone(), ident.clone()) {
            Some(first) if &first != ident => Err(Error::DuplicateTypeName {
                path: path.to_string(),
                first: Self::describe_type(meta, &first),
                second: Self::describe_type(meta, ident),
            }),
            _ => Ok(()),
        }
    }

    /// Describes `ident` by its namespace and identifier type for error messages.
    fn describe_type(meta: &MetaData<'types>, ident: &TypeIdent) -> String {
        let namespace = meta
            .types
            .modules
            .get(&ident.ns)
            .and_then(|module| module.namespace.as_ref())
            .map(|ns| String::from_utf8_lossy(&ns.0).into_owned())
            .unwrap_or_default();

        format!("{{{namespace}}}{ident:-}")
    }
}
