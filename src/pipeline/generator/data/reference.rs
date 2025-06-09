use crate::config::TypedefMode;
use crate::models::{
    data::{Occurs, ReferenceData},
    meta::ReferenceMeta,
};

use super::super::{Context, Error};

impl<'types> ReferenceData<'types> {
    pub(super) fn new(
        meta: &'types ReferenceMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let occurs = Occurs::from_occurs(meta.min_occurs, meta.max_occurs);
        let type_ident = ctx.current_type_ref().type_ident.clone();
        let target_ref = ctx.get_or_create_type_ref(meta.type_.clone())?;
        let target_type = target_ref.to_ident_path();
        let trait_impls = ctx.make_trait_impls()?;

        let mode = match (ctx.typedef_mode, occurs) {
            (TypedefMode::Auto, Occurs::None | Occurs::Single) => TypedefMode::Typedef,
            (TypedefMode::Auto, _) => TypedefMode::NewType,
            (mode, _) => mode,
        };

        Ok(Self {
            meta,
            mode,
            occurs,
            type_ident,
            target_type,
            trait_impls,
        })
    }
}
