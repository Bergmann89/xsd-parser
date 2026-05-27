use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::models::data::{ComplexData, ComplexDataEnum, DataTypeVariant, Occurs};
use crate::pipeline::renderer::{Context, RenderStep, RenderStepType};

/// RenderStep that generates ergonomic helper accessors for flattened struct content.
///
/// This targets the pattern:
///   pub struct Foo { pub content: Vec<FooContent>, ... }
///   pub enum FooContent { PrivateNote(String), ... }
///
/// and generates:
///   impl Foo {
///     pub fn private_note(&self) -> Option<&String> { ... }
///   }
/// Enabled via [`RendererFlags::FLATTENED_CONTENT_HELPERS`].
#[derive(Debug, Clone, Copy)]
pub struct ContentHelpersRenderStep;

impl RenderStep for ContentHelpersRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::ExtraImpls
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        let DataTypeVariant::Complex(complex) = &ctx.data.variant else {
            return;
        };
        let ComplexData::Struct {
            type_,
            content_type,
        } = complex
        else {
            return;
        };
        let Some(content) = type_.content() else {
            return;
        };
        if content.occurs != Occurs::DynamicList {
            return;
        };

        // The content enum is stored inline as content_type of ComplexData::Struct
        let Some(content_type) = content_type else {
            return;
        };

        let impl_block = match content_type.as_ref() {
            ComplexData::Enum {
                type_: enum_type, ..
            } => render_helpers_for_complex_enum(
                ctx,
                &type_.base.type_ident,
                &content.field_ident,
                enum_type,
            ),
            ComplexData::Struct { .. } => {
                // Struct content (e.g. a sequence) - no enum variants to flatten
                return;
            }
        };

        ctx.current_module().append(impl_block);
    }
}

/// Generate helper accessor methods for a [`ComplexDataEnum`] content type.
fn render_helpers_for_complex_enum(
    ctx: &Context<'_, '_>,
    struct_ident: &proc_macro2::Ident,
    content_field_ident: &proc_macro2::Ident,
    enum_type: &ComplexDataEnum<'_>,
) -> TokenStream {
    let enum_ident = &enum_type.base.type_ident;

    let methods = enum_type.elements.iter().filter_map(|e| {
        if e.occurs != Occurs::Single {
            return None;
        }
        let variant_ident = &e.variant_ident;
        let method_ident = &e.field_ident;
        let mut_method_ident = format_ident!("{method_ident}_mut");
        let target_ty = ctx.resolve_type_for_module(&e.target_type);
        let option = ctx.resolve_build_in("::core::option::Option");

        let out = quote! {
            #[inline]
            pub fn #method_ident(&self) -> #option<&#target_ty> {
                self.#content_field_ident.iter().find_map(|x| {
                    match x {
                        #enum_ident::#variant_ident(v) => #option::Some(v),
                        _ => #option::None,
                    }
                })
            }
            #[inline]
            pub fn #mut_method_ident(&mut self) -> #option<&mut #target_ty> {
                self.#content_field_ident.iter_mut().find_map(|x| {
                    match x {
                        #enum_ident::#variant_ident(v) => #option::Some(v),
                        _ => #option::None,
                    }
                })
            }
        };
        Some(out)
    });

    quote! {
        impl #struct_ident {
            #( #methods )*
        }
    }
}