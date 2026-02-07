use proc_macro2::Literal;
use quote::format_ident;

use crate::models::{
    data::{DerivedType, DynamicData, PathData},
    meta::{DerivedTypeMeta, DynamicMeta, MetaTypeVariant},
};

use super::super::{Context, Error};

impl<'types> DynamicData<'types> {
    pub(super) fn new(
        meta: &'types DynamicMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let type_ident = ctx.current_type_ref().path.ident().clone();
        let trait_ident = format_ident!("{type_ident}Trait");
        let ident = ctx.ident.clone();
        let sub_traits = ctx
            .get_trait_infos()
            .get(&ident)
            .map(|info| info.traits_direct.clone())
            .map(|traits_direct| {
                traits_direct
                    .iter()
                    .map(|ident| {
                        ctx.get_or_create_type_ref(ident).map(|x| {
                            let ident = format_ident!("{}Trait", x.path.ident());

                            let target_type = (*x.path).clone().with_ident(ident);

                            PathData::from_path(target_type)
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let derived_types = meta
            .derived_types
            .iter()
            .map(|x| make_derived_type_data(ctx, x))
            .collect::<Result<Vec<_>, _>>()?;

        let deserializer_ident = format_ident!("{type_ident}Deserializer");

        Ok(Self {
            meta,
            type_ident,
            trait_ident,
            deserializer_ident,
            sub_traits,
            derived_types,
        })
    }
}

fn make_derived_type_data<'types>(
    ctx: &mut Context<'_, 'types>,
    meta: &'types DerivedTypeMeta,
) -> Result<DerivedType, Error> {
    let s_name = meta.type_.name.to_string();
    let b_name = Literal::byte_string(s_name.as_bytes());

    let ty = ctx
        .types
        .items
        .get(&meta.type_)
        .ok_or_else(|| Error::UnknownType(meta.type_.clone()))?;

    let base_ident = if let MetaTypeVariant::Dynamic(di) = &ty.variant {
        di.type_.clone()
    } else {
        None
    };

    let ident = base_ident.unwrap_or(meta.type_.clone());
    let target_ref = ctx.get_or_create_type_ref(&ident)?;
    let target_type = target_ref.path.clone();
    let variant_ident = ctx
        .types
        .naming
        .format_variant_ident(&ident.name, meta.display_name.as_deref());

    Ok(DerivedType {
        ident,
        b_name,
        target_type,
        variant_ident,
    })
}
