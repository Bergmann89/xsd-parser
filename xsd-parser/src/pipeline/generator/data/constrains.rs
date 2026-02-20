use std::ops::{Bound, Range};

use crate::models::{data::ConstrainsData, meta::Constrains, TypeIdent};
use crate::pipeline::generator::ValueGeneratorMode;

use super::super::{Context, Error};

impl<'types> ConstrainsData<'types> {
    pub(super) fn new(
        meta: &'types Constrains,
        base: Option<&TypeIdent>,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let range = if let Some(base) = base {
            let start = match &meta.range.start {
                Bound::Unbounded => Bound::Unbounded,
                Bound::Included(x) => {
                    Bound::Included(ctx.make_value_renderer(base, x, ValueGeneratorMode::Value)?)
                }
                Bound::Excluded(x) => {
                    Bound::Excluded(ctx.make_value_renderer(base, x, ValueGeneratorMode::Value)?)
                }
            };
            let end = match &meta.range.end {
                Bound::Unbounded => Bound::Unbounded,
                Bound::Included(x) => {
                    Bound::Included(ctx.make_value_renderer(base, x, ValueGeneratorMode::Value)?)
                }
                Bound::Excluded(x) => {
                    Bound::Excluded(ctx.make_value_renderer(base, x, ValueGeneratorMode::Value)?)
                }
            };

            Range { start, end }
        } else {
            Range {
                start: Bound::Unbounded,
                end: Bound::Unbounded,
            }
        };

        Ok(Self { meta, range })
    }
}
