use proc_macro2::Literal;
use quote::format_ident;

use crate::models::{
    code::format_variant_ident,
    data::{EnumerationData, EnumerationTypeVariant},
    meta::{EnumerationMeta, EnumerationMetaVariant},
    schema::xs::Use,
};

use super::super::{Context, Error};

impl<'types> EnumerationData<'types> {
    pub(super) fn new(
        meta: &'types EnumerationMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let mut unknown = 0usize;
        let type_ident = ctx.current_type_ref().path.ident().clone();
        let trait_impls = ctx.make_trait_impls()?;

        let variants = meta
            .variants
            .iter()
            .filter_map(|var| var.make_variant(&mut unknown, ctx))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(EnumerationData {
            meta,
            type_ident,
            variants,
            trait_impls,
        })
    }
}

impl EnumerationMetaVariant {
    fn make_variant<'types>(
        &'types self,
        unknown: &mut usize,
        ctx: &mut Context<'_, 'types>,
    ) -> Option<Result<EnumerationTypeVariant<'types>, Error>> {
        match self.use_ {
            Use::Prohibited => None,
            Use::Required | Use::Optional => {
                let type_ref = if let Some(t) = &self.type_ {
                    match ctx.get_or_create_type_ref(t) {
                        Ok(target_ref) => Some(target_ref),
                        Err(error) => return Some(Err(error)),
                    }
                } else {
                    None
                };

                let variant_ident = if let Some(display_name) = self.display_name.as_deref() {
                    format_ident!("{display_name}")
                } else if let (Some(type_ref), true) = (type_ref, self.ident.name.is_generated()) {
                    type_ref.path.ident().clone()
                } else if self.ident.name.as_str().is_empty() {
                    *unknown += 1;

                    format_ident!("Unknown{unknown}")
                } else {
                    format_variant_ident(&self.ident.name, self.display_name.as_deref())
                };

                let s_name = self.ident.name.to_string();
                let b_name = Literal::byte_string(s_name.as_bytes());
                let target_type = type_ref.map(|x| x.path.clone());

                Some(Ok(EnumerationTypeVariant {
                    meta: self,
                    s_name,
                    b_name,
                    variant_ident,
                    target_type,
                }))
            }
        }
    }
}
