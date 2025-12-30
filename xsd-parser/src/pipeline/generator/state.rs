use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::Deref;

use bit_set::BitSet;

use crate::models::{
    data::PathData,
    meta::{DynamicMeta, MetaType, MetaTypeVariant, MetaTypes},
    TypeIdent,
};

/* State */

#[derive(Debug)]
pub(super) struct State<'types> {
    pub cache: BTreeMap<TypeIdent, TypeRef>,
    pub pending: VecDeque<PendingType<'types>>,
    pub trait_infos: Option<TraitInfos>,
    pub loop_detection: LoopDetection,
}

/* PendingType */

#[derive(Debug)]
pub(super) struct PendingType<'types> {
    pub ty: &'types MetaType,
    pub ident: TypeIdent,
}

/* TypeRef */

#[derive(Debug)]
pub(super) struct TypeRef {
    pub id: usize,
    pub path: PathData,
    pub reachable: BitSet<u64>,
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
    pub(super) fn new(types: &MetaTypes) -> Self {
        let mut ret = Self(BTreeMap::new());

        for (base_ident, ty) in types.items.iter() {
            let MetaTypeVariant::Dynamic(ai) = &ty.variant else {
                continue;
            };

            for type_ident in &ai.derived_types {
                ret.0
                    .entry(type_ident.clone())
                    .or_default()
                    .traits_all
                    .insert(base_ident.clone());

                #[allow(clippy::single_match)]
                match types.get_variant(type_ident) {
                    Some(MetaTypeVariant::Dynamic(DynamicMeta {
                        type_: Some(type_ident),
                        ..
                    })) => {
                        ret.0
                            .entry(type_ident.clone())
                            .or_default()
                            .traits_all
                            .insert(base_ident.clone());
                    }
                    _ => (),
                }
            }
        }

        for ident in ret.0.keys().cloned().collect::<Vec<_>>() {
            let mut traits_second_level = BTreeSet::new();

            ret.collect_traits(&ident, 0, &mut traits_second_level);

            let info = ret.0.get_mut(&ident).unwrap();
            info.traits_direct = info
                .traits_all
                .difference(&traits_second_level)
                .cloned()
                .collect();
        }

        ret
    }

    fn collect_traits(
        &self,
        ident: &TypeIdent,
        depth: usize,
        traits_second_level: &mut BTreeSet<TypeIdent>,
    ) {
        if depth > 1 {
            traits_second_level.insert(ident.clone());
        }

        if let Some(info) = self.0.get(ident) {
            for trait_ in &info.traits_all {
                self.collect_traits(trait_, depth + 1, traits_second_level);
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
    pub traits_all: BTreeSet<TypeIdent>,
    pub traits_direct: BTreeSet<TypeIdent>,
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
