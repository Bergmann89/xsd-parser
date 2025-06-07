use crate::models::{
    meta::{MetaTypeVariant, UnionMeta, UnionMetaType},
    Ident,
};

use super::{Error, Optimizer};

struct FlattenUnionInfo {
    count: usize,
    info: UnionMeta,
}

impl Optimizer {
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
    #[doc = include_str!("../../../tests/optimizer/union_flatten.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected0/flatten_unions.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected1/flatten_unions.rs")]
    /// ```
    pub fn flatten_union(mut self, ident: Ident) -> Result<Self, Error> {
        tracing::debug!("flatten_union(ident={ident:?})");

        let Some(ty) = self.types.items.get(&ident) else {
            return Err(Error::UnknownType(ident));
        };

        let MetaTypeVariant::Union(ui) = &ty.variant else {
            return Err(Error::ExpectedUnion(ident));
        };

        let mut info = FlattenUnionInfo {
            count: 0,
            info: UnionMeta::default(),
        };

        self.flatten_union_impl(&ident, None, &mut info);

        if info.count > 1 {
            info.info.base = ui.base.clone();

            let ty = self.types.items.get_mut(&ident).unwrap();
            ty.variant = MetaTypeVariant::Union(info.info);
        }

        Ok(self)
    }

    /// This will flatten all union types.
    ///
    /// For details see [`flatten_union`](Self::flatten_union).
    pub fn flatten_unions(mut self) -> Self {
        tracing::debug!("flatten_unions");

        let idents = self
            .types
            .items
            .iter()
            .filter_map(|(ident, type_)| {
                if matches!(&type_.variant, MetaTypeVariant::Union(_)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            self = self.flatten_union(ident).unwrap();
        }

        self
    }

    fn flatten_union_impl(
        &self,
        ident: &Ident,
        display_name: Option<&str>,
        next: &mut FlattenUnionInfo,
    ) {
        let Some(type_) = self.types.items.get(ident) else {
            return;
        };

        match &type_.variant {
            MetaTypeVariant::Union(x) => {
                next.count += 1;
                for t in &*x.types {
                    self.flatten_union_impl(&t.type_, t.display_name.as_deref(), next);
                }
            }
            MetaTypeVariant::Reference(x) if x.is_single() => {
                self.flatten_union_impl(&x.type_, display_name, next);
            }
            _ => {
                let mut ui = UnionMetaType::new(ident.clone());
                ui.display_name = display_name.map(ToOwned::to_owned);

                next.info.types.push(ui);
            }
        }
    }
}
