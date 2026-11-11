use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::models::{
    meta::{MetaType, MetaTypeVariant, MetaTypes, ReferenceMeta, TypeEq},
    IdentType, Name, TypeIdent,
};

use super::Optimizer;

impl Optimizer {
    /// If two types are completely equal this optimization will generate the
    /// first type complete and just a type definition for the second one.
    ///
    /// <div class="warning">
    /// *Caution*
    ///
    /// Be careful with this optimization. This will compare each known
    /// type with each other type and check if the types are identical or not.
    /// This would result in a type reference for two types, even if the types
    /// itself are not logically related to each other.
    ///
    /// Furthermore this may result in typedef loops. The code generator should
    /// be able to deal with them (using a Box), but it is still risky to use it.
    /// </div>
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../../tests/optimizer/duplicate.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected0/remove_duplicates.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected1/remove_duplicates.rs")]
    /// ```
    ///
    /// This is the combination of [`remove_duplicate_types`](Self::remove_duplicate_types)
    /// and [`remove_duplicate_contents`](Self::remove_duplicate_contents).
    pub fn remove_duplicates(self) -> Self {
        self.remove_duplicates_impl(true, true)
    }

    /// Like [`remove_duplicates`](Self::remove_duplicates), but only for types
    /// that are neither the content of a complex type nor a copy of a group
    /// (`xs:group`).
    ///
    /// The first of the identical types is generated, all others become a type
    /// definition for it. Content types are not merged, so each complex type
    /// keeps its own content type. Together with
    /// [`remove_duplicate_contents`](Self::remove_duplicate_contents) this is
    /// [`remove_duplicates`](Self::remove_duplicates).
    pub fn remove_duplicate_types(self) -> Self {
        self.remove_duplicates_impl(true, false)
    }

    /// Like [`remove_duplicates`](Self::remove_duplicates), but only for content
    /// types of complex types and copies of groups (`xs:group`).
    ///
    /// The merged type gets a name that is derived from the schema, so it does
    /// not depend on the order of the types:
    /// - Identical copies of the same group are merged into a type named like
    ///   the group (`<Group>Content` if this name is already used by another
    ///   type), defined in the schema of the group. Copies of different groups
    ///   are not merged.
    /// - If one of the content types belongs to a complex type all other owners
    ///   are derived from, the content types are merged into a type named like
    ///   the content of this base type, defined in the schema of the base type.
    /// - If all owners of the content types have generated names, the first
    ///   content type is used for all owners.
    ///
    /// In all other cases the content types are not merged, because the types
    /// are not logically related to each other.
    pub fn remove_duplicate_contents(self) -> Self {
        self.remove_duplicates_impl(false, true)
    }

    fn remove_duplicates_impl(mut self, types: bool, contents: bool) -> Self {
        struct Value<'a> {
            type_: &'a MetaType,
            types: &'a MetaTypes,
        }
        impl Debug for Value<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Value").field("type_", &self.type_).finish()
            }
        }

        impl PartialEq for Value<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.type_.type_eq(other.type_, self.types)
            }
        }

        impl Eq for Value<'_> {}

        impl Hash for Value<'_> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.type_.type_hash(state, self.types);
            }
        }

        tracing::debug!("remove_duplicates");

        let mut changed = true;

        while changed {
            changed = false;

            tracing::trace!("remove_duplicates new iteration");

            let meta = &self.types;
            let owners = content_owners(meta);

            #[allow(clippy::mutable_key_type)]
            let mut map = HashMap::new();
            let mut buckets = Vec::<Vec<&TypeIdent>>::new();

            for (ident, type_) in &meta.items {
                // These aliases intentionally preserve each schema declaration.
                if matches!(&type_.variant, MetaTypeVariant::Custom(custom) if custom.alias) {
                    continue;
                }
                let index = *map.entry(Value { type_, types: meta }).or_insert_with(|| {
                    buckets.push(Vec::new());

                    buckets.len() - 1
                });

                buckets[index].push(ident);
            }

            let mut merge = Merge::default();

            for bucket in buckets.into_iter().filter(|bucket| bucket.len() > 1) {
                let (structural, plain): (Vec<_>, Vec<_>) = bucket
                    .into_iter()
                    .filter(|ident| contents || !owners.contains_key(*ident))
                    .partition(|ident| {
                        contents && (owners.contains_key(*ident) || group_of(meta, ident).is_some())
                    });

                if types {
                    merge.first(meta, &plain);
                }

                if contents {
                    merge.structural(meta, &owners, structural);
                }
            }

            if !merge.references.is_empty() {
                changed = true;
                self.typedefs = None;
            }

            for (ident, type_) in merge.new_types {
                tracing::trace!("Create merged type: {ident}");

                self.types.items.insert(ident, type_);
            }

            for (ident, referenced_type) in merge.references {
                tracing::trace!(
                    "Create reference for duplicate type: {ident} => {referenced_type}"
                );

                let ty = self.types.items.get_mut(&ident).unwrap();
                ty.variant = MetaTypeVariant::Reference(ReferenceMeta::new(referenced_type));
            }
        }

        self
    }
}

