use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::generator::{
    data::{
        ComplexType, ComplexTypeAttribute, ComplexTypeStruct, DynamicType, EnumerationType,
        ReferenceType, UnionType,
    },
    Context,
};

/* UnionType */

impl UnionType<'_> {
    pub(crate) fn render_impl(&self, ctx: &Context<'_, '_>) {
        let _self = self;
        let _ctx = ctx;
    }
}

/* DynamicType */

impl DynamicType<'_> {
    pub(crate) fn render_impl(&self, ctx: &Context<'_, '_>) {
        let _self = self;
        let _ctx = ctx;
    }
}

/* ReferenceType */

impl ReferenceType<'_> {
    pub(crate) fn render_impl(&self, ctx: &Context<'_, '_>) {
        let _self = self;
        let _ctx = ctx;
    }
}

/* EnumerationType */

impl EnumerationType<'_> {
    pub(crate) fn render_impl(&self, ctx: &Context<'_, '_>) {
        let _self = self;
        let _ctx = ctx;
    }
}

/* ComplexType */

impl ComplexType<'_> {
    pub(crate) fn render_impl(&self, ctx: &mut Context<'_, '_>) {
        match self {
            ComplexType::Enum {
                type_: _,
                content_type,
            } => {
                if let Some(content_type) = content_type {
                    content_type.render_impl(ctx);
                }
            }
            ComplexType::Struct {
                type_,
                content_type,
            } => {
                type_.render_impl(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_impl(ctx);
                }
            }
        }
    }
}

impl ComplexTypeStruct<'_> {
    pub(crate) fn render_impl(&self, ctx: &mut Context<'_, '_>) {
        let type_ident = &self.type_ident;
        let mut has_attributes = false;
        let attribute_defaults = self
            .attributes
            .iter()
            .filter_map(ComplexTypeAttribute::render_default_fn)
            .inspect(|_| has_attributes = true);

        let impl_ = quote! {
            impl #type_ident {
                #( #attribute_defaults )*
            }
        };

        if has_attributes {
            ctx.main().code(impl_);
        }
    }
}

impl ComplexTypeAttribute<'_> {
    fn render_default_fn(&self) -> Option<TokenStream> {
        let default = self.default_value.as_ref()?;
        let target_type = self.target_type.ident();
        let default_fn_ident = format_ident!("default_{}", self.ident);

        Some(quote! {
            #[must_use]
            pub fn #default_fn_ident() -> #target_type {
                #default
            }
        })
    }
}
