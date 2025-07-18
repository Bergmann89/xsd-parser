use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::ops::Deref;

use crate::models::{
    data::PathData,
    meta::{DynamicMeta, MetaType, MetaTypeVariant, MetaTypes},
    Ident,
};

/* State */

#[derive(Debug)]
pub(super) struct State<'types> {
    pub cache: BTreeMap<Ident, TypeRef>,
    pub pending: VecDeque<PendingType<'types>>,
    pub trait_infos: Option<TraitInfos>,
}

/* PendingType */

#[derive(Debug)]
pub(super) struct PendingType<'types> {
    pub ty: &'types MetaType,
    pub ident: Ident,
}

/* TypeRef */

#[derive(Debug)]
pub(super) struct TypeRef {
    pub path: PathData,
    pub boxed_elements: HashSet<Ident>,
}

/* TraitInfos */

#[derive(Debug)]
pub(super) struct TraitInfos(BTreeMap<Ident, TraitInfo>);

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
                    Some(MetaTypeVariant::Reference(ri)) if ri.is_single() => {
                        ret.0
                            .entry(ri.type_.clone())
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
        ident: &Ident,
        depth: usize,
        traits_second_level: &mut BTreeSet<Ident>,
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
    type Target = BTreeMap<Ident, TraitInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/* TraitInfo */

#[derive(Default, Debug)]
pub(super) struct TraitInfo {
    pub traits_all: BTreeSet<Ident>,
    pub traits_direct: BTreeSet<Ident>,
}
