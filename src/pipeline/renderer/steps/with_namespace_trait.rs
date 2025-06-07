use proc_macro2::{Ident as Ident2, TokenStream};
use quote::quote;

use crate::models::data::{
    ComplexData, ComplexDataEnum, ComplexDataStruct, DynamicData, EnumerationData, ReferenceData,
    UnionData,
};

use super::super::{Context, DataTypeVariant, RenderStep};

/// Implements a [`RenderStep`] that renders the [`WithNamespace`](crate::WithNamespace)
/// trait for each type defined in the schema.
#[derive(Debug)]
pub struct WithNamespaceTraitRenderStep;

impl RenderStep for WithNamespaceTraitRenderStep {
    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) | DataTypeVariant::Custom(_) => (),
            DataTypeVariant::Union(x) => x.render_with_namespace_trait(ctx),
            DataTypeVariant::Dynamic(x) => x.render_with_namespace_trait(ctx),
            DataTypeVariant::Reference(x) => x.render_with_namespace_trait(ctx),
            DataTypeVariant::Enumeration(x) => x.render_with_namespace_trait(ctx),
            DataTypeVariant::Complex(x) => x.render_with_namespace_trait(ctx),
        }
    }
}

/* UnionType */

impl UnionData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

/* DynamicType */

impl DynamicData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

/* ReferenceType */

impl ReferenceData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

/* EnumerationType */

impl EnumerationData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

/* ComplexType */

impl ComplexData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_with_namespace_trait(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_with_namespace_trait(ctx);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_with_namespace_trait(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_with_namespace_trait(ctx);
                }
            }
        }
    }
}

impl ComplexDataEnum<'_> {
    fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

impl ComplexDataStruct<'_> {
    fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

fn render_trait_with_namespace(ctx: &Context<'_, '_>, type_ident: &Ident2) -> Option<TokenStream> {
    let ns = ctx.ident.ns.as_ref()?;
    let module = ctx.types.meta.types.modules.get(ns)?;
    let xsd_parser = &ctx.xsd_parser_crate;

    let (prefix, namespace) = match (&module.name, &module.namespace) {
        (Some(prefix), Some(namespace)) => {
            let prefix = prefix.to_string();
            let namespace = namespace.to_string();

            (quote!(Some(#prefix)), quote!(Some(#namespace)))
        }
        (None, Some(namespace)) => {
            let namespace = namespace.to_string();

            (quote!(None), quote!(Some(#namespace)))
        }
        (_, None) => (quote!(None), quote!(None)),
    };

    Some(quote! {
        impl #xsd_parser::WithNamespace for #type_ident {
            fn prefix() -> Option<&'static str> {
                #prefix
            }

            fn namespace() -> Option<&'static str> {
                #namespace
            }
        }
    })
}
