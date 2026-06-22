use std::borrow::Cow;

use proc_macro2::Literal;
use quote::format_ident;

use crate::models::{
    data::{DerivedType, DynamicData, TagName},
    meta::{DerivedTypeMeta, DynamicMeta},
    TypeIdent,
};

use super::super::{Context, Error};

impl<'types> DynamicData<'types> {
    pub(super) fn new(
        meta: &'types DynamicMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let type_ident = ctx.current_type_ref().path.ident().clone();
        let trait_ident = format_ident!("{type_ident}Trait");
        let trait_impls = ctx.make_trait_impls()?;
        let sub_traits = ctx.make_traits_derive()?;
        let form = ctx.types.get_form(ctx.ident);
        let tag_name = TagName::new(ctx.types, ctx.ident.ns, &ctx.ident.name, form);

        let derived_types = meta
            .derived_types
            .iter()
            .map(|(type_, meta)| make_derived_type_data(ctx, type_, meta))
            .collect::<Result<Vec<_>, _>>()?;

        let meta = Cow::Borrowed(meta);
        let deserializer_ident = format_ident!("{type_ident}Deserializer");

        Ok(Self {
            meta,
            type_ident,
            trait_ident,
            deserializer_ident,
            trait_impls,
            tag_name,
            sub_traits,
            derived_types,
        })
    }
}

fn make_derived_type_data<'types>(
    ctx: &mut Context<'_, 'types>,
    key: &'types TypeIdent,
    meta: &'types DerivedTypeMeta,
) -> Result<DerivedType<'types>, Error> {
    let s_name = key.name.to_string();
    let b_name = Literal::byte_string(s_name.as_bytes());
    let form = ctx.types.get_form(key);
    let tag_name = TagName::new(ctx.types, key.ns, &key.name, form);

    let key = key.clone();
    let target_ref = ctx.get_or_create_type_ref(&meta.type_)?;
    let target_type = target_ref.path.clone();
    let variant_ident = ctx
        .types
        .naming
        .format_variant_ident(&key.name, meta.display_name.as_deref());

    let meta = Cow::Borrowed(meta);

    Ok(DerivedType {
        key,
        meta,
        b_name,
        tag_name,
        target_type,
        variant_ident,
    })
}
