use std::collections::HashSet;

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::{
    config::TypedefMode,
    models::{
        data::{
            ComplexDataAttribute, ComplexDataContent, ComplexDataElement, DataTypeVariant,
            EnumerationDataVariant, Occurs, ReferenceData, SimpleData, UnionTypeVariant,
        },
        meta::AttributeMetaVariant,
        TypeIdent,
    },
    pipeline::renderer::Context,
};

use super::{field_overrides, FieldOverrides, SerdeOptions};

struct Adapter {
    path: String,
    optional: bool,
}

fn attribute(
    ctx: &Context<'_, '_>,
    occurs: Occurs,
    boxed: bool,
    target: Option<&TypeIdent>,
    options: SerdeOptions,
    overrides: &FieldOverrides,
    named_field: bool,
) -> Option<TokenStream> {
    if !options.large_arrays {
        return None;
    }
    if overrides.serialize && overrides.deserialize {
        return None;
    }
    let inner = target
        .and_then(|ident| alias_adapter(ctx, ident, options.arrays_as_vec, &mut HashSet::new()));
    let adapter = container_adapter(&ctx.alloc_crate, occurs, boxed, inner)?;
    let needs_default =
        adapter.optional && !overrides.deserialize && !overrides.has_default && named_field;
    let default = needs_default.then(|| quote!(#[serde(default)]));
    let adapter = adapter.path;
    let attribute = match (overrides.serialize, overrides.deserialize) {
        (true, false) => {
            let path = format!("::serde_with::As::<{adapter}>::deserialize");
            quote!(#[serde(deserialize_with = #path)])
        }
        (false, true) => {
            let path = format!("::serde_with::As::<{adapter}>::serialize");
            quote!(#[serde(serialize_with = #path)])
        }
        _ => {
            let path = format!("::serde_with::As::<{adapter}>");
            quote!(#[serde(with = #path)])
        }
    };
    Some(quote!(#default #attribute))
}

fn alias_adapter<'a>(
    ctx: &'a Context<'_, '_>,
    ident: &'a TypeIdent,
    arrays_as_vec: bool,
    visited: &mut HashSet<&'a TypeIdent>,
) -> Option<Adapter> {
    let item = ctx.meta.types.items.get(ident)?;
    let DataTypeVariant::Reference(reference) = &item.variant else {
        return None;
    };
    if reference.mode != TypedefMode::Typedef || !visited.insert(ident) {
        return None;
    }
    let inner = reference
        .target_type
        .serde_target_ident(Some(&reference.meta.type_))
        .and_then(|ident| alias_adapter(ctx, ident, arrays_as_vec, visited));
    container_adapter(
        &ctx.alloc_crate,
        effective_occurs(reference.occurs, arrays_as_vec),
        false,
        inner,
    )
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

fn effective_occurs(occurs: Occurs, arrays_as_vec: bool) -> Occurs {
    if arrays_as_vec {
        occurs.array_to_vec()
    } else {
        occurs
    }
}

impl SimpleData<'_> {
    pub(crate) fn serde_array_attribute(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> Option<TokenStream> {
        attribute(
            ctx,
            self.occurs,
            false,
            self.target_type.serde_target_ident(Some(&self.meta.base)),
            options,
            &FieldOverrides::default(),
            false,
        )
    }
}

impl ReferenceData<'_> {
    pub(crate) fn serde_array_attribute(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> Option<TokenStream> {
        attribute(
            ctx,
            effective_occurs(self.occurs, options.arrays_as_vec),
            false,
            self.target_type.serde_target_ident(Some(&self.meta.type_)),
            options,
            &FieldOverrides::default(),
            false,
        )
    }
}

impl UnionTypeVariant<'_> {
    pub(crate) fn serde_array_attribute(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> Option<TokenStream> {
        attribute(
            ctx,
            Occurs::Single,
            false,
            self.target_type.serde_target_ident(Some(&self.meta.type_)),
            options,
            &field_overrides(&self.extra_attributes),
            false,
        )
    }
}

impl EnumerationDataVariant<'_> {
    pub(crate) fn serde_array_attribute(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> Option<TokenStream> {
        let target_type = self.target_type.as_ref()?;
        attribute(
            ctx,
            Occurs::Single,
            false,
            target_type.serde_target_ident(self.meta.type_.as_ref()),
            options,
            &field_overrides(&self.extra_attributes),
            false,
        )
    }
}

impl ComplexDataElement<'_> {
    pub(crate) fn serde_array_field_attribute(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
        overrides: &FieldOverrides,
    ) -> Option<TokenStream> {
        self.array_attribute(ctx, options, overrides, true)
    }

    pub(crate) fn serde_array_attribute(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> Option<TokenStream> {
        self.array_attribute(
            ctx,
            options,
            &field_overrides(&self.extra_attributes),
            false,
        )
    }

    fn array_attribute(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
        overrides: &FieldOverrides,
        named_field: bool,
    ) -> Option<TokenStream> {
        attribute(
            ctx,
            effective_occurs(self.occurs, options.arrays_as_vec),
            self.need_indirection,
            self.serde_target_ident(),
            options,
            overrides,
            named_field,
        )
    }
}

impl ComplexDataContent<'_> {
    pub(crate) fn serde_array_attribute(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
        overrides: &FieldOverrides,
    ) -> Option<TokenStream> {
        attribute(
            ctx,
            effective_occurs(self.occurs, options.arrays_as_vec),
            false,
            self.target_type.serde_target_ident(self.simple_type),
            options,
            overrides,
            true,
        )
    }
}

impl ComplexDataAttribute<'_> {
    pub(crate) fn serde_array_attribute(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
        overrides: &FieldOverrides,
    ) -> Option<TokenStream> {
        let target = match &self.meta.variant {
            AttributeMetaVariant::Type(type_) => Some(type_),
            _ => None,
        };
        attribute(
            ctx,
            if self.is_option {
                Occurs::Optional
            } else {
                Occurs::Single
            },
            false,
            self.target_type.serde_target_ident(target),
            options,
            overrides,
            true,
        )
    }
}
