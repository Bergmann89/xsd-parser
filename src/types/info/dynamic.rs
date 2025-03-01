//! Contains the [`DynamicInfo`] type information and all related types.

use std::hash::Hasher;

use crate::types::{Ident, TypeEq, Types};

/// Type information that contains data about dynamic types.
#[derive(Default, Debug, Clone)]
pub struct DynamicInfo {
    /// Base type of the dynamic type.
    pub type_: Option<Ident>,

    /// List of derived types.
    pub derived_types: Vec<Ident>,
}

impl TypeEq for DynamicInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self {
            type_,
            derived_types,
        } = self;

        type_.type_hash(hasher, types);
        TypeEq::type_hash_slice(derived_types, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            type_,
            derived_types,
        } = self;

        type_.type_eq(&other.type_, types)
            && TypeEq::type_eq_iter(derived_types.iter(), other.derived_types.iter(), types)
    }
}
