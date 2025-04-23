use std::ops::DerefMut;

use crate::types::{ReferenceInfo, TypeVariant, Types};

use super::{Error, TypeTransformer};

/// This will use the unrestricted base type instead the restricted version
/// when ever possible.
///
/// This is useful if you want to reduce the amount of different types,
/// because the base type can store the same data than the restricted one.
/// However this is only useful if you want to deserialize the type only.
/// Using this feature for serializing types will cause problems because the
/// type information is dropped during deserialization.
///
/// # Examples
///
/// Consider the following XML schema.
/// ```xml
#[doc = include_str!("../../tests/optimizer/complex_restricted.xsd")]
/// ```
///
/// Without this optimization this will result in the following code:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected0/use_unrestricted_base_type.rs")]
/// ```
///
/// With this optimization the following code is generated:
/// ```rust
#[doc = include_str!("../../tests/optimizer/expected1/use_unrestricted_base_type.rs")]
/// ```
#[derive(Debug)]
pub struct UseUnrestrictedBaseType;

impl TypeTransformer for UseUnrestrictedBaseType {
    fn transform(&self, types: &mut Types) -> Result<(), Error> {
        tracing::debug!("use_unrestricted_base_type");

        let bases = crate::optimizer::BaseMap::new(types);

        for (ident, type_) in types.deref_mut() {
            match &type_.variant {
                TypeVariant::ComplexType(_)
                | TypeVariant::Enumeration(_)
                | TypeVariant::Union(_) => {
                    let base = bases.get_unrestricted(ident).clone();
                    if *ident != base {
                        type_.variant = TypeVariant::Reference(ReferenceInfo::new(base));
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}
