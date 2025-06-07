use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;

use parking_lot::Mutex;
use proc_macro2::{Ident as Ident2, TokenStream};
use quote::format_ident;

use crate::config::GeneratorFlags;
use crate::models::{
    code::{IdentPath, Module, ModulePath},
    data::DataType,
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
    pub fn resolve_type_for_module(&self, ident: &IdentPath) -> TokenStream {
        ident.relative_to(&self.module_path)
    }

    /// Add using directives to the module the of the current rendered type.
    pub fn add_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let mut root = self.module.lock();
        Self::main_module(self.module_path.last(), &mut root).usings(usings);
    }

    /// Returns a mutable reference to the module of the current rendered type.
    pub fn module(&mut self) -> &mut Module {
        let root = self.module.get_mut();

        Self::main_module(self.module_path.last(), root)
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
        self.values
            .get(&TypeId::of::<K>())
            .unwrap()
            .downcast_ref::<K::Type>()
            .unwrap()
            .clone()
    }

    /// Removes any values for the specified key `K`.
    pub fn unset<K>(&mut self)
    where
        K: ValueKey,
    {
        self.values.remove(&TypeId::of::<K>());
    }

    pub(crate) fn resolve_type_for_serialize_module(&self, ident: &IdentPath) -> TokenStream {
        ident.relative_to(&self.serialize_module_path)
    }

    pub(crate) fn resolve_type_for_deserialize_module(&self, ident: &IdentPath) -> TokenStream {
        ident.relative_to(&self.deserialize_module_path)
    }

    pub(crate) fn quick_xml_serialize(&mut self) -> &mut Module {
        self.module().module_mut("quick_xml_serialize")
    }

    pub(crate) fn quick_xml_deserialize(&mut self) -> &mut Module {
        self.module().module_mut("quick_xml_deserialize")
    }

    pub(crate) fn add_quick_xml_serialize_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let mut root = self.module.lock();
        Self::main_module(self.module_path.last(), &mut root)
            .module_mut("quick_xml_serialize")
            .usings(usings);
    }

    pub(crate) fn add_quick_xml_deserialize_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let mut root = self.module.lock();
        Self::main_module(self.module_path.last(), &mut root)
            .module_mut("quick_xml_deserialize")
            .usings(usings);
    }

    pub(super) fn new(
        meta: &'a MetaData<'types>,
        data: &'a DataType<'types>,
        ident: &'a Ident,
        module: &'a mut Module,
    ) -> Self {
        let ns = meta
            .check_generator_flags(GeneratorFlags::USE_MODULES)
            .then_some(ident.ns)
            .flatten();
        let module_path = ModulePath::from_namespace(ns, meta.types.meta.types);
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

    fn main_module<'x>(ident: Option<&Ident2>, root: &'x mut Module) -> &'x mut Module {
        if let Some(ident) = ident {
            root.module_mut(ident.to_string())
        } else {
            root
        }
    }
}

impl<'types> Deref for Context<'_, 'types> {
    type Target = MetaData<'types>;

    fn deref(&self) -> &Self::Target {
        self.meta
    }
}
