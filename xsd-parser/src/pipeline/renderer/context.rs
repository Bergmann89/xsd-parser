use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::hash_map::{Entry, HashMap};
use std::ops::Deref;
use std::str::FromStr;

use parking_lot::Mutex;
use proc_macro2::{Ident as Ident2, TokenStream};
use quote::format_ident;

use crate::config::GeneratorFlags;
use crate::models::code::{IdentPath, ModuleIdent};
use crate::models::{
    code::{Module, ModulePath},
    data::{DataType, PathData},
    TypeIdent,
};

use super::MetaData;

/// Context for the rendering process.
///
/// This contains different additional information and configuration that may be
/// needed by a [`Renderer`](super::Renderer) to render the actual code. It is
/// also used to collect the rendered code and add it to the corresponding module.
#[derive(Debug)]
pub struct Context<'a, 'types> {
    /// Meta information for the rendering process.
    pub meta: &'a MetaData<'types>,

    /// Data type that needs to be rendered.
    pub data: &'a DataType<'types>,

    /// Identifier of the data type that needs to be rendered.
    pub ident: &'a TypeIdent,

    /// Generic data storage for any type that implements [`ValueKey`].
    pub values: Values,

    module: Mutex<&'a mut Module>,

    module_path: ModulePath,
    serialize_module_path: ModulePath,
    deserialize_module_path: ModulePath,
}

/// Cache that can store any value.
///
/// It uses a key type to identify the value inserted to the cache. The key type
/// needs to implement [`ValueKey`].
#[derive(Default, Debug)]
pub struct Values(HashMap<TypeId, Box<dyn Any>>);

/// Trait that represents a key of a certain type that can be stored in the
/// [`Values`] cache.
pub trait ValueKey: Any {
    /// Actual value type that is stored in the [`Values`] cache.
    ///
    /// You can also use `Self` as type if you don't want to make use of a
    /// separate key type.
    type Type: Any;
}

impl<'a, 'types> Context<'a, 'types> {
    /// Resolves the passed `ident` relative to the module of the current rendered type.
    pub fn resolve_type_for_module(&self, target_type: &PathData) -> TokenStream {
        self.add_usings(&target_type.usings);

        target_type.resolve_relative_to(&self.module_path)
    }

    /// Resolves the passed identifier `path` for build-in types before it can
    /// be rendered.
    ///
    /// If [`GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS`] is set, the path is returned
    /// as it, otherwise it will return the identifier only (without the path).
    ///
    /// # Panics
    ///
    /// Panics if the passed `path` is not a valid [`IdentPath`].
    pub fn resolve_build_in(&self, path: &str) -> IdentPath {
        let path = self.patch_using(Cow::Borrowed(path));
        let path = IdentPath::from_str(&path).unwrap();

        if self.check_generator_flags(GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS) {
            path
        } else {
            let (ident, _path, _absolute) = path.into_parts();

            IdentPath::from_ident(ident)
        }
    }

    /// Resolve the passed identifier `path` before it can be rendered.
    ///
    /// If [`GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS`] is set this will
    /// returns the `path` as is, otherwise it will add a suitable using
    /// (see [`add_usings`](Context::add_usings)) and returns the identifier
    /// only (without the path).
    pub fn resolve_ident_path(&self, path: &str) -> IdentPath {
        self.resolve_ident_path_impl(path, Self::add_usings)
    }

