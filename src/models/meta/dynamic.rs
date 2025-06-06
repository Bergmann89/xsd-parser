//! Contains the [`DynamicMeta`] type information and all related types.

use std::hash::Hasher;

use crate::models::Ident;

use super::{MetaTypes, TypeEq};

/// Type information that contains data about dynamic types.
#[derive(Default, Debug, Clone)]
pub struct DynamicMeta {
    /// Base type of the dynamic type.
    pub type_: Option<Ident>,

    /// List of derived types.
    pub derived_types: Vec<Ident>,
}

impl TypeEq for DynamicMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            type_,
            derived_types,
        } = self;

        type_.type_hash(hasher, types);
        TypeEq::type_hash_slice(derived_types, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            type_,
            derived_types,
        } = self;

        type_.type_eq(&other.type_, types)
            && TypeEq::type_eq_iter(derived_types.iter(), other.derived_types.iter(), types)
    }
}