/// Changes that are collected for one iteration of
/// [`remove_duplicates`](Optimizer::remove_duplicates).
#[derive(Default)]
struct Merge {
    new_types: BTreeMap<TypeIdent, MetaType>,
    references: BTreeMap<TypeIdent, TypeIdent>,
}

impl Merge {
    /// Uses the first of the `idents` for all others.
    fn first(&mut self, types: &MetaTypes, idents: &[&TypeIdent]) {
        let Some((first, others)) = idents.split_first() else {
            return;
        };
        let Some(target) = types.get_resolved_ident(first) else {
            return;
        };

        for ident in others {
            self.reference(types, ident, target);
        }
    }

    /// Merges the content types and group copies in `idents` following the
    /// rules described in [`remove_duplicate_contents`](Optimizer::remove_duplicate_contents).
    fn structural(
        &mut self,
        types: &MetaTypes,
        owners: &BTreeMap<TypeIdent, Vec<TypeIdent>>,
        idents: Vec<&TypeIdent>,
    ) {
        let mut groups = BTreeMap::<&TypeIdent, Vec<&TypeIdent>>::new();
        let mut others = Vec::new();

        for ident in idents {
            match group_of(types, ident) {
                Some(group) => groups.entry(group).or_default().push(ident),
                None => others.push(ident),
            }
        }

        for (group, idents) in groups {
            let name = group.name.as_str();

            self.merge_into(
                types,
                &[
                    merged_ident(group, name.to_owned()),
                    merged_ident(group, format!("{name}Content")),
                ],
                &idents,
            );
        }

        let mut members = Vec::new();
        for ident in &others {
            for owner in owners.get(*ident).into_iter().flatten() {
                members.push((owner, *ident));
            }
        }

        // Owners that only differ in their content are merged themselves as
        // soon as their contents are merged, so the name of the content is
        // never visible.
        if let Some(((first, _), rest)) = members.split_first() {
            if rest
                .iter()
                .all(|(owner, _)| equal_except_content(types, first, owner))
            {
                self.first(types, &others);

                return;
            }
        }

        let base = members.iter().find(|(base, _)| {
            members
                .iter()
                .all(|(owner, _)| owner == base || derives_from(types, owner, base))
        });

        if let Some((base, _)) = base {
            let target = merged_ident(base, format!("{}Content", base.name));

            self.merge_into(types, &[target], &others);
        } else if members.iter().all(|(owner, _)| owner.name.is_generated()) {
            self.first(types, &others);
        }
    }

    /// Merges all `idents` into a new type identified by the first of the
    /// `candidates` that is not used by another type yet.
    ///
    /// Nothing is merged if all candidates are already used by other types.
    fn merge_into(&mut self, types: &MetaTypes, candidates: &[TypeIdent], idents: &[&TypeIdent]) {
        // Generated and named identifiers are not equal, so the names are
        // compared to detect conflicts in the generated code.
        let used = |target: &TypeIdent| {
            !idents.contains(&target)
                && types
                    .items
                    .keys()
                    .chain(self.new_types.keys())
                    .any(|ident| same_name(ident, target))
        };
        let Some(target) = candidates.iter().find(|target| !used(target)) else {
            tracing::debug!("Unable to merge duplicate types: all names are already used");

            return;
        };

        if idents.len() < 2 && !idents.contains(&target) {
            return;
        }

        if !idents.contains(&target) {
            let Some(type_) = idents.first().and_then(|ident| types.items.get(*ident)) else {
                return;
            };

            let mut type_ = type_.clone();
            type_.schema = Some(target.schema);

            self.new_types.insert(target.clone(), type_);
        }

        for ident in idents {
            self.reference(types, ident, target);
        }
    }