    /// Add using directives to the module the of the current rendered type.
    pub fn add_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let usings = self.patch_usings(usings);
        let mut root = self.module.lock();
        Self::get_current_module(&self.module_path.0, &mut root).usings(false, usings);
    }

    /// Add using directives to the root module.
    pub fn add_root_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let usings = self.patch_usings(usings);
        self.module.lock().usings(false, usings);
    }

    /// Returns a mutable reference to the main module.
    ///
    /// This might be useful if you need to add code anywhere to the generated
    /// result.
    pub fn main_module(&mut self) -> &mut Module {
        self.module.get_mut()
    }

    /// Returns a mutable reference to the module of the current rendered type.
    pub fn current_module(&mut self) -> &mut Module {
        let root = self.module.get_mut();

        Self::get_current_module(&self.module_path.0, root)
    }

    /// Set a `value` for the specified key `K`.
    pub fn set<K>(&mut self, value: K::Type)
    where
        K: ValueKey,
    {
        self.values.set::<K>(value);
    }

    /// Get the value that was stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    pub fn get<K>(&self) -> K::Type
    where
        K: ValueKey,
        K::Type: Clone,
    {
        self.values.get::<K>()
    }

    /// Get a reference to the value that was stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    pub fn get_ref<K>(&self) -> &K::Type
    where
        K: ValueKey,
    {
        self.values.get_ref::<K>()
    }

    /// Get a mutable reference to the value that was stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    pub fn get_mut<K>(&mut self) -> &mut K::Type
    where
        K: ValueKey,
    {
        self.values.get_mut::<K>()
    }

    /// Get a mutable reference to the value that was stored for the specified key `K`.
    /// If no value is available a new one is created.
    pub fn get_or_create<K>(&mut self) -> &mut K::Type
    where
        K: ValueKey,
        K::Type: Default,
    {
        self.values.get_or_create::<K>()
    }

    /// Get a mutable reference to the value that was stored for the specified key `K`.
    /// If no value is available a new one is created with the provided function `f`.
    pub fn get_or_create_with<K, F>(&mut self, f: F) -> &mut K::Type
    where
        K: ValueKey,
        F: FnOnce() -> K::Type,
    {
        self.values.get_or_create_with::<K, _>(f)
    }

    /// Extracts the value stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    pub fn extract<K>(&mut self) -> K::Type
    where
        K: ValueKey,
    {
        self.values.extract::<K>()
    }

    /// Removes any values for the specified key `K`.
    pub fn unset<K>(&mut self)
    where
        K: ValueKey,
    {
        self.values.unset::<K>();
    }

    /// Takes an iterator of usings (anything that implements `ToString`) and
    /// executes [`patch_using`](Self::patch_using) for each element.
    pub fn patch_usings<I>(&self, usings: I) -> impl Iterator<Item = String> + use<'_, I>
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let alloc = &self.alloc_crate;
        let xsd_parser_types = &self.xsd_parser_types;

        usings.into_iter().map(move |s| {
            Self::patch_using_impl(alloc, xsd_parser_types, Cow::Owned(s.to_string())).into_owned()
        })
    }

    /// Replaces the `alloc::` and `xsd_parser::` in the path with the configured
    /// name for the `alloc` and `xsd-parser` crate.
    ///
    /// See [`Renderer::alloc_crate`](crate::pipeline::Renderer::alloc_crate) and
    /// [`Renderer::xsd_parser_types`](crate::pipeline::Renderer::xsd_parser_types)
    /// for details.
    ///
    /// This should be used before you add using directives to a module manually.
    /// Normally you should use [`add_usings`](Self::add_usings) which does the
    /// patching automatically, but if you want to add code somewhere in the module
    /// tree, this might be useful.
    pub fn patch_using<'x>(&self, using: Cow<'x, str>) -> Cow<'x, str> {
        Self::patch_using_impl(&self.alloc_crate, &self.xsd_parser_types, using)
    }

    pub(crate) fn resolve_root_ident_path(&self, path: &str) -> IdentPath {
        self.resolve_ident_path_impl(path, Self::add_root_usings)
    }

    pub(crate) fn resolve_type_for_serialize_module(&self, target_type: &PathData) -> TokenStream {
        self.add_quick_xml_serialize_usings(false, &target_type.usings);

        target_type.resolve_relative_to(&self.serialize_module_path)
    }

    pub(crate) fn resolve_type_for_deserialize_module(
        &self,
        target_type: &PathData,
    ) -> TokenStream {
        self.add_quick_xml_deserialize_usings(false, &target_type.usings);

        target_type.resolve_relative_to(&self.deserialize_module_path)
    }

    pub(crate) fn quick_xml_serialize(&mut self) -> &mut Module {
        self.current_module().module_mut("quick_xml_serialize")
    }

    pub(crate) fn quick_xml_deserialize(&mut self) -> &mut Module {
        self.current_module().module_mut("quick_xml_deserialize")
    }

    pub(crate) fn resolve_quick_xml_serialize_ident_path(&self, path: &str) -> IdentPath {
        self.resolve_ident_path_impl(path, |x, path| {
            x.add_quick_xml_serialize_usings(false, path);
        })
    }

    pub(crate) fn resolve_quick_xml_deserialize_ident_path(&self, path: &str) -> IdentPath {
        self.resolve_ident_path_impl(path, |x, path| {
            x.add_quick_xml_deserialize_usings(false, path);
        })
    }

    pub(crate) fn add_quick_xml_serialize_usings<I>(&self, anonymous: bool, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let usings = self.patch_usings(usings);

        let mut root = self.module.lock();
        Self::get_current_module(&self.module_path.0, &mut root)
            .module_mut("quick_xml_serialize")
            .usings(anonymous, usings);
    }

    pub(crate) fn add_quick_xml_deserialize_usings<I>(&self, anonymous: bool, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let usings = self.patch_usings(usings);

        let mut root = self.module.lock();
        Self::get_current_module(&self.module_path.0, &mut root)
            .module_mut("quick_xml_deserialize")
            .usings(anonymous, usings);
    }

    pub(super) fn new(
        meta: &'a MetaData<'types>,
        data: &'a DataType<'types>,
        ident: &'a TypeIdent,
        module: &'a mut Module,
        values: Values,
    ) -> Self {
        let module_ident = ModuleIdent::new(
            meta.types,
            ident,
            meta.check_generator_flags(GeneratorFlags::USE_NAMESPACE_MODULES),
            meta.check_generator_flags(GeneratorFlags::USE_SCHEMA_MODULES),
        );
        let module_path = ModulePath::from_ident(meta.types.meta.types, module_ident);
        let serialize_module_path = module_path
            .clone()
            .join(format_ident!("quick_xml_serialize"));
        let deserialize_module_path = module_path
            .clone()
            .join(format_ident!("quick_xml_deserialize"));

        Self {
            meta,
            data,
            ident,
            module: Mutex::new(module),

            module_path,
            serialize_module_path,
            deserialize_module_path,

            values,
        }
    }

    fn get_current_module<I>(idents: I, root: &mut Module) -> &mut Module
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let mut module = root;

        for ident in idents {
            module = module.module_mut(ident.to_string());
        }

        module
    }

    fn resolve_ident_path_impl<'x, F>(&self, path: &'x str, add_usings: F) -> IdentPath
    where
        F: FnOnce(&Self, [Cow<'x, str>; 1]),
    {
        let path = self.patch_using(Cow::Borrowed(path));
        let ret = IdentPath::from_str(&path).unwrap();

        if self.check_generator_flags(GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS) {
            ret
        } else {
            add_usings(self, [path]);
            let (ident, _path, _absolute) = ret.into_parts();

            IdentPath::from_ident(ident)
        }
    }

    fn patch_using_impl<'x>(
        alloc: &Ident2,
        xsd_parser_types: &Ident2,
        using: Cow<'x, str>,
    ) -> Cow<'x, str> {
        if let Some(s) = using.strip_prefix("xsd_parser_types::") {
            Cow::Owned(format!("{xsd_parser_types}::{s}"))
        } else if let Some(s) = using.strip_prefix("::xsd_parser_types::") {
            Cow::Owned(format!("::{xsd_parser_types}::{s}"))
        } else if let Some(s) = using.strip_prefix("alloc::") {
            Cow::Owned(format!("{alloc}::{s}"))
        } else if let Some(s) = using.strip_prefix("::alloc::") {
            Cow::Owned(format!("::{alloc}::{s}"))
        } else {
            using
        }
    }
}

