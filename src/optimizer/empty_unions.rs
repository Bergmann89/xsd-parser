use std::collections::HashSet;

use crate::types::{ReferenceInfo, Type};

use super::{get_typedefs, Optimizer};

impl Optimizer {
    /// This will remove variants of an union, if the type of the variant resolves
    /// to the same type as an other variant. In other words, the variant will be
    /// removed if the types are identical.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema. `xs:language` and `xs:string` are both
    /// type definitions for [`String`].
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/union_duplicate.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/remove_duplicate_union_variants.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/remove_duplicate_union_variants.rs")]
    /// ```
    pub fn remove_duplicate_union_variants(mut self) -> Self {
        tracing::debug!("remove_duplicate_union_variants");

        let typedefs = get_typedefs!(self);

        for type_ in self.types.types.values_mut() {
            if let Type::Union(x) = type_ {
                let mut i = 0;
                let mut types_ = HashSet::new();

                while i < x.types.len() {
                    let type_ = typedefs.resolve(&x.types[i].type_).clone();
                    if types_.insert(type_) {
                        i += 1;
                    } else {
                        x.types.remove(i);
                    }
                }
            }
        }

        self
    }

    /// This will replace an union with a simple type definition, if the union
    /// has only one variant.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/union_empty.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/remove_empty_unions.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/remove_empty_unions.rs")]
    /// ```
    pub fn remove_empty_unions(mut self) -> Self {
        tracing::debug!("remove_empty_unions");

        for type_ in self.types.types.values_mut() {
            if let Type::Union(x) = type_ {
                if x.types.len() <= 1 {
                    let base = x.types.first().map(|x| &x.type_).or(x.base.as_ident());
                    if let Some(base) = base {
                        self.typedefs = None;
                        *type_ = Type::Reference(ReferenceInfo::new(base.clone()));
                    }
                }
            }
        }

        self
    }
}
