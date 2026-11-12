use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::{models::data::Occurs, pipeline::renderer::Context};

use super::misc::{FieldOverrides, SerdeField, SerdeOptions};

pub(super) struct ArrayAdapter {
    pub(super) argument: TokenStream,
    pub(super) needs_default: bool,
}

struct Adapter {
    path: String,
    optional: bool,
}

pub(super) fn arguments(
    ctx: &Context<'_, '_>,
    options: SerdeOptions,
    fields: &[SerdeField<'_>],
    overrides: &FieldOverrides,
) -> Option<ArrayAdapter> {
    if !options.large_arrays || (overrides.serialize && overrides.deserialize) {
        return None;
    }
    let adapter = fields.iter().rev().fold(None, |inner, field| {
        container_adapter(&ctx.alloc_crate, field.occurs, field.boxed, inner)
    })?;
    let needs_default = adapter.optional && !overrides.deserialize && fields.first()?.named;
    let path = adapter.path;
    let argument = match (overrides.serialize, overrides.deserialize) {
        (true, false) => {
            let path = format!("::serde_with::As::<{path}>::deserialize");
            quote!(deserialize_with = #path)
        }
        (false, true) => {
            let path = format!("::serde_with::As::<{path}>::serialize");
            quote!(serialize_with = #path)
        }
        _ => {
            let path = format!("::serde_with::As::<{path}>");
            quote!(with = #path)
        }
    };
    Some(ArrayAdapter {
        argument,
        needs_default,
    })
}

fn container_adapter(
    alloc: &Ident,
    occurs: Occurs,
    boxed: bool,
    inner: Option<Adapter>,
) -> Option<Adapter> {
    if occurs == Occurs::None {
        return None;
    }
    let inner = match (occurs, inner) {
        (Occurs::StaticList(length), None) if length > 32 => Adapter {
            path: "::serde_with::Same".into(),
            optional: false,
        },
        (_, Some(inner)) => inner,
        _ => return None,
    };
    let optional = match occurs {
        Occurs::Single => inner.optional,
        Occurs::Optional => true,
        _ => false,
    };
    let inner = inner.path;
    let inner = if boxed && occurs != Occurs::DynamicList {
        format!("::{alloc}::boxed::Box<{inner}>")
    } else {
        inner
    };
    let path = match occurs {
        Occurs::Optional => format!("::core::option::Option<{inner}>"),
        Occurs::DynamicList => format!("::{alloc}::vec::Vec<{inner}>"),
        Occurs::StaticList(length) => format!("[{inner}; {length}]"),
        _ => inner,
    };
    Some(Adapter { path, optional })
}
