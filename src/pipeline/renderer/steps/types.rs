use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::config::{RendererFlags, TypedefMode};
use crate::models::data::{
    ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement, ComplexDataEnum,
    ComplexDataStruct, CustomData, DynamicData, EnumerationData, EnumerationTypeVariant, Occurs,
    ReferenceData, UnionData, UnionTypeVariant,
};

use super::super::{Context, DataTypeVariant, RenderStep};
use super::{format_traits, get_derive, get_dyn_type_traits, render_trait_impls};

/// Implements a [`RenderStep`] that renders the actual rust types of the types defined in the schema.
#[derive(Debug)]
pub struct TypesRenderStep;

impl RenderStep for TypesRenderStep {
    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) => (),
            DataTypeVariant::Custom(x) => x.render_types(ctx),
            DataTypeVariant::Union(x) => x.render_types(ctx),
            DataTypeVariant::Dynamic(x) => x.render_types(ctx),
            DataTypeVariant::Reference(x) => x.render_types(ctx),
            DataTypeVariant::Enumeration(x) => x.render_types(ctx),
            DataTypeVariant::Complex(x) => x.render_types(ctx),
        }
    }
}

/* CustomType */

impl CustomData<'_> {
    fn render_types(&self, ctx: &mut Context<'_, '_>) {
        let Some(include) = self.meta.include() else {
            return;
        };

        ctx.module().usings([include]);
    }
}

/* UnionType */

impl UnionData<'_> {
    fn render_types(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            trait_impls,
            variants,
            ..
        } = self;
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, Option::<String>::None);
        let trait_impls = render_trait_impls(type_ident, trait_impls);
        let variants = variants.iter().map(|x| x.render_variant(ctx));

        let code = quote! {
            #docs
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        };

        ctx.module().append(code);
    }
}

/* UnionTypeVariant */

impl UnionTypeVariant<'_> {
    fn render_variant(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let Self {
            target_type,
            variant_ident,
            ..
        } = self;

        let target_type = ctx.resolve_type_for_module(target_type);

        quote! {
            #variant_ident ( #target_type ),
        }
    }
}

/* DynamicType */

impl DynamicData<'_> {
    fn render_types(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            trait_ident,
            sub_traits,
            ..
        } = self;

        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, Option::<String>::None);
        let trait_impls = render_trait_impls(type_ident, &[]);
        let dyn_traits = sub_traits.as_ref().map_or_else(
            || get_dyn_type_traits(ctx, []),
            |traits| format_traits(traits.iter().map(|x| ctx.resolve_type_for_module(x))),
        );

        let code = quote! {
            #docs
            #derive
            pub struct #type_ident(pub Box<dyn #trait_ident>);

            pub trait #trait_ident: #dyn_traits { }

            #( #trait_impls )*
        };

        ctx.module().append(code);
    }
}

/* ReferenceType */

impl ReferenceData<'_> {
    fn render_types(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            mode,
            occurs,
            type_ident,
            target_type,
            trait_impls,
            ..
        } = self;

        let docs = ctx.render_type_docs();
        let target_type = ctx.resolve_type_for_module(target_type);

        let code = match mode {
            TypedefMode::Auto => crate::unreachable!(),
            TypedefMode::Typedef => {
                let target_type = occurs.make_type(&target_type, false);

                quote! {
                    #docs
                    pub type #type_ident = #target_type;
                }
            }
            TypedefMode::NewType => {
                let target_type = occurs.make_type(&target_type, false);
                let extra_derive =
                    matches!(occurs, Occurs::Optional | Occurs::DynamicList).then_some("Default");
                let derive = get_derive(ctx, extra_derive);
                let trait_impls = render_trait_impls(type_ident, trait_impls);

                quote! {
                    #docs
                    #derive
                    pub struct #type_ident(pub #target_type);

                    #( #trait_impls )*
                }
            }
        };

        ctx.module().append(code);
    }
}

/* EnumerationType */

