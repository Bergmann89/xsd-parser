use std::collections::BTreeMap;
use std::sync::{atomic::AtomicUsize, Arc};

use crate::models::{
    schema::{Namespace, NamespaceId},
    Ident, Name,
};

use super::{MetaType, MetaTypeVariant, NameBuilder};

/// Intermediate representation of all resolved type and module metadata.
///
/// This structure is created by the [`Interpreter`](crate::Interpreter)
/// from the parsed [`Schemas`](crate::Schemas). It contains the full set of
/// interpreted types, along with module-to-namespace mappings.
///
/// This type acts as the canonical source of truth for all derived type definitions,
/// which can be passed to the [`Optimizer`](crate::Optimizer) for post-processing,
/// or used directly by the code [`Generator`](crate::Generator).
#[derive(Default, Debug)]
pub struct MetaTypes {
    /// Map of the different types.
    pub items: BTreeMap<Ident, MetaType>,

    /// Map of the different namespaces.
    pub modules: BTreeMap<NamespaceId, ModuleMeta>,

    next_name_id: Arc<AtomicUsize>,
}

/// Represents a module used by type information in the [`MetaTypes`] structure.
#[derive(Debug)]
pub struct ModuleMeta {
    /// Name of the module (also used as xml prefix).
    pub name: Option<Name>,

    /// Namespace of the module.
    pub namespace: Option<Namespace>,
}

impl MetaTypes {
    /// Create a new [`NameBuilder`] instance, that can be used to build type named.
    pub fn name_builder(&mut self) -> NameBuilder {
        NameBuilder::new(self.next_name_id.clone())
    }

    /// Get the identifier and the type of the passed `ident` with all single
    /// type references resolved.
    ///
    /// Tries to find the type specified by the passed `ident` and resolve simple
    /// type definitions to the very base type. If the type could not be found `None`
    /// is returned.
    #[must_use]
    pub fn get_resolved<'a>(&'a self, ident: &'a Ident) -> Option<(&'a Ident, &'a MetaType)> {
        let mut visit = Vec::new();

        get_resolved_impl(self, ident, &mut visit)
    }

    /// Get the type of the passed `ident` with all single type references resolved.
    ///
    /// Like [`get_resolved`](Self::get_resolved), but instead of returning the identifier and
    /// the type it will return only the resolved type.
    #[must_use]
    pub fn get_resolved_type<'a>(&'a self, ident: &'a Ident) -> Option<&'a MetaType> {
        self.get_resolved(ident).map(|(_ident, ty)| ty)
    }

    /// Get the type ident of the passed `ident` with all single type references resolved.
    ///
    /// Like [`get_resolved`](Self::get_resolved), but instead of returning the identifier and
    /// the type it will return only the identifier of the resolved type.
    #[must_use]
    pub fn get_resolved_ident<'a>(&'a self, ident: &'a Ident) -> Option<&'a Ident> {
        self.get_resolved(ident).map(|(ident, _ty)| ident)
    }

    /// Return the [`MetaTypeVariant`] of corresponding type for the passed identifier.
    ///
    /// This is a shorthand for `self.get(ident).map(|ty| &type.variant)`.
    #[inline]
    #[must_use]
    pub fn get_variant(&self, ident: &Ident) -> Option<&MetaTypeVariant> {
        self.items.get(ident).map(|ty| &ty.variant)
    }

    /// Return the [`MetaTypeVariant`] of corresponding type for the passed identifier.
    ///
    /// This is a shorthand for `self.get_mut(ident).map(|ty| &type.variant)`.
    #[inline]
    #[must_use]
    pub fn get_variant_mut(&mut self, ident: &Ident) -> Option<&mut MetaTypeVariant> {
        self.items.get_mut(ident).map(|ty| &mut ty.variant)
    }
}

fn get_resolved_impl<'a>(
    types: &'a MetaTypes,
    ident: &'a Ident,
    visited: &mut Vec<&'a Ident>,
) -> Option<(&'a Ident, &'a MetaType)> {
    if visited.contains(&ident) {
        let chain = visited
            .iter()
            .map(ToString::to_string)
            .chain(Some(ident.to_string()))
            .collect::<Vec<_>>()
            .join(" >> ");

        tracing::debug!("Detected type reference loop: {chain}");

        return None;
    }

    let ty = types.items.get(ident)?;

    match &ty.variant {
        MetaTypeVariant::Reference(x) if x.is_single() => {
            visited.push(ident);

            let ret = match get_resolved_impl(types, &x.type_, visited) {
                None => Some((ident, ty)),
                Some((ident, ty)) => Some((ident, ty)),
            };

            visited.pop();

            ret
        }
        _ => Some((ident, ty)),
    }
}
