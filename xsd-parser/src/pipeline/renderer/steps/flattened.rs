use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::config::RendererFlags;
use crate::models::data::{
    ComplexData, ComplexDataEnum, DataType, DataTypeVariant, Occurs, UnionData,
};
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
pub struct FlattenedContentHelpersRenderStep;

impl RenderStep for FlattenedContentHelpersRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::ExtraImpls
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        if !ctx.meta.check_renderer_flags(RendererFlags::FLATTENED_CONTENT_HELPERS) {
            return;
        }

        let DataTypeVariant::Complex(complex) = &ctx.data.variant else {
            return;
        };
        let ComplexData::Struct { type_, .. } = complex else {
            return;
        };
        let Some(content) = type_.content() else {
            return;
        };
        if content.occurs != Occurs::DynamicList {
            return;
        }

        let content_rust_ident = content.target_type.path.ident();

        let Some(content_dt) =
            find_data_type_by_rust_ident(ctx.meta.types.items.values(), content_rust_ident)
        else {
            return;
        };

        let impl_block = match &content_dt.variant {
            DataTypeVariant::Union(union_data) => {
                render_helpers_for_union(ctx, &type_.base.type_ident, &content.field_ident, union_data)
            }
            DataTypeVariant::Complex(ComplexData::Enum { type_: enum_type, .. }) => {
                render_helpers_for_complex_enum(ctx, &type_.base.type_ident, &content.field_ident, enum_type)
            }
            _ => return,
        };

        ctx.current_module().append(impl_block);
    }
}

fn render_helpers_for_union(
    ctx: &Context<'_, '_>,
    struct_ident: &proc_macro2::Ident,
    content_field_ident: &proc_macro2::Ident,
    union: &UnionData<'_>,
) -> TokenStream {
    let enum_ident = &union.type_ident;

    // Generate one helper per variant:
    //   pub fn private_note(&self) -> Option<&T> {
    //       self.content.iter().find_map(|x| match x { Enum::PrivateNote(v) => Some(v), _ => None })
    //   }
    let methods = union.variants.iter().map(|v| {
        let variant_ident = &v.variant_ident;

        // method name: snake_case of the variant ident.
        // We keep it simple/minimal here: lower-case the ident string.
        // If you want proper casing (PrivateNote -> private_note), we can use `inflector`
        // like other renderer pieces do, but I’m keeping dependencies minimal in this step.
        let method_name_str = variant_ident.to_string().to_snake_case();
        let method_ident = format_ident!("{}", method_name_str);

        // Return type is Option<&TargetType>.
        // We render the same target type the enum variant already uses.
        let target_ty = &v.target_type;
        let target_ty = ctx.resolve_type_for_module(target_ty);

        let option = ctx.resolve_build_in("::core::option::Option");

        quote! {
            #[inline]
            pub fn #method_ident(&self) -> #option<&#target_ty> {
                self.#content_field_ident.iter().find_map(|x| {
                    match x {
                        #enum_ident::#variant_ident(v) => #option::Some(v),
                        _ => #option::None,
                    }
                })
            }
        }
    });

    quote! {
        impl #struct_ident {
            #( #methods )*
        }
    }
}

/// Find a [`DataType`] in the types collection by its rendered Rust type identifier.
fn find_data_type_by_rust_ident<'a, I>(
    items: I,
    rust_ident: &proc_macro2::Ident,
) -> Option<&'a DataType<'a>>
where
    I: IntoIterator<Item = &'a DataType<'a>>,
{
    items.into_iter().find(|dt| {
        let candidate = match &dt.variant {
            DataTypeVariant::Complex(c) => match c {
                ComplexData::Enum { type_, .. } => &type_.base.type_ident,
                ComplexData::Struct { type_, .. } => &type_.base.type_ident,
            },
            DataTypeVariant::Union(u) => &u.type_ident,
            DataTypeVariant::Enumeration(e) => &e.type_ident,
            _ => return false,
        };
        candidate == rust_ident
    })
}

/// Generate helper accessor methods for a [`ComplexDataEnum`] content type.
fn render_helpers_for_complex_enum(
    ctx: &Context<'_, '_>,
    struct_ident: &proc_macro2::Ident,
    content_field_ident: &proc_macro2::Ident,
    enum_type: &ComplexDataEnum<'_>,
) -> TokenStream {
    let enum_ident = &enum_type.base.type_ident;

    let methods = enum_type.elements.iter().map(|e| {
        let variant_ident = &e.variant_ident;
        let method_ident = snake_case_ident(variant_ident);
        let target_ty = ctx.resolve_type_for_module(&e.target_type);
        let option = ctx.resolve_build_in("::core::option::Option");

        quote! {
            #[inline]
            pub fn #method_ident(&self) -> #option<&#target_ty> {
                self.#content_field_ident.iter().find_map(|x| {
                    match x {
                        #enum_ident::#variant_ident(v) => #option::Some(v),
                        _ => #option::None,
                    }
                })
            }
        }
    });

    quote! {
        impl #struct_ident {
            #( #methods )*
        }
    }
}

/// Convert an identifier to a snake_case method name identifier.
fn snake_case_ident(ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    format_ident!("{}", ident.to_string().to_snake_case())
}
