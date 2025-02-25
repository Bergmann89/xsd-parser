use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::types::{ReferenceInfo, Type, TypeEq, Types};

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
    pub fn remove_duplicates(mut self) -> Self {
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

            let types = &self.types;

            let mut map = HashMap::new();
            let mut idents = HashMap::new();

            for (ident, type_) in self.types.iter() {
                match map.entry(Value { type_, types }) {
                    Entry::Vacant(e) => {
                        e.insert(ident.clone());
                    }
                    Entry::Occupied(e) => {
                        let reference_ident = e.get();
                        if !matches!(type_, Type::Reference(ti) if &ti.type_ == reference_ident) {
                            idents.insert(ident.clone(), reference_ident.clone());
                        }
                    }
                }
            }

            if !idents.is_empty() {
                changed = true;
                self.typedefs = None;
            }

            for (ident, referenced_type) in idents {
                println!("Create reference for duplicate type: {ident} => {referenced_type}");

                self.types
                    .insert(ident, Type::Reference(ReferenceInfo::new(referenced_type)));
            }
        }

        self
    }
}
