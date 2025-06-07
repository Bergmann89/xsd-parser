use proc_macro2::Literal;
use quote::format_ident;

use crate::models::{
    code::format_variant_ident,
    data::{DerivedType, DynamicData},
    meta::{DynamicMeta, MetaTypeVariant},
    Ident,
};

use super::super::{Context, Error};

impl<'types> DynamicData<'types> {
    pub(super) fn new(
        info: &'types DynamicMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let type_ident = ctx.current_type_ref().type_ident.clone();
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
                        ctx.get_or_create_type_ref(ident.clone()).map(|x| {
                            let ident = format_ident!("{}Trait", x.type_ident);

                            x.to_ident_path().with_ident(ident)
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let derived_types = info
            .derived_types
            .iter()
            .map(|ident| make_derived_type_data(ctx, ident))
            .collect::<Result<Vec<_>, _>>()?;

        let deserializer_ident = format_ident!("{type_ident}Deserializer");

        Ok(Self {
            meta: info,
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
    ident: &'types Ident,
) -> Result<DerivedType, Error> {
    let s_name = ident.name.to_string();
    let b_name = Literal::byte_string(s_name.as_bytes());

    let ty = ctx
        .types
        .items
        .get(ident)
        .ok_or_else(|| Error::UnknownType(ident.clone()))?;
    let base_ident = if let MetaTypeVariant::Dynamic(di) = &ty.variant {
        di.type_.clone()
    } else {
        None
    };
    let ident = base_ident.unwrap_or(ident.clone());

    let target_ref = ctx.get_or_create_type_ref(ident.clone())?;
    let target_type = target_ref.to_ident_path();
    let variant_ident = format_variant_ident(&ident.name, None);

    Ok(DerivedType {
        ident,
        b_name,
        target_type,
        variant_ident,
    })
}
