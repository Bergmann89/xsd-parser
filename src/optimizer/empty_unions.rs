use std::collections::HashSet;

use crate::types::{type_::SimpleTypeVariant, ReferenceInfo, Types};

use super::TypeTransformer;

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
#[derive(Debug)]
pub struct RemoveDuplicateUnionVariants;

impl TypeTransformer for RemoveDuplicateUnionVariants {
    type Error = super::Error;

    fn transform(&self, types: &mut Types) -> Result<(), super::Error> {
        tracing::debug!("remove_duplicate_union_variants");

        let typedefs = crate::optimizer::TypedefMap::new(types);

        for (_, type_) in types.simple_types_iter_mut() {
            if let SimpleTypeVariant::Union(x) = &mut type_.variant {
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

        Ok(())
    }
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
#[derive(Debug)]
pub struct RemoveEmptyUnions;

impl TypeTransformer for RemoveEmptyUnions {
    type Error = super::Error;

    fn transform(&self, types: &mut Types) -> Result<(), super::Error> {
        tracing::debug!("remove_empty_unions");

        for (_, type_) in types.simple_types_iter_mut() {
            if let SimpleTypeVariant::Union(x) = &type_.variant {
                if x.types.len() <= 1 {
                    let base = x.types.first().map(|x| &x.type_).or(x.base.as_ident());
                    if let Some(base) = base {
                        type_.variant =
                            SimpleTypeVariant::Reference(ReferenceInfo::new(base.clone()));
                    }
                }
            }
        }

        Ok(())
    }
}
