use crate::models::types::{ReferenceInfo, TypeVariant};

use super::{get_bases, Optimizer};

impl Optimizer {
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
    #[doc = include_str!("../../../tests/optimizer/complex_restricted.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected0/use_unrestricted_base_type.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected1/use_unrestricted_base_type.rs")]
    /// ```
    pub fn use_unrestricted_base_type(mut self) -> Self {
        tracing::debug!("use_unrestricted_base_type");

        let bases = get_bases!(self);

        for (ident, type_) in &mut *self.types {
            match &type_.variant {
                TypeVariant::ComplexType(_)
                | TypeVariant::Enumeration(_)
                | TypeVariant::Union(_) => {
                    let base = bases.get_unrestricted(ident).clone();
                    if *ident != base {
                        self.typedefs = None;
                        type_.variant = TypeVariant::Reference(ReferenceInfo::new(base));
                    }
                }
                _ => (),
            }
        }

        self
    }
}