impl<'types> Deref for Context<'_, 'types> {
    type Target = MetaData<'types>;

    fn deref(&self) -> &Self::Target {
        self.meta
    }
}

/* Values */

impl Values {
    /// Create a new empty [`Values`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a `value` for the specified key `K`.
    pub fn set<K>(&mut self, value: K::Type)
    where
        K: ValueKey,
    {
        self.0.insert(TypeId::of::<K>(), Box::new(value));
    }

    /// Get the value that was stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    #[must_use]
    pub fn get<K>(&self) -> K::Type
    where
        K: ValueKey,
        K::Type: Clone,
    {
        self.get_ref::<K>().clone()
    }

    /// Get a reference to the value that was stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    #[must_use]
    pub fn get_ref<K>(&self) -> &K::Type
    where
        K: ValueKey,
    {
        self.0
            .get(&TypeId::of::<K>())
            .unwrap()
            .downcast_ref::<K::Type>()
            .unwrap()
    }

    /// Get a mutable reference to the value that was stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    pub fn get_mut<K>(&mut self) -> &mut K::Type
    where
        K: ValueKey,
    {
        self.0
            .get_mut(&TypeId::of::<K>())
            .unwrap()
            .downcast_mut::<K::Type>()
            .unwrap()
    }

    /// Get a mutable reference to the value that was stored for the specified key `K`.
    /// If no value is available a new one is created.
    pub fn get_or_create<K>(&mut self) -> &mut K::Type
    where
        K: ValueKey,
        K::Type: Default,
    {
        self.get_or_create_with::<K, _>(Default::default)
    }

    /// Get a mutable reference to the value that was stored for the specified key `K`.
    /// If no value is available a new one is created with the provided function `f`.
    pub fn get_or_create_with<K, F>(&mut self, f: F) -> &mut K::Type
    where
        K: ValueKey,
        F: FnOnce() -> K::Type,
    {
        match self.0.entry(TypeId::of::<K>()) {
            Entry::Vacant(e) => e.insert(Box::new(f())),
            Entry::Occupied(e) => e.into_mut(),
        }
        .downcast_mut::<K::Type>()
        .unwrap()
    }

    /// Extracts the value stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    pub fn extract<K>(&mut self) -> K::Type
    where
        K: ValueKey,
    {
        *self
            .0
            .remove(&TypeId::of::<K>())
            .unwrap()
            .downcast::<K::Type>()
            .unwrap()
    }

    /// Removes any values for the specified key `K`.
    pub fn unset<K>(&mut self)
    where
        K: ValueKey,
    {
        self.0.remove(&TypeId::of::<K>());
    }
}
