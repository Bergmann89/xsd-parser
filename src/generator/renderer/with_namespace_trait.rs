use proc_macro2::{Ident as Ident2, TokenStream};
use quote::quote;

use crate::generator::data::{
    ComplexType, ComplexTypeEnum, ComplexTypeStruct, DynamicType, EnumerationType, ReferenceType,
    UnionType,
};

use super::{Context, Renderer, TypeData};

/// Implements a [`Renderer`] that renders the [`WithNamespace`](crate::WithNamespace)
/// trait for each type defined in the schema.
#[derive(Debug)]
pub struct WithNamespaceTraitRenderer;

impl Renderer for WithNamespaceTraitRenderer {
    fn render_type(&mut self, ctx: &mut Context<'_, '_>, ty: &TypeData<'_>) {
        match ty {
            TypeData::BuildIn(_) => (),
            TypeData::Union(x) => x.render_with_namespace_trait(ctx),
            TypeData::Dynamic(x) => x.render_with_namespace_trait(ctx),
            TypeData::Reference(x) => x.render_with_namespace_trait(ctx),
            TypeData::Enumeration(x) => x.render_with_namespace_trait(ctx),
            TypeData::Complex(x) => x.render_with_namespace_trait(ctx),
        };
    }
}

/* UnionType */

impl UnionType<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

/* DynamicType */

impl DynamicType<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

/* ReferenceType */

impl ReferenceType<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

/* EnumerationType */

impl EnumerationType<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

/* ComplexType */

impl ComplexType<'_> {
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
        };
    }
}

impl ComplexTypeEnum<'_> {
    fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

impl ComplexTypeStruct<'_> {
    fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.module().append(code);
        }
    }
}

fn render_trait_with_namespace(ctx: &Context<'_, '_>, type_ident: &Ident2) -> Option<TokenStream> {
    let ns = ctx.current_ns()?;
    let module = ctx.types.modules.get(&ns)?;
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
