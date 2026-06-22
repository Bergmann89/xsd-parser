use std::borrow::Cow;

use crate::models::{data::CustomData, meta::CustomMeta};

use super::super::{Context, Error};

impl<'types> CustomData<'types> {
    pub(super) fn new(
        meta: &'types CustomMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        if let Some(generator) = &meta.generator {
            generator.exec(ctx, meta)?;
        }

        Ok(Self {
            meta: Cow::Borrowed(meta),
        })
    }
}
