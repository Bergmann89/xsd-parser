//! The `types` module contains all type information related types.

pub mod ident;
pub mod info;
pub mod name;
pub mod type_;

mod helper;
mod name_builder;

use std::collections::BTreeMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

pub use self::helper::{VecHelper, WithIdent};
pub use self::ident::{Ident, IdentType};
pub use self::info::{
    AnyAttributeInfo, AnyInfo, AttributeInfo, AttributesInfo, Base, ComplexInfo, CustomInfo,
    DynamicInfo, ElementInfo, ElementMode, ElementsInfo, EnumerationInfo, GroupInfo, ReferenceInfo,
    UnionInfo, UnionTypeInfo, UnionTypesInfo, VariantInfo,
};
pub use self::name::Name;
pub use self::name_builder::{NameBuilder, NameFallback};
pub use self::type_::{BuildInInfo, Type, TypeEq, TypeVariant};

use crate::schema::{Namespace, NamespaceId};

/// This structure contains information about the type and module definitions.
///
/// It is created by the [`Interpreter`](crate::interpreter::Interpreter) by reading
/// the data of a specific [`Schemas`](crate::schema::Schemas). The types of this
/// structure can be optimized further using the [`Optimizer`](crate::optimizer::Optimizer).
#[derive(Default, Debug)]
pub struct Types {
    /// Map of the different types.
    pub types: BTreeMap<Ident, Type>,

    /// Map of the different namespaces.
    pub modules: BTreeMap<NamespaceId, Module>,

    next_name_id: Arc<AtomicUsize>,
}

/// Represents a module used by type information in the [`Types`] structure.
#[derive(Debug)]
pub struct Module {
    /// Name of the module (also used as xml prefix).
    pub name: Option<Name>,

    /// Namespace of the module.
    pub namespace: Option<Namespace>,
}

impl Types {
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
    pub fn get_resolved<'a>(&'a self, ident: &'a Ident) -> Option<(&'a Ident, &'a Type)> {
        let mut visit = Vec::new();

        get_resolved_impl(self, ident, &mut visit)
    }

    /// Get the type of the passed `ident` with all single type references resolved.
    ///
    /// Like [`get_resolved`](Self::get_resolved), but instead of returning the identifier and
    /// the type it will return only the resolved type.
    #[must_use]
    pub fn get_resolved_type<'a>(&'a self, ident: &'a Ident) -> Option<&'a Type> {
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

    /// Return the [`TypeVariant`] of corresponding type for the passed identifier.
    ///
    /// This is a shorthand for `self.get(ident).map(|ty| &type.variant)`.
    #[inline]
    #[must_use]
    pub fn get_variant(&self, ident: &Ident) -> Option<&TypeVariant> {
        self.get(ident).map(|ty| &ty.variant)
    }

    /// Return the [`TypeVariant`] of corresponding type for the passed identifier.
    ///
    /// This is a shorthand for `self.get_mut(ident).map(|ty| &type.variant)`.
    #[inline]
    #[must_use]
    pub fn get_variant_mut(&mut self, ident: &Ident) -> Option<&mut TypeVariant> {
        self.get_mut(ident).map(|ty| &mut ty.variant)
    }
}

impl Deref for Types {
    type Target = BTreeMap<Ident, Type>;

    fn deref(&self) -> &Self::Target {
        &self.types
    }
}

impl DerefMut for Types {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.types
    }
}

fn get_resolved_impl<'a>(
    types: &'a Types,
    ident: &'a Ident,
    visited: &mut Vec<&'a Ident>,
) -> Option<(&'a Ident, &'a Type)> {
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

    let ty = types.get(ident)?;

    match &ty.variant {
        TypeVariant::Reference(x) if x.is_single() => {
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
