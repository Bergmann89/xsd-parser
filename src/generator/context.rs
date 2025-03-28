use std::ops::Deref;

use parking_lot::Mutex;
use proc_macro2::{Ident as Ident2, TokenStream};
use quote::format_ident;

use crate::{
    code::{IdentPath, Module, ModulePath},
    schema::NamespaceId,
    types::Ident,
};

use super::{Config, GeneratorFlags};

/// Context for the rendering process.
///
/// This contains different additional information and configuration that may be
/// needed by a [`Renderer`](super::Renderer) to render the actual code. It is
/// also used to collect the rendered code and add it to the corresponding module.
#[derive(Debug)]
pub struct Context<'a, 'types> {
    ident: &'a Ident,
    config: &'a Config<'types>,
    module: Mutex<&'a mut Module>,

    module_path: ModulePath,
    serialize_module_path: ModulePath,
    deserialize_module_path: ModulePath,
}

impl<'a, 'types> Context<'a, 'types> {
    /// Returns the namespace ID of the currently rendered type.
    pub fn current_ns(&self) -> Option<NamespaceId> {
        self.check_flags(GeneratorFlags::USE_MODULES)
            .then_some(self.ident.ns)
            .flatten()
    }

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
        ident: &'a Ident,
        config: &'a Config<'types>,
        module: &'a mut Module,
    ) -> Self {
        let ns = config
            .check_flags(GeneratorFlags::USE_MODULES)
            .then_some(ident.ns)
            .flatten();
        let module_path = ModulePath::from_namespace(ns, config.types);
        let serialize_module_path = module_path
            .clone()
            .join(format_ident!("quick_xml_serialize"));
        let deserialize_module_path = module_path
            .clone()
            .join(format_ident!("quick_xml_deserialize"));

        Self {
            ident,
            config,
            module: Mutex::new(module),

            module_path,
            serialize_module_path,
            deserialize_module_path,
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
    type Target = Config<'types>;

    fn deref(&self) -> &Self::Target {
        self.config
    }
}
