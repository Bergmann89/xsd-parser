mod defaults;
mod namespace_const;
mod quick_xml;
mod serde;
mod types;
mod with_namespace_trait;

use std::fmt::Display;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::config::RendererFlags;
use crate::models::code::IdentPath;
use crate::models::data::ConfigValue;

use super::Context;

pub use self::defaults::DefaultsRenderStep;
pub use self::namespace_const::NamespaceConstantsRenderStep;
pub use self::quick_xml::{QuickXmlDeserializeRenderStep, QuickXmlSerializeRenderStep};
pub use self::serde::{
    SerdeQuickXmlTypesRenderStep, SerdeXmlRsV7TypesRenderStep, SerdeXmlRsV8TypesRenderStep,
};
pub use self::types::TypesRenderStep;
pub use self::with_namespace_trait::WithNamespaceTraitRenderStep;

impl Context<'_, '_> {
    pub(crate) fn render_type_docs(&self) -> Option<TokenStream> {
        self.render_docs(
            RendererFlags::RENDER_TYPE_DOCS,
            &self.data.documentation[..],
        )
    }

    pub(crate) fn render_docs(&self, flags: RendererFlags, docs: &[String]) -> Option<TokenStream> {
        self.check_renderer_flags(flags).then(|| {
            let docs = docs.iter().flat_map(|s| s.split('\n')).map(|s| {
                let s = s.trim_end();

                quote!(#[doc = #s])
            });

            quote!(#( #docs )*)
        })
    }
}

fn get_derive<I>(ctx: &Context<'_, '_>, extra: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: Display,
{
    let extra = extra.into_iter().map(|x| format_ident!("{x}"));
    let types = ctx.derive.iter().cloned().chain(extra);

    let types = match &ctx.data.derive {
        ConfigValue::Default => types.collect::<Vec<_>>(),
        ConfigValue::Extend(extra) => types.chain(extra.iter().cloned()).collect::<Vec<_>>(),
        ConfigValue::Overwrite(types) => types.clone(),
    };

    if types.is_empty() {
        quote! {}
    } else {
        quote! {
            #[derive( #( #types ),* )]
        }
    }
}

fn get_dyn_type_traits<'a, I>(ctx: &'a Context<'_, '_>, extra: I) -> TokenStream
where
    I: IntoIterator<Item = &'a IdentPath>,
{
    format_traits(ctx.dyn_type_traits.iter().chain(extra).map(|ident| {
        ctx.add_usings([quote!(#ident)]);

        let ident = ident.ident();

        quote!(#ident)
    }))
}

fn format_traits<I>(iter: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: ToTokens,
{
    let parts = iter
        .into_iter()
        .enumerate()
        .map(|(i, x)| if i == 0 { quote!(#x) } else { quote!(+ #x) });

    quote! {
        #( #parts )*
    }
}

fn render_trait_impls<'a>(
    type_ident: &'a Ident2,
    trait_idents: &'a [TokenStream],
) -> impl Iterator<Item = TokenStream> + 'a {
    trait_idents.iter().map(move |trait_ident| {
        quote! {
            impl #trait_ident for #type_ident { }
        }
    })
}

fn make_mixed(is_mixed: bool, tokens: TokenStream) -> TokenStream {
    if is_mixed {
        quote!(Mixed<#tokens>)
    } else {
        tokens
    }
}
