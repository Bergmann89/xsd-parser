use std::ops::{Bound, Range};

use crate::models::{data::SimpleData, meta::SimpleMeta};

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
        let current_ns = ctx.current_module();
        let type_ident = ctx.current_type_ref().path.ident().clone();

        let start = match &meta.range.start {
            Bound::Unbounded => Bound::Unbounded,
            Bound::Included(x) => Bound::Included(ctx.render_literal(current_ns, x, base)?),
            Bound::Excluded(x) => Bound::Excluded(ctx.render_literal(current_ns, x, base)?),
        };
        let end = match &meta.range.end {
            Bound::Unbounded => Bound::Unbounded,
            Bound::Included(x) => Bound::Included(ctx.render_literal(current_ns, x, base)?),
            Bound::Excluded(x) => Bound::Excluded(ctx.render_literal(current_ns, x, base)?),
        };
        let range = Range { start, end };

        let target_ref = ctx.get_or_create_type_ref(&meta.base)?;
        let target_type = target_ref.path.clone();

        let trait_impls = ctx.make_trait_impls()?;

        Ok(Self {
            meta,
            range,
            type_ident,
            target_type,
            trait_impls,
        })
    }
}
