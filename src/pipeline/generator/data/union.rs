use proc_macro2::Literal;

use crate::models::{
    code::format_variant_ident,
    data::{UnionData, UnionTypeVariant},
    meta::{UnionMeta, UnionMetaType},
};

use super::super::{Context, Error};

impl<'types> UnionData<'types> {
    pub(super) fn new(
        meta: &'types UnionMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let type_ident = ctx.current_type_ref().type_ident.clone();
        let trait_impls = ctx.make_trait_impls()?;
        let variants = meta
            .types
            .iter()
            .map(|meta| meta.make_variant(ctx))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            meta,
            type_ident,
            variants,
            trait_impls,
        })
    }
}

impl UnionMetaType {
    fn make_variant<'types>(
        &'types self,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<UnionTypeVariant<'types>, Error> {
        let s_name = self.type_.name.to_string();
        let b_name = Literal::byte_string(s_name.as_bytes());
        let type_ref = ctx.get_or_create_type_ref(&self.type_)?;
        let target_type = type_ref.to_ident_path();
        let variant_ident = format_variant_ident(&self.type_.name, self.display_name.as_deref());

        Ok(UnionTypeVariant {
            meta: self,
            s_name,
            b_name,
            target_type,
            variant_ident,
        })
    }
}
