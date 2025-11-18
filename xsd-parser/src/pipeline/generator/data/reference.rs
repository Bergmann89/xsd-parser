use crate::config::{GeneratorFlags, TypedefMode};
use crate::models::data::PathData;
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
        let nillable =
            meta.nillable && ctx.check_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT);
        let absolute = ctx.check_generator_flags(GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS);
        let type_ident = ctx.current_type_ref().path.ident().clone();

        let target_ref = ctx.get_or_create_type_ref_for_value(&meta.type_, occurs.is_direct())?;
        let target_type = target_ref.path.clone();
        let target_type = PathData::from_path_data_nillable(nillable, absolute, target_type);

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
