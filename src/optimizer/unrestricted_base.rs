use crate::types::{ComplexTypeVariant, ReferenceInfo, SimpleTypeVariant, Types};

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
    type Error = super::Error;

    fn transform(&self, types: &mut Types) -> Result<(), Error> {
        tracing::debug!("use_unrestricted_base_type");

        let bases = crate::optimizer::BaseMap::new(types);

        for (ident, variant) in types.complex_types_iter_mut() {
            if let ComplexTypeVariant::ComplexType(_) = variant {
                let base = bases.get_unrestricted(ident).clone();
                if *ident != base {
                    *variant = ComplexTypeVariant::Reference(ReferenceInfo::new(base));
                }
            }
        }

        for (ident, variant) in types.simple_types_iter_mut() {
            if let SimpleTypeVariant::Enumeration(_) | SimpleTypeVariant::Union(_) = variant {
                let base = bases.get_unrestricted(ident).clone();
                if *ident != base {
                    *variant = SimpleTypeVariant::Reference(ReferenceInfo::new(base));
                }
            }
        }

        Ok(())
    }
}
