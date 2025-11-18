use crate::models::{
    data::{ConstrainsData, Occurs, SimpleData},
    meta::SimpleMeta,
};

use super::super::{Context, Error};

impl<'types> SimpleData<'types> {
    pub(super) fn new(
        meta: &'types SimpleMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let base = ctx
            .types
            .get_resolved_ident(&meta.base)
            .unwrap_or(&meta.base);
        let type_ident = ctx.current_type_ref().path.ident().clone();

        let occurs = if meta.is_list {
            Occurs::DynamicList
        } else {
            Occurs::Single
        };

        let constrains = ConstrainsData::new(&meta.constrains, Some(base), ctx)?;

        let target_ref = ctx.get_or_create_type_ref(&meta.base)?;
        let target_type = target_ref.path.clone();

        let trait_impls = ctx.make_trait_impls()?;

        Ok(Self {
            meta,
            occurs,
            constrains,
            type_ident,
            target_type,
            trait_impls,
        })
    }
}
