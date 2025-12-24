use std::collections::BTreeMap;

use xsd_parser_types::misc::Namespace;

use crate::models::IdentMap;
use crate::models::{
    schema::{NamespaceId, SchemaId},
    Ident, Name, Naming,
};
use crate::traits::{NameBuilder as NameBuilderTrait, Naming as NamingTrait};

use super::{MetaType, MetaTypeVariant};

/// Intermediate representation of all resolved type and module metadata.
///
/// This structure is created by the [`Interpreter`](crate::Interpreter)
/// from the parsed [`Schemas`](crate::Schemas). It contains the full set of
/// interpreted types, along with module-to-namespace mappings.
///
/// This type acts as the canonical source of truth for all derived type definitions,
/// which can be passed to the [`Optimizer`](crate::Optimizer) for post-processing,
/// or used directly by the code [`Generator`](crate::Generator).
#[derive(Debug)]
pub struct MetaTypes {
    /// Map of the different types.
    pub items: IdentMap<BTreeMap<Ident, MetaType>>,

    /// Map of the different namespaces.
    pub modules: BTreeMap<NamespaceId, ModuleMeta>,

    /// Map of the different schemas.
    pub schemas: BTreeMap<SchemaId, SchemaMeta>,

    /// Trait to control how name generation is done in `xsd_parser`.
    pub naming: Box<dyn NamingTrait>,
}

/// Represents a module used by type information in the [`MetaTypes`] structure.
#[derive(Debug)]
pub struct ModuleMeta {
    /// Name of the module.
    pub name: Option<Name>,

    /// Prefix of the modules namespace.
    pub prefix: Option<Name>,

    /// Namespace of the module.
    pub namespace: Option<Namespace>,

    /// Id of the namespace the module was created from.
    pub namespace_id: NamespaceId,

    /// Number of schemas assigned to this module/namespace.
    pub schema_count: usize,
}

/// Represents a schema used by type information in the [`MetaTypes`] structure.
#[derive(Debug)]
pub struct SchemaMeta {
    /// Name of the schema.
    pub name: Option<Name>,

    /// Id of the namespace this schema belongs to.
    pub namespace: NamespaceId,
}

impl MetaTypes {
    /// Create a new boxed [`NameBuilder`](NameBuilderTrait), that can be used to build type names.
    #[must_use]
    pub fn name_builder(&self) -> Box<dyn NameBuilderTrait> {
        self.naming.builder()
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

impl Default for MetaTypes {
    fn default() -> Self {
        Self {
            items: Default::default(),
            modules: Default::default(),
            schemas: Default::default(),
            naming: Box::new(Naming::default()),
        }
    }
}

impl ModuleMeta {
    /// Get the name or the prefix of the module.
    #[must_use]
    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref().or(self.prefix.as_ref())
    }

    /// Get the prefix or the name of the module.
    #[must_use]
    pub fn prefix(&self) -> Option<&Name> {
        self.prefix.as_ref().or(self.name.as_ref())
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

    //let ident = types.items.find_original_schema(ident);
    let ty = types.items.get(ident)?;

    match &ty.variant {
        MetaTypeVariant::Reference(x) if x.is_simple() => {
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