impl EnumerationData<'_> {
    fn render_types(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            trait_impls,
            ..
        } = self;

        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, Option::<String>::None);
        let trait_impls = render_trait_impls(type_ident, trait_impls);

        let variants = variants
            .iter()
            .map(|d| d.render_variant(ctx))
            .collect::<Vec<_>>();

        let code = quote! {
            #docs
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        };

        ctx.module().append(code);
    }
}

impl EnumerationTypeVariant<'_> {
    fn render_variant(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let Self {
            meta,
            variant_ident,
            target_type,
            ..
        } = self;

        let docs = ctx.render_docs(RendererFlags::RENDER_VARIANT_DOCS, &meta.documentation[..]);

        let target_type = target_type.as_ref().map(|target_type| {
            let target_type = ctx.resolve_type_for_module(target_type);

            quote!((#target_type))
        });

        quote! {
            #docs
            #variant_ident #target_type,
        }
    }
}

/* ComplexType */

impl ComplexData<'_> {
    fn render_types(&self, ctx: &mut Context<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_type(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_types(ctx);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_type(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_types(ctx);
                }
            }
        }
    }
}

impl ComplexDataEnum<'_> {
    fn render_type(&self, ctx: &mut Context<'_, '_>) {
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, Option::<String>::None);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(type_ident, &self.trait_impls);

        let variants = self.elements.iter().map(|x| x.render_variant(ctx));

        let code = quote! {
            #docs
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        };

        ctx.module().append(code);
    }
}

impl ComplexDataStruct<'_> {
    fn render_type(&self, ctx: &mut Context<'_, '_>) {
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, Option::<String>::None);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(type_ident, &self.trait_impls);

        let attributes = self.attributes.iter().map(|x| x.render_field(ctx));
        let text_before = self
            .is_mixed
            .then(|| quote!(pub text_before: Option<String>,));
        let fields = self.elements().iter().map(|x| x.render_field(ctx));
        let content = self.content().as_ref().and_then(|x| x.render_field(ctx));

        let struct_data = if self.is_unit_struct() {
            quote!(;)
        } else {
            quote! {
                {
                    #( #attributes )*
                    #text_before
                    #( #fields )*
                    #content
                }
            }
        };

        let code = quote! {
            #docs
            #derive
            pub struct #type_ident
                #struct_data

            #( #trait_impls )*
        };

        ctx.module().append(code);
    }
}

impl ComplexDataContent {
    fn render_field(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self.occurs.make_type(&target_type, false)?;

        Some(quote! {
            pub content: #target_type,
        })
    }
}

impl ComplexDataAttribute<'_> {
    fn render_field(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let field_ident = &self.ident;

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = if self.is_option {
            quote!(Option<#target_type>)
        } else {
            target_type
        };

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ATTRIBUTE_DOCS,
            &self.meta.documentation[..],
        );

        if let Some(ty) = self
            .meta
            .is_any()
            .then_some(())
            .and(ctx.any_attribute_type.as_ref())
        {
            ctx.add_usings([ty.to_token_stream()]);
        }

        quote! {
            #docs
            pub #field_ident: #target_type,
        }
    }
}

impl ComplexDataElement<'_> {
    fn render_field(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let field_ident = &self.field_ident;

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self
            .occurs
            .make_type(&target_type, self.need_indirection)
            .unwrap();

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ELEMENT_DOCS,
            &self.meta.documentation[..],
        );

        if let Some(ty) = self.meta.is_any().then_some(()).and(ctx.any_type.as_ref()) {
            ctx.add_usings([ty.to_token_stream()]);
        }

        quote! {
            #docs
            pub #field_ident: #target_type,
        }
    }

    fn render_variant(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let variant_ident = &self.variant_ident;

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self.occurs.make_type(&target_type, self.need_indirection);

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ELEMENT_DOCS,
            &self.meta.documentation[..],
        );

        quote! {
            #docs
            #variant_ident(#target_type),
        }
    }
}
