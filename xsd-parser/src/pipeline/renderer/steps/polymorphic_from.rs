use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::config::{GeneratorFlags, TypedefMode};
use crate::models::{
    code::IdentPath,
    data::{ComplexData, ComplexDataEnum, DataTypeVariant, Occurs, PathData},
    meta::{ElementMetaVariant, MetaTypeVariant},
    TypeIdent,
};

use super::super::{Context, RenderStep, RenderStepType};

/// Generate `From<Concrete>` implementations for existing polymorphic choices.
///
/// This step works independently of the type and serialization renderers. It
/// follows nested content types so unflattened choices receive their conversions
/// on the content enum. Ordinary choices and dynamic trait objects are unchanged.
///
/// See [`config::RenderStep::PolymorphicFrom`](crate::config::RenderStep::PolymorphicFrom)
/// for configuration and ambiguity rules, including the limitation that aliases
/// defined in external Rust code cannot be detected.
#[derive(Debug, Clone, Copy)]
pub struct PolymorphicFromRenderStep;

impl RenderStep for PolymorphicFromRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::ExtraImpls
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        if let DataTypeVariant::Complex(complex) = &ctx.data.variant {
            complex.render_polymorphic_from(ctx);
        }
    }
}

impl ComplexData<'_> {
    fn render_polymorphic_from(&self, ctx: &mut Context<'_, '_>) {
        let content_type = match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_polymorphic_from(ctx);
                content_type
            }
            Self::Struct { content_type, .. } => content_type,
        };
        if let Some(content_type) = content_type {
            content_type.render_polymorphic_from(ctx);
        }
    }
}

impl ComplexDataEnum<'_> {
    fn render_polymorphic_from(&self, ctx: &mut Context<'_, '_>) {
        if !self.is_dynamic() {
            return;
        }

        // Compare conservative keys of the underlying Rust types, including
        // typedefs. Qualifiers can spell the same external type in different ways.
        let keys = self
            .elements
            .iter()
            .map(|element| {
                let ElementMetaVariant::Type { type_: ident, .. } = &element.meta().variant else {
                    return None;
                };
                (element.occurs == Occurs::Single).then(|| {
                    source_key(ctx, ident, &element.target_type, &mut Vec::new()).to_string()
                })
            })
            .collect::<Vec<_>>();
        let mut counts = HashMap::new();
        for key in keys.iter().flatten() {
            *counts.entry(key).or_insert(0usize) += 1;
        }

        let type_ident = &self.type_ident;
        let from = ctx.resolve_build_in("::core::convert::From");
        let mut code = TokenStream::new();
        for (element, key) in self.elements.iter().zip(&keys) {
            if key.as_ref().is_none_or(|key| counts[key] != 1) {
                continue;
            }
            let target_type = ctx.resolve_type_for_module(&element.target_type);
            let variant = &element.variant_ident;
            let value = if element.need_indirection {
                let box_ = ctx.resolve_build_in("::alloc::boxed::Box");
                quote!(#box_::new(value))
            } else {
                quote!(value)
            };
            code.extend(quote! {
                impl #from<#target_type> for #type_ident {
                    fn from(value: #target_type) -> Self {
                        Self::#variant(#value)
                    }
                }
            });
        }
        ctx.current_module().append(code);
    }
}

// Resolve generated aliases, preserving wrappers and occurrence. External paths
// are compared by their final identifiers: we cannot resolve Rust imports here,
// and suppressing a conversion is preferable to emitting conflicting impls.
// Keep a visited list because native aliases use self-referential placeholder
// metadata, and custom generator data may contain reference cycles.
fn source_key(
    ctx: &Context<'_, '_>,
    ident: &TypeIdent,
    path: &PathData,
    visited: &mut Vec<TypeIdent>,
) -> TokenStream {
    if !visited.contains(ident) {
        if let Some(data) = ctx.types.items.get(ident) {
            if let DataTypeVariant::Reference(reference) = &data.variant {
                if reference.mode == TypedefMode::NewType {
                    return path_key(path);
                }
                if reference.mode == TypedefMode::Typedef {
                    visited.push(ident.clone());
                    let nillable = reference.meta.nillable
                        && ctx.check_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT);
                    let target = match (&data.meta.variant, &reference.target_type.generics[..]) {
                        (MetaTypeVariant::Reference(_), [inner]) if nillable => {
                            let target = source_key(
                                ctx,
                                &reference.meta.type_,
                                &PathData::from_path(inner.clone()),
                                visited,
                            );
                            let mut wrapper = reference.target_type.clone();
                            wrapper.generics.clear();
                            let wrapper = path_key(&wrapper);
                            quote!(#wrapper<#target>)
                        }
                        (MetaTypeVariant::Reference(_), []) => {
                            source_key(ctx, &reference.meta.type_, &reference.target_type, visited)
                        }
                        _ => path_key(&reference.target_type),
                    };
                    return match reference.occurs {
                        Occurs::None => TokenStream::new(),
                        Occurs::Single => target,
                        Occurs::Optional => quote!(Option<#target>),
                        Occurs::DynamicList => quote!(Vec<#target>),
                        Occurs::StaticList(size) => quote!([#target; #size]),
                    };
                }
            }
        }
    }
    if path.generics.is_empty() {
        if let Some(meta) = ctx.types.meta.types.get_resolved_type(ident) {
            if let MetaTypeVariant::BuildIn(builtin) = &meta.variant {
                let ident = builtin.ident_path();
                return quote!(#ident);
            }
        }
    }
    path_key(path)
}

fn path_key(path: &PathData) -> TokenStream {
    let ident = path.path.ident();
    if path.generics.is_empty() {
        quote!(#ident)
    } else {
        let generics = path.generics.iter().map(IdentPath::ident);
        quote!(#ident<#(#generics),*>)
    }
}
