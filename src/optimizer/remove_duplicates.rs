use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::types::{ReferenceInfo, Type, TypeEq, TypeVariant, Types};

use super::{Error, TypeTransformer};

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
#[doc = include_str!("../../tests/optimizer/duplicate.xsd")]
/// ```
///
/// Without this optimization this will result in the following code:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected0/remove_duplicates.rs")]
/// ```
///
/// With this optimization the following code is generated:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected1/remove_duplicates.rs")]
/// ```
#[derive(Debug)]
pub struct RemoveDuplicates;

impl TypeTransformer for RemoveDuplicates {
    type Error = super::Error;

    fn transform(&self, types: &mut Types) -> Result<(), Error> {
        use std::collections::hash_map::Entry;

        struct Value<'a> {
            type_: &'a Type,
            types: &'a Types,
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

            #[allow(clippy::mutable_key_type)]
            let mut map = HashMap::new();
            let mut idents = HashMap::new();

            for (ident, type_) in types.iter() {
                match map.entry(Value { type_, types }) {
                    Entry::Vacant(e) => {
                        if let Some(ident) = types.get_resolved_ident(ident) {
                            e.insert(ident.clone());
                        }
                    }
                    Entry::Occupied(e) => {
                        let reference_ident = e.get();
                        if !matches!(&type_.variant, TypeVariant::Reference(ti) if &ti.type_ == reference_ident)
                        {
                            idents.insert(ident.clone(), reference_ident.clone());
                        }
                    }
                }
            }

            if !idents.is_empty() {
                changed = true;
            }

            for (ident, referenced_type) in idents {
                tracing::trace!(
                    "Create reference for duplicate type: {ident} => {referenced_type}"
                );

                let ty = types.get_mut(&ident).unwrap();
                ty.variant = TypeVariant::Reference(ReferenceInfo::new(referenced_type));
            }
        }

        Ok(())
    }
}
