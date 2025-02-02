//! The `types` module contains all type information related types.

pub mod custom;
pub mod ident;
pub mod info;
pub mod name;
pub mod type_;

mod helper;

use std::collections::BTreeMap;
use std::ops::Deref;
use std::ops::DerefMut;

pub use self::custom::CustomType;
pub use self::helper::{VecHelper, WithIdent};
pub use self::ident::{Ident, IdentType};
pub use self::info::{
    AnyAttributeInfo, AnyInfo, AttributeInfo, AttributesInfo, Base, ComplexInfo, DynamicInfo,
    ElementInfo, ElementMode, ElementsInfo, EnumerationInfo, GroupInfo, ReferenceInfo, UnionInfo,
    UnionTypeInfo, UnionTypesInfo, VariantInfo,
};
pub use self::name::Name;
pub use self::type_::{BuildInInfo, Type, TypeEq};

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

    next_name_id: usize,
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
    /// Create a new [`Name::Unnamed`] name by using a unique io for this name.
    pub fn make_unnamed(&mut self) -> Name {
        self.next_name_id = self.next_name_id.wrapping_add(1);

        Name::Unnamed {
            id: self.next_name_id,
            ext: None,
        }
    }

    /// Get the type of the passed `ident` with all single type references resolved.
    ///
    /// Tries to find the type specified by the passed `ident` and resolve simple
    /// type definitions to the very base type. If the type could not be found `None`
    /// is returned.
    #[must_use]
    pub fn get_resolved<'a>(&'a self, ident: &'a Ident) -> Option<&'a Type> {
        let mut visit = Vec::new();

        get_resolved(self, ident, &mut visit).map(|(_ident, ty)| ty)
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

pub(crate) fn get_resolved<'a>(
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

    match ty {
        Type::Reference(x) if x.is_single() => {
            visited.push(ident);

            let ret = match get_resolved(types, &x.type_, visited) {
                None => Some((ident, ty)),
                Some((ident, ty)) => Some((ident, ty)),
            };

            visited.pop();

            ret
        }
        _ => Some((ident, ty)),
    }
}
