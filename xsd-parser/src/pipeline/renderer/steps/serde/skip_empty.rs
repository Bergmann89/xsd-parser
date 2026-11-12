use crate::{models::data::Occurs, pipeline::renderer::Context};

use super::misc::{FieldOverrides, SerdeField, SerdeOptions};

pub(super) struct SkipEmpty {
    pub(super) predicate: String,
    pub(super) needs_default: bool,
}

pub(super) fn arguments(
    ctx: &Context<'_, '_>,
    options: SerdeOptions,
    fields: &[SerdeField<'_>],
    overrides: &FieldOverrides,
    has_default: bool,
) -> Option<SkipEmpty> {
    if (!options.skip_none && !options.skip_empty_vecs) || !fields.first()?.named {
        return None;
    }
    // Omission must preserve explicit predicates, defaults and deserializers.
    if overrides.skip || overrides.has_custom_default || (overrides.deserialize && !has_default) {
        return None;
    }
    let occurs = fields
        .iter()
        .find(|field| field.occurs != Occurs::Single)?
        .occurs;
    let (predicate, needs_default) = match occurs {
        Occurs::Optional if options.skip_none => ("::core::option::Option::is_none".into(), false),
        Occurs::DynamicList if options.skip_empty_vecs => {
            (format!("::{}::vec::Vec::is_empty", ctx.alloc_crate), true)
        }
        _ => return None,
    };
    Some(SkipEmpty {
        predicate,
        needs_default,
    })
}
