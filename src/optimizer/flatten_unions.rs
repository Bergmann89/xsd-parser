use crate::types::{Ident, SimpleTypeVariant, Types, UnionInfo, UnionTypeInfo};

use super::{Error, TypeTransformer};

struct FlattenUnionInfo {
    count: usize,
    info: UnionInfo,
}

/// This will flatten all union types.
///
/// For details see [`flatten_union`](Self::flatten_union).
#[derive(Debug)]
pub struct FlattenUnions;

impl TypeTransformer for FlattenUnions {
    type Error = super::Error;

    fn transform(&self, types: &mut Types) -> Result<(), Error> {
        tracing::debug!("flatten_unions");

        let idents = types
            .simple_types_iter()
            .filter_map(|(ident, variant)| {
                if matches!(&variant, SimpleTypeVariant::Union(_)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            self.flatten_union(types, ident).unwrap();
        }

        Ok(())
    }
}

impl FlattenUnions {
    /// This will flatten the union identified by `ident` to one single union.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed `ident` could not be found,
    /// or is not an union.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/union_flatten.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/flatten_unions.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/flatten_unions.rs")]
    /// ```
    pub fn flatten_union(&self, types: &mut Types, ident: Ident) -> Result<(), Error> {
        tracing::debug!("flatten_union(ident={ident:?})");

        let Some(ty) = types.get_simple_type(&ident) else {
            return Err(Error::UnknownType(ident));
        };

        let SimpleTypeVariant::Union(ui) = &ty.variant else {
            return Err(Error::ExpectedUnion(ident));
        };

        let mut info = FlattenUnionInfo {
            count: 0,
            info: UnionInfo::default(),
        };

        Self::flatten_union_impl(types, &ident, None, &mut info);

        if info.count > 1 {
            info.info.base = ui.base.clone();

            let ty = types.get_simple_type_mut(&ident).unwrap();
            ty.variant = SimpleTypeVariant::Union(info.info);
        }

        Ok(())
    }

    fn flatten_union_impl(
        types: &Types,
        ident: &Ident,
        display_name: Option<&str>,
        next: &mut FlattenUnionInfo,
    ) {
        let Some(type_) = types.get_simple_type(ident) else {
            return;
        };

        match &type_.variant {
            SimpleTypeVariant::Union(x) => {
                next.count += 1;
                for t in &*x.types {
                    Self::flatten_union_impl(types, &t.type_, t.display_name.as_deref(), next);
                }
            }
            SimpleTypeVariant::Reference(x) if x.is_single() => {
                Self::flatten_union_impl(types, &x.type_, display_name, next);
            }
            _ => {
                let mut ui = UnionTypeInfo::new(ident.clone());
                ui.display_name = display_name.map(ToOwned::to_owned);

                next.info.types.push(ui);
            }
        }
    }
}
