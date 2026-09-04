use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::Deref;

use bit_set::BitSet;

use crate::models::meta::DeriveRelationship;
use crate::models::{
    data::PathData,
    meta::{ComplexMeta, MetaType, MetaTypeVariant, MetaTypes},
    TypeIdent,
};

/* State */

#[derive(Debug)]
pub(super) struct State<'types> {
    pub cache: BTreeMap<TypeIdent, TypeRef>,
    pub pending: VecDeque<PendingType<'types>>,
    pub trait_infos: TraitInfos,
    pub loop_detection: LoopDetection,

    /// Resolved identifiers of the content types that are used by more than one
    /// complex type.
    ///
    /// Collected once by [`new`](Self::new), because
    /// [`shared_content`](Self::shared_content) is called for every complex type
    /// on every path the boxing analysis walks.
    shared_content_idents: BTreeSet<TypeIdent>,
}

impl State<'_> {
    pub(super) fn new(types: &MetaTypes) -> Self {
        let mut seen = BTreeSet::new();
        let mut shared_content_idents = BTreeSet::new();

        for ty in types.items.values() {
            let MetaTypeVariant::ComplexType(ci) = &ty.variant else {
                continue;
            };

            let Some(content) = ci
                .content
                .as_ref()
                .and_then(|content| types.get_resolved_ident(content))
            else {
                continue;
            };

            if !seen.insert(content) {
                shared_content_idents.insert(content.clone());
            }
        }

        Self {
            cache: BTreeMap::new(),
            pending: VecDeque::new(),
            trait_infos: TraitInfos::empty(),
            loop_detection: LoopDetection::default(),
            shared_content_idents,
        }
    }

    /// Resolved identifier of the content type of the complex type `ident`, if that
    /// content type is shared with other complex types.
    ///
    /// A shared content type is rendered as an own type that all owners reference,
    /// while a content type with only one owner is rendered as a type nested below
    /// that owner.
    ///
    /// Mixed content is excluded for two reasons: A mixed parent parameterizes its
    /// content type with `ComplexMeta::is_mixed`, so it can not be shared. And
    /// `Optimizer::simplify_mixed_type` clears `ComplexMeta::is_mixed` but leaves
    /// `GroupMeta::is_mixed` set, so the groups own flag may be stale. Referencing
    /// such a group would generate it with `MixedMode::Group`, wrapping every element
    /// in `Mixed<..>` on top of the `Text` element the optimizer already inserted.
    pub(super) fn shared_content<'a>(
        &self,
        types: &'a MetaTypes,
        ident: &TypeIdent,
    ) -> Option<&'a TypeIdent> {
        let MetaTypeVariant::ComplexType(ci) = types.get_variant(ident)? else {
            return None;
        };

        self.shared_content_of(types, ci)
    }

    /// Like [`shared_content`](Self::shared_content), but for a [`ComplexMeta`] the
    /// caller already has at hand.
    pub(super) fn shared_content_of<'a>(
        &self,
        types: &'a MetaTypes,
        ci: &'a ComplexMeta,
    ) -> Option<&'a TypeIdent> {
        let (ident, ty) = types.get_resolved(ci.content.as_ref()?)?;

        match &ty.variant {
            MetaTypeVariant::All(si)
            | MetaTypeVariant::Choice(si)
            | MetaTypeVariant::Sequence(si)
                if !ci.is_mixed && !si.is_mixed && self.shared_content_idents.contains(ident) =>
            {
                Some(ident)
            }
            _ => None,
        }
    }
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
