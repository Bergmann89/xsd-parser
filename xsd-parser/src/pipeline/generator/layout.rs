use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::Ident;
use quote::format_ident;

use crate::config::GeneratorFlags;
use crate::models::{
    code::{ModuleIdent, ModulePath},
    Name, TypeIdent,
};

use super::MetaData;

/// Assign type modules as generated types are queued, before resolving their paths.
#[derive(Debug, Default)]
pub(super) struct TypeModules {
    modules: BTreeMap<TypeIdent, Ident>,
    occupied: BTreeSet<(ModulePath, String)>,
}

impl TypeModules {
    pub(super) fn new(meta: &MetaData<'_>) -> Self {
        let mut this = Self::default();
        if !meta.check_generator_flags(GeneratorFlags::USE_TYPE_MODULES) {
            return this;
        }

        // Namespace and schema grouping modules share the same directory tree.
        for ident in meta.types.items.keys() {
            let module = ModuleIdent::new(
                meta.types,
                ident,
                meta.check_generator_flags(GeneratorFlags::USE_NAMESPACE_MODULES),
                meta.check_generator_flags(GeneratorFlags::USE_SCHEMA_MODULES),
            );
            let path = ModulePath::from_ident(meta.types, module);
            for index in 0..path.len() {
                this.occupied.insert((
                    ModulePath(path.0[..index].iter().cloned().collect()),
                    path[index].to_string().to_lowercase(),
                ));
            }
        }
        this
    }

    pub(super) fn allocate(
        &mut self,
        meta: &MetaData<'_>,
        ident: &TypeIdent,
        path: &ModulePath,
        type_ident: &Ident,
    ) -> Option<Ident> {
        if !meta.check_generator_flags(GeneratorFlags::USE_TYPE_MODULES) {
            return None;
        }
        let name = meta
            .naming
            .format_module_ident(&Name::new_named(format!("type_{type_ident}")));
        let mut candidate = name.clone();
        let mut suffix = 2;
        while !self
            .occupied
            .insert((path.clone(), candidate.to_string().to_lowercase()))
        {
            candidate = format_ident!("{name}_{suffix}");
            suffix += 1;
        }
        self.modules.insert(ident.clone(), candidate.clone());
        Some(candidate)
    }

    pub(super) fn name(&self, ident: &TypeIdent) -> Option<&Ident> {
        self.modules.get(ident)
    }
}
