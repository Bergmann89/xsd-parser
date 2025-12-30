use proc_macro2::{Ident as Ident2, TokenStream};
use quote::quote;

use crate::models::data::{
    ComplexData, ComplexDataEnum, ComplexDataStruct, DynamicData, EnumerationData, ReferenceData,
    SimpleData, UnionData,
};

use super::super::{Context, DataTypeVariant, RenderStep, RenderStepType};

/// Implements a [`RenderStep`] that renders the [`WithNamespace`](xsd_parser_types::WithNamespace)
/// trait for each type defined in the schema.
#[derive(Debug, Clone, Copy)]
pub struct WithNamespaceTraitRenderStep;

impl RenderStep for WithNamespaceTraitRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::ExtraImpls
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) | DataTypeVariant::Custom(_) => (),
            DataTypeVariant::Union(x) => x.render_with_namespace_trait(ctx),
            DataTypeVariant::Dynamic(x) => x.render_with_namespace_trait(ctx),
            DataTypeVariant::Reference(x) => x.render_with_namespace_trait(ctx),
            DataTypeVariant::Enumeration(x) => x.render_with_namespace_trait(ctx),
            DataTypeVariant::Simple(x) => x.render_with_namespace_trait(ctx),
            DataTypeVariant::Complex(x) => x.render_with_namespace_trait(ctx),
        }
    }
}

/* UnionData */

impl UnionData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.current_module().append(code);
        }
    }
}

/* DynamicData */

impl DynamicData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.current_module().append(code);
        }
    }
}

/* ReferenceData */

impl ReferenceData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.current_module().append(code);
        }
    }
}

/* EnumerationData */

impl EnumerationData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.current_module().append(code);
        }
    }
}

/* SimpleData */

impl SimpleData<'_> {
    pub(crate) fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.current_module().append(code);
        }
    }
}

/* ComplexData */

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
            ctx.current_module().append(code);
        }
    }
}

impl ComplexDataStruct<'_> {
    fn render_with_namespace_trait(&self, ctx: &mut Context<'_, '_>) {
        if let Some(code) = render_trait_with_namespace(ctx, &self.type_ident) {
            ctx.current_module().append(code);
        }
    }
}

fn render_trait_with_namespace(ctx: &Context<'_, '_>, type_ident: &Ident2) -> Option<TokenStream> {
    let ns = ctx.ident.ns;
    let module = ctx.types.meta.types.modules.get(&ns)?;
    let xsd_parser_types = &ctx.xsd_parser_types;

    let (prefix, namespace) = match (&module.prefix, &module.namespace) {
        (Some(prefix), Some(namespace)) => {
            let prefix = prefix.as_str();
            let namespace = namespace.to_string();

            (quote!(Some(#prefix)), quote!(Some(#namespace)))
        }
        (None, Some(namespace)) => {
            let namespace = namespace.to_string();

            (quote!(None), quote!(Some(#namespace)))
        }
        (_, None) => (quote!(None), quote!(None)),
    };

    let str_ = ctx.resolve_build_in("::core::primitive::str");
    let option = ctx.resolve_build_in("::core::option::Option");

    Some(quote! {
        impl #xsd_parser_types::WithNamespace for #type_ident {
            fn prefix() -> #option<&'static #str_> {
                #prefix
            }

            fn namespace() -> #option<&'static #str_> {
                #namespace
            }
        }
    })
}
