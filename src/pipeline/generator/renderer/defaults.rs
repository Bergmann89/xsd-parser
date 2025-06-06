use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::super::data::{ComplexType, ComplexTypeAttribute, ComplexTypeStruct};
use super::{Context, Renderer, TypeData};

/// Implements a [`Renderer`] that renders associated methods that return the default
/// values of the different attributes and elements according to the schema.
#[derive(Debug)]
pub struct DefaultsRenderer;

impl Renderer for DefaultsRenderer {
    fn render_type(&mut self, ctx: &mut Context<'_, '_>, ty: &TypeData<'_>) {
        if let TypeData::Complex(x) = ty {
            x.render_defaults(ctx);
        }
    }
}

/* ComplexType */

impl ComplexType<'_> {
    pub(crate) fn render_defaults(&self, ctx: &mut Context<'_, '_>) {
        match self {
            ComplexType::Enum {
                type_: _,
                content_type,
            } => {
                if let Some(content_type) = content_type {
                    content_type.render_defaults(ctx);
                }
            }
            ComplexType::Struct {
                type_,
                content_type,
            } => {
                type_.render_defaults(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_defaults(ctx);
                }
            }
        }
    }
}

impl ComplexTypeStruct<'_> {
    pub(crate) fn render_defaults(&self, ctx: &mut Context<'_, '_>) {
        let type_ident = &self.type_ident;
        let mut has_attributes = false;
        let attribute_defaults = self
            .attributes
            .iter()
            .filter_map(|attrib| attrib.render_default_fn(ctx))
            .inspect(|_| has_attributes = true);

        let impl_ = quote! {
            impl #type_ident {
                #( #attribute_defaults )*
            }
        };

        if has_attributes {
            ctx.module().append(impl_);
        }
    }
}

impl ComplexTypeAttribute<'_> {
    fn render_default_fn(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let default = self.default_value.as_ref()?;
        let target_ident = ctx.resolve_type_for_module(&self.target_type);
        let default_fn_ident = format_ident!("default_{}", self.ident);

        Some(quote! {
            #[must_use]
            pub fn #default_fn_ident() -> #target_ident {
                #default
            }
        })
    }
}
