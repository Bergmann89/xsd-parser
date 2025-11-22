use std::collections::BTreeMap;

use xsd_parser_types::misc::Namespace;

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
    items: BTreeMap<Ident, MetaType>,

    /// Map generic schemaless Idents to the schema they come from.
    schema_idents: BTreeMap<Ident, Ident>,

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
        self.get_type(ident).map(|ty| &ty.variant)
    }

    /// Return the [`MetaTypeVariant`] of corresponding type for the passed identifier.
    ///
    /// This is a shorthand for `self.get_mut(ident).map(|ty| &type.variant)`.
    #[inline]
    #[must_use]
    pub fn get_variant_mut(&mut self, ident: &Ident) -> Option<&mut MetaTypeVariant> {
        self.items.get_mut(ident).map(|ty| &mut ty.variant)
    }

    /// Iterate over all types and Idents
    pub fn iter_items(&self) -> impl Iterator<Item = (&Ident, &MetaType)> {
        self.items.iter()
    }

    /// Iterate over all types and Idents mutably
    pub fn iter_items_mut(&mut self) -> impl Iterator<Item = (&Ident, &mut MetaType)> {
        self.items.iter_mut()
    }

    /// Iterate over all Idents
    pub fn iter_type_idents(&self) -> impl Iterator<Item = &Ident> {
        self.items.keys()
    }

    /// Iterate over all types mutably
    pub fn iter_types_mut(&mut self) -> impl Iterator<Item = &mut MetaType> {
        self.items.values_mut()
    }

    /// Look up a type by Ident.
    /// Types from the schema given by the ident are preferred over types from other schemas.
    #[must_use]
    pub fn get_type(&self, ident: &Ident) -> Option<&MetaType> {
        self.items.get(ident)
            .or_else(|| {
                self.schema_idents.get(&ident.clone().with_schema(None))
                    .and_then(|i| self.items.get(i))
            })
    }

    /// Look up a type by Ident mutably.
    /// Types from the schema given by the ident are preferred over types from other schemas.
    pub fn get_type_mut<'a, 'b>(&'a mut self, mut ident: &'b Ident) -> Option<&'a mut MetaType> where 'a: 'b {
        if !self.items.contains_key(ident) {
            ident = self.schema_idents.get(&ident.clone().with_schema(None))?;
        }
        self.items.get_mut(ident)
    }

    /// Check if this exact Ident (including schema) was added.
    #[must_use]
    pub fn contains_exact_type(&self, ident: &Ident) -> bool {
        self.items.contains_key(ident)
    }

    /// Resolve this Ident to the schema where it was defined.
    /// If the provided Ident is defined in the schema file specified by `Ident::schema`,
    /// the Ident is returned unchanged.
    /// Else an otherwise identical Ident pointing to a schema where the Ident is defined is returned.
    /// In a valid schema this should be unambiguous. In schemas that define duplicate types,
    /// an arbitrary instance is selected.
    #[must_use]
    pub fn find_original_schema<'a>(&'a self, ident: &'a Ident) -> &'a Ident {
        if self.items.contains_key(ident) {
            ident
        } else {
            self.schema_idents.get(&ident.clone().with_schema(None)).unwrap_or(ident)
        }
    }

    /// Register a type
    pub fn insert_type(&mut self, ident: Ident, type_: MetaType) {
        self.schema_idents.entry(ident.clone().with_schema(None)).or_insert_with(|| ident.clone());
        self.items.insert(ident, type_);
    }
}

impl Default for MetaTypes {
    fn default() -> Self {
        Self {
            items: Default::default(),
            schema_idents: Default::default(),
            modules: Default::default(),
            schemas: Default::default(),
            naming: Box::new(Naming::default()),
        }
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

    let ident = types.find_original_schema(ident);
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
