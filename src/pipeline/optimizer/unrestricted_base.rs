use bitflags::bitflags;

use crate::models::meta::{MetaTypeVariant, ReferenceMeta};

use super::{get_bases, Optimizer};

bitflags! {
    /// Flags to control the [`Optimizer::use_unrestricted_base_type`].
    #[derive(Debug, Clone, Copy)]
    pub struct UnrestrictedBaseFlags: u32 {
        /// Use the unrestricted base type for complex types.
        const COMPLEX = 1 << 0;

        /// Use the unrestricted base type for simple types.
        const SIMPLE = 1 << 1;

        /// Use the unrestricted base type for enum types.
        const ENUM = 1 << 2;

        /// Use the unrestricted base type for union types.
        const UNION = 1 << 3;
    }
}

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
    pub fn use_unrestricted_base_type(mut self, flags: UnrestrictedBaseFlags) -> Self {
        tracing::debug!("use_unrestricted_base_type");

        let bases = get_bases!(self);

        for (ident, type_) in &mut self.types.items {
            let replace = match &type_.variant {
                MetaTypeVariant::ComplexType(_) => flags.intersects(UnrestrictedBaseFlags::COMPLEX),
                MetaTypeVariant::SimpleType(_) => flags.intersects(UnrestrictedBaseFlags::SIMPLE),
                MetaTypeVariant::Enumeration(_) => flags.intersects(UnrestrictedBaseFlags::ENUM),
                MetaTypeVariant::Union(_) => flags.intersects(UnrestrictedBaseFlags::UNION),
                _ => false,
            };

            if replace {
                let base = bases.get_unrestricted(ident).clone();
                if *ident != base {
                    self.typedefs = None;
                    type_.variant = MetaTypeVariant::Reference(ReferenceMeta::new(base));
                }
            }
        }

        self
    }
}
