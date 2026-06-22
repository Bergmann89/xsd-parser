use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::Deref;

use bit_set::BitSet;

use crate::models::meta::DeriveRelationship;
use crate::models::{
    data::PathData,
    meta::{MetaType, MetaTypeVariant, MetaTypes},
    TypeIdent,
};

/* State */

#[derive(Debug)]
pub(super) struct State<'types> {
    pub cache: BTreeMap<TypeIdent, TypeRef>,
    pub pending: VecDeque<PendingType<'types>>,
    pub trait_infos: TraitInfos,
    pub loop_detection: LoopDetection,
}

/* PendingType */

#[derive(Debug)]
pub(super) struct PendingType<'types> {
    pub ty: &'types MetaType,
    pub ident: TypeIdent,
}

/* TypeRef */

/// Reference to a type that is processed by the generator.
///
/// It mainly carries the resolved [`PathData`] of the type, which can be used
/// to reference the type from the generated code.
#[derive(Debug)]
pub struct TypeRef {
    pub(super) id: usize,

    /// Resolved path of the type.
    pub path: PathData,

    pub(super) reachable: BitSet<u64>,
}

impl TypeRef {
    pub(super) fn new_pending(id: usize, path: PathData) -> Self {
        Self {
            id,
            path,
            reachable: BitSet::default(),
        }
    }

    pub(super) fn new_fixed(id: usize, path: PathData) -> Self {
        Self {
            id,
            path,
            reachable: BitSet::default(),
        }
    }
}

/* TraitInfos */

#[derive(Debug)]
pub(super) struct TraitInfos(BTreeMap<TypeIdent, TraitInfo>);

impl TraitInfos {
    #[must_use]
    pub(super) fn empty() -> Self {
        Self(BTreeMap::new())
    }

    pub(super) fn update(&mut self, types: &MetaTypes, ident: &TypeIdent) {
        if let Some(base_type) = types.items.get(ident) {
            self.update_impl(types, ident, base_type);
        }
    }

    fn update_impl(&mut self, types: &MetaTypes, base_ident: &TypeIdent, base_ty: &MetaType) {
        let MetaTypeVariant::Dynamic(meta) = &base_ty.variant else {
            return;
        };

        for meta in meta.derived_types.values() {
            let info = self.0.entry(meta.type_.clone()).or_default();

            info.traits_to_impl.insert(base_ident.clone());

            let Some(derived_ty) = types.items.get(&meta.type_) else {
                continue;
            };

            let MetaTypeVariant::Dynamic(_) = &derived_ty.variant else {
                continue;
            };

            if base_ident.type_ == meta.type_.type_
                && meta.relationship == DeriveRelationship::DirectChild
            {
                info.traits_to_derive.insert(base_ident.clone());
            }
        }
    }
}

impl Deref for TraitInfos {
    type Target = BTreeMap<TypeIdent, TraitInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/* TraitInfo */

#[derive(Default, Debug)]
pub(super) struct TraitInfo {
    pub traits_to_impl: BTreeSet<TypeIdent>,
    pub traits_to_derive: BTreeSet<TypeIdent>,
}

/* LoopDetection */

#[derive(Debug, Default)]
pub(super) struct LoopDetection {
    pub types: Vec<TypeIdent>,
}

impl LoopDetection {
    pub(super) fn next_id(&mut self, ident: TypeIdent) -> usize {
        let ret = self.types.len();

        self.types.push(ident);

        ret
    }

    pub(super) fn get_reachable(
        &self,
        cache: &BTreeMap<TypeIdent, TypeRef>,
        ident: &TypeIdent,
    ) -> BitSet<u64> {
        let type_ref = cache.get(ident).unwrap();
        let mut reachable = BitSet::default();
        reachable.insert(type_ref.id);

        for id in type_ref.reachable.iter() {
            let ident = &self.types[id];
            let type_ref = cache.get(ident).unwrap();
            reachable.insert(id);
            reachable.union_with(&type_ref.reachable);
        }

        reachable
    }
}
