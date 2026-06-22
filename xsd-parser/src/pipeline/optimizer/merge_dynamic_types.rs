use indexmap::IndexMap;

use crate::models::{
    meta::{DerivedTypeMeta, MetaTypeVariant},
    TypeIdent,
};

use super::{Error, Optimizer};

impl Optimizer {
    /// This will merge the derived types of a dynamic type into the base type if
    /// the base type is also a dynamic type representing the same element. This
    /// is useful if the schema uses dynamic types in combination with substitution
    /// groups. Both of them cover more or less the same dynamic types, but used
    /// different `Box`es to store them. After you've applied this optimization
    /// the instances of the substitution group and the dynamic type will be stored
    /// in the same `Box`.
    ///
    /// This optimization need to be used with care, because it can lead to a loss
    /// of information. If you serialize a XML document that was deserialized with
    /// this optimization before, the serializer can not determine if an element
    /// needs to be emitted as a substitution group or as a dynamic type anymore.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed `ident` could not be found or the referenced
    /// type is not dynamic type.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../../tests/optimizer/merge_dynamic_types.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected0/merge_dynamic_types.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected1/merge_dynamic_types.rs")]
    /// ```
    pub fn merge_dynamic_type(mut self, ident: TypeIdent) -> Result<Self, Error> {
        tracing::debug!("merge_dynamic_types(ident={ident:?})");

        self.merge_dynamic_types_impl(ident)?;

        Ok(self)
    }

    /// This will flatten all complex types.
    ///
    /// For details see [`merge_dynamic_type`](Self::merge_dynamic_type).
    pub fn merge_dynamic_types(mut self) -> Self {
        tracing::debug!("merge_dynamic_types");

        let idents = self
            .types
            .items
            .iter()
            .filter_map(|(ident, type_)| {
                if matches!(&type_.variant, MetaTypeVariant::Dynamic(_)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            let _ = self.merge_dynamic_types_impl(ident);
        }

        self
    }

    fn merge_dynamic_types_impl(&mut self, ident: TypeIdent) -> Result<(), Error> {
        tracing::debug!("merge_dynamic_types_impl(ident={ident:?})");

        let Some(ty) = self.types.items.get(&ident) else {
            return Err(Error::UnknownType(ident));
        };

        let MetaTypeVariant::Dynamic(dm) = &ty.variant else {
            return Err(Error::ExpectedDynamicType(ident));
        };

        let Some(base_ident) = dm
            .type_
            .as_ref()
            .and_then(|ident| self.types.get_resolved_ident(ident))
        else {
            return Ok(());
        };

        let mut derived_types = IndexMap::new();
        self.add_derived_types(&mut derived_types, base_ident, &dm.derived_types);

        if let Some(MetaTypeVariant::Dynamic(dm)) = self.types.get_variant_mut(&ident) {
            dm.derived_types = derived_types;
        }

        Ok(())
    }

    fn add_derived_types(
        &self,
        new: &mut IndexMap<TypeIdent, DerivedTypeMeta>,
        base_ident: &TypeIdent,
        old: &IndexMap<TypeIdent, DerivedTypeMeta>,
    ) {
        for (key, old) in old {
            if let Some((ident, ty)) = self.types.get_resolved(&old.type_) {
                if base_ident == ident {
                    if let MetaTypeVariant::Dynamic(dm) = &ty.variant {
                        self.add_derived_types(new, base_ident, &dm.derived_types);
                        continue;
                    }
                }
            }

            new.entry(key.clone()).or_insert_with(|| old.clone());
        }
    }
}