    fn reference(&mut self, types: &MetaTypes, ident: &TypeIdent, target: &TypeIdent) {
        if ident == target {
            return;
        }

        if matches!(types.get_variant(ident), Some(MetaTypeVariant::Reference(ti)) if ti.type_.eq(target))
        {
            return;
        }

        self.references.insert(ident.clone(), target.clone());
    }
}

/// Returns the identifier of a merged type called `name` that is defined in the
/// same schema as `origin`.
///
/// The name is derived from the schema, but it is still a generated one, like
/// the names of the merged types, so the type is only rendered if it is used.
fn merged_ident(origin: &TypeIdent, name: String) -> TypeIdent {
    TypeIdent {
        ns: origin.ns,
        schema: origin.schema,
        name: Name::new_generated(name),
        type_: IdentType::Type,
    }
}

/// Returns `true` if `a` and `b` would be generated with the same name.
fn same_name(a: &TypeIdent, b: &TypeIdent) -> bool {
    a.ns == b.ns && a.type_ == b.type_ && a.name.as_str() == b.name.as_str()
}

/// Returns the complex types for each content type, by the resolved identifier
/// of the content type.
fn content_owners(types: &MetaTypes) -> BTreeMap<TypeIdent, Vec<TypeIdent>> {
    let mut ret = BTreeMap::<TypeIdent, Vec<TypeIdent>>::new();

    for (ident, type_) in &types.items {
        let MetaTypeVariant::ComplexType(ci) = &type_.variant else {
            continue;
        };

        if let Some(content) = ci
            .content
            .as_ref()
            .and_then(|content| types.get_resolved_ident(content))
        {
            ret.entry(content.clone()).or_default().push(ident.clone());
        }
    }

    ret
}

/// Returns `true` if the complex types `a` and `b` are equal if their content
/// types are ignored.
fn equal_except_content(types: &MetaTypes, a: &TypeIdent, b: &TypeIdent) -> bool {
    let (Some(a), Some(b)) = (types.items.get(a), types.items.get(b)) else {
        return false;
    };
    let (MetaTypeVariant::ComplexType(ci_a), MetaTypeVariant::ComplexType(_)) =
        (&a.variant, &b.variant)
    else {
        return false;
    };

    let mut b = b.clone();
    if let MetaTypeVariant::ComplexType(ci_b) = &mut b.variant {
        ci_b.content.clone_from(&ci_a.content);
    }

    a.type_eq(&b, types)
}

/// Returns the group (`xs:group`) the type `ident` was copied from.
fn group_of<'a>(types: &'a MetaTypes, ident: &TypeIdent) -> Option<&'a TypeIdent> {
    match types.get_variant(ident)? {
        MetaTypeVariant::All(gi) | MetaTypeVariant::Choice(gi) | MetaTypeVariant::Sequence(gi) => {
            gi.group.as_ref()
        }
        _ => None,
    }
}

/// Returns `true` if the complex type `ident` is directly or indirectly derived
/// from `base`.
fn derives_from(types: &MetaTypes, ident: &TypeIdent, base: &TypeIdent) -> bool {
    let mut current = ident;

    // The depth limit protects against invalid circular base types.
    for _ in 0..64 {
        let Some(MetaTypeVariant::ComplexType(ci)) = types.get_variant(current) else {
            return false;
        };
        let Some(next) = ci
            .base
            .as_ident()
            .and_then(|next| types.get_resolved_ident(next))
        else {
            return false;
        };

        if next == base {
            return true;
        }

        current = next;
    }

    false
}
