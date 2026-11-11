use std::borrow::Cow;
use std::str::FromStr;

use crate::config::{GeneratorFlags, TypedefMode};
use crate::models::{
    code::IdentPath,
    data::{Occurs, PathData, ReferenceData},
    meta::{CustomMeta, ReferenceMeta},
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
        let type_ident = ctx.current_type_ref().path.ident().clone();

        let target_ref = ctx.get_or_create_type_ref_for_value(&meta.type_, occurs.is_direct())?;
        let target_type = target_ref.path.clone();
        let target_type = ctx.path_data_nillable(nillable, target_type);

        let trait_impls = ctx.make_trait_impls()?;

        let meta = Cow::Borrowed(meta);
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

    pub(super) fn new_custom_alias(
        custom: &CustomMeta,
        ctx: &Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let include = custom
            .include()
            .ok_or_else(|| Error::MissingCustomAliasTarget(ctx.ident.clone()))?;
        let target_type = PathData::from_path(IdentPath::from_str(include)?);

        Ok(Self {
            // Native Rust targets have no separate schema identifier. This placeholder
            // points to the original CustomMeta in MetaTypes; target_type carries
            // the Rust alias target, not meta.type_.
            meta: Cow::Owned(ReferenceMeta::new(ctx.ident.clone())),
            mode: TypedefMode::Typedef,
            occurs: Occurs::Single,
            type_ident: ctx.current_type_ref().path.ident().clone(),
            target_type,
            trait_impls: Vec::new(),
        })
    }
}
