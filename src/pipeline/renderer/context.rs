use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use parking_lot::Mutex;
use proc_macro2::TokenStream;
use quote::format_ident;

use crate::config::GeneratorFlags;
use crate::models::code::{IdentPath, ModuleIdent};
use crate::models::{
    code::{Module, ModulePath},
    data::{DataType, PathData},
    Ident,
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
    pub ident: &'a Ident,

    module: Mutex<&'a mut Module>,

    module_path: ModulePath,
    serialize_module_path: ModulePath,
    deserialize_module_path: ModulePath,

    values: HashMap<TypeId, Box<dyn Any>>,
}

pub trait ValueKey: Any {
    type Type: Any + Clone;
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

    /// Add using directives to the module the of the current rendered type.
    pub fn add_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let usings = self.patch_usings(usings);
        let mut root = self.module.lock();
        Self::get_current_module(&self.module_path.0, &mut root).usings(usings);
    }

    /// Add using directives to the root module.
    pub fn add_root_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let usings = self.patch_usings(usings);
        self.module.lock().usings(usings);
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
        self.values.insert(TypeId::of::<K>(), Box::new(value));
    }

    /// Get the value that was stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    pub fn get<K>(&self) -> K::Type
    where
        K: ValueKey,
    {
        self.get_ref::<K>().clone()
    }

    /// Get a reference to the value that was stored for the specified key `K`.
    ///
    /// Panics if the key was not set before.
    pub fn get_ref<K>(&self) -> &K::Type
    where
        K: ValueKey,
    {
        self.values
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
        self.values
            .get_mut(&TypeId::of::<K>())
            .unwrap()
            .downcast_mut::<K::Type>()
            .unwrap()
    }

    /// Removes any values for the specified key `K`.
    pub fn unset<K>(&mut self)
    where
        K: ValueKey,
    {
        self.values.remove(&TypeId::of::<K>());
    }

    /// Takes an iterator of usings (anything that implements `ToString`) and
    /// executes [`patch_using`](Self::patch_using) for each element.
    pub fn patch_usings<I>(&self, usings: I) -> impl Iterator<Item = String> + use<'_, 'a, I>
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        usings
            .into_iter()
            .map(move |s| self.patch_using(Cow::Owned(s.to_string())).into_owned())
    }

    /// Replaces the `alloc::` and `xsd_parser::` in the path with the configured
    /// name for the `alloc` and `xsd-parser` crate.
    ///
    /// See [`Renderer::alloc_crate`](crate::pipeline::Renderer::alloc_crate) and
    /// [`Renderer::xsd_parser_crate`](crate::pipeline::Renderer::xsd_parser_crate)
    /// for details.
    ///
    /// This should be used before you add using directives to a module manually.
    /// Normally you should use [`add_usings`](Self::add_usings) which does the
    /// patching automatically, but if you want to add code somewhere in the module
    /// tree, this might be useful.
    pub fn patch_using<'x>(&self, using: Cow<'x, str>) -> Cow<'x, str> {
        let alloc = &self.alloc_crate;
        let xsd_parser = &self.xsd_parser_crate;

        if let Some(s) = using.strip_prefix("xsd_parser::") {
            Cow::Owned(format!("{xsd_parser}::{s}"))
        } else if let Some(s) = using.strip_prefix("::xsd_parser::") {
            Cow::Owned(format!("::{xsd_parser}::{s}"))
        } else if let Some(s) = using.strip_prefix("alloc::") {
            Cow::Owned(format!("{alloc}::{s}"))
        } else if let Some(s) = using.strip_prefix("::alloc::") {
            Cow::Owned(format!("::{alloc}::{s}"))
        } else {
            using
        }
    }

    pub(crate) fn resolve_type_for_serialize_module(&self, target_type: &PathData) -> TokenStream {
        self.add_quick_xml_serialize_usings(&target_type.usings);

        target_type.resolve_relative_to(&self.serialize_module_path)
    }

    pub(crate) fn resolve_type_for_deserialize_module(
        &self,
        target_type: &PathData,
    ) -> TokenStream {
        self.add_quick_xml_deserialize_usings(&target_type.usings);

        target_type.resolve_relative_to(&self.deserialize_module_path)
    }

    pub(crate) fn quick_xml_serialize(&mut self) -> &mut Module {
        self.current_module().module_mut("quick_xml_serialize")
    }

    pub(crate) fn quick_xml_deserialize(&mut self) -> &mut Module {
        self.current_module().module_mut("quick_xml_deserialize")
    }

    pub(crate) fn add_quick_xml_serialize_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let usings = self.patch_usings(usings);

        let mut root = self.module.lock();
        Self::get_current_module(&self.module_path.0, &mut root)
            .module_mut("quick_xml_serialize")
            .usings(usings);
    }

    pub(crate) fn add_quick_xml_deserialize_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let usings = self.patch_usings(usings);

        let mut root = self.module.lock();
        Self::get_current_module(&self.module_path.0, &mut root)
            .module_mut("quick_xml_deserialize")
            .usings(usings);
    }

    pub(super) fn new(
        meta: &'a MetaData<'types>,
        data: &'a DataType<'types>,
        ident: &'a Ident,
        module: &'a mut Module,
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

            values: HashMap::new(),
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
}

impl<'types> Deref for Context<'_, 'types> {
    type Target = MetaData<'types>;

    fn deref(&self) -> &Self::Target {
        self.meta
    }
}
