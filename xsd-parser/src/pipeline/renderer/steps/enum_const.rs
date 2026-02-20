use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;

use crate::models::data::{EnumerationData, EnumerationDataVariant, EnumerationVariantValue};

use super::super::{Context, DataTypeVariant, RenderStep, RenderStepType};

/// Implements a [`RenderStep`] that renders the actual rust types of the types defined in the schema.
#[derive(Debug, Clone, Copy)]
pub struct EnumConstantsRenderStep;

impl RenderStep for EnumConstantsRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::Types
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        if let DataTypeVariant::Enumeration(x) = &ctx.data.variant {
            x.render_enum_constants(ctx);
        }
    }
}

/* EnumerationData */

impl EnumerationData<'_> {
    fn render_enum_constants(&self, ctx: &mut Context<'_, '_>) {
        let Some(base) = &self.simple_base_type else {
            return;
        };
        let base = ctx.resolve_type_for_module(base);

        let variants = self
            .variants
            .iter()
            .filter_map(|var| var.render_enum_value(ctx, &base))
            .collect::<Vec<_>>();
        if variants.is_empty() {
            return;
        }

        let type_ident = &self.type_ident;

        let code = quote! {
            impl #type_ident {
                #( #variants )*
            }
        };

        ctx.current_module().append(code);
    }
}

impl EnumerationDataVariant<'_> {
    fn render_enum_value(&self, ctx: &Context<'_, '_>, base: &TokenStream) -> Option<TokenStream> {
        match &self.value {
            EnumerationVariantValue::None => None,
            EnumerationVariantValue::ByteLiteral(ident, renderer) => {
                let value = renderer.render(ctx);
                let value = value.to_string();
                let value = value.trim_start_matches('b');
                let value = TokenStream::from_str(value).unwrap();
                let str_ = ctx.resolve_build_in("::core::primitive::str");

                Some(quote! {
                    pub const #ident: &#str_ = #value;
                })
            }
            EnumerationVariantValue::Constant(ident, renderer) => {
                let value = renderer.render(ctx);

                Some(quote! {
                    pub const #ident: &'static #base = &#value;
                })
            }
        }
    }
}
