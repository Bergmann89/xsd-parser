use std::fmt::Display;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote, ToTokens};
use smallvec::{smallvec, SmallVec};

use crate::config::{RendererFlags, SerdeSupport, TypedefMode};
use crate::models::{
    data::{
        ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement, ComplexDataEnum,
        ComplexDataStruct, CustomData, DynamicData, EnumerationData, EnumerationTypeVariant,
        Occurs, ReferenceData, UnionData, UnionTypeVariant,
    },
    schema::xs::Use,
};

use super::super::{Context, DataTypeVariant, RenderStep};

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

/* Context */

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

/* CustomType */

impl CustomData<'_> {
    pub(crate) fn render_types(&self, ctx: &mut Context<'_, '_>) {
        let Some(include) = self.meta.include() else {
            return;
        };

        ctx.module().usings([include]);
    }
}

/* UnionType */

impl UnionData<'_> {
    pub(crate) fn render_types(&self, ctx: &mut Context<'_, '_>) {
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
    pub(crate) fn render_types(&self, ctx: &mut Context<'_, '_>) {
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
            || get_dyn_type_traits(ctx),
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
    pub(crate) fn render_types(&self, ctx: &mut Context<'_, '_>) {
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
    pub(crate) fn render_types(&self, ctx: &mut Context<'_, '_>) {
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
            info,
            variant_ident,
            target_type,
        } = self;

        let docs = ctx.render_docs(RendererFlags::RENDER_VARIANT_DOCS, &info.documentation[..]);

        let serde = if ctx.serde_support == SerdeSupport::None {
            None
        } else if info.type_.is_some() {
            Some(quote!(#[serde(other)]))
        } else {
            let name = format!("{}", info.ident.name);

            Some(quote!(#[serde(rename = #name)]))
        };

        let target_type = target_type.as_ref().map(|target_type| {
            let target_type = ctx.resolve_type_for_module(target_type);

            quote!((#target_type))
        });

        quote! {
            #docs
            #serde
            #variant_ident #target_type,
        }
    }
}

/* ComplexType */

impl ComplexData<'_> {
    pub(crate) fn render_types(&self, ctx: &mut Context<'_, '_>) {
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

        let attributes = self
            .attributes
            .iter()
            .map(|x| x.render_field(ctx, type_ident));
        let fields = self.elements().iter().map(|x| x.render_field(ctx));
        let content = self.content().as_ref().and_then(|x| x.render_field(ctx));

        let struct_data = if self.is_unit_struct() {
            quote!(;)
        } else {
            quote! {
                {
                    #( #attributes )*
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
        let serde_default = (self.min_occurs == 0).then(|| quote!(default,));
        let serde = match (ctx.serde_support, self.is_simple) {
            (SerdeSupport::None, _) => None,
            (SerdeSupport::QuickXml, true) => {
                Some(quote!(#[serde(#serde_default rename = "$text")]))
            }
            (_, _) => Some(quote!(#[serde(#serde_default rename = "$value")])),
        };

        Some(quote! {
            #serde
            pub content: #target_type,
        })
    }
}

impl ComplexDataAttribute<'_> {
    fn render_field(&self, ctx: &Context<'_, '_>, type_ident: &Ident2) -> TokenStream {
        let field_ident = &self.ident;
        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let default = if self.default_value.is_some() {
            let default_path = format!("{type_ident}::default_{field_ident}");

            quote!(default = #default_path,)
        } else if self.info.use_ == Use::Optional {
            quote!(default,)
        } else {
            quote!()
        };

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ATTRIBUTE_DOCS,
            &self.info.documentation[..],
        );

        let serde = match ctx.serde_support {
            SerdeSupport::None => None,
            SerdeSupport::QuickXml => {
                let name = format!("@{}", self.info.ident.name);

                Some(quote!(#[serde(#default rename = #name)]))
            }
            SerdeSupport::SerdeXmlRs => {
                let name = format!("{}", self.info.ident.name);

                Some(quote!(#[serde(#default rename = #name)]))
            }
        };

        if let Some(ty) = self
            .info
            .is_any()
            .then_some(())
            .and(ctx.any_attribute_type.as_ref())
        {
            ctx.add_usings([ty.to_token_stream()]);
        }

        if self.is_option {
            quote! {
                #docs
                #serde
                pub #field_ident: Option<#target_type>,
            }
        } else {
            quote! {
                #docs
                #serde
                pub #field_ident: #target_type,
            }
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
            &self.info.documentation[..],
        );

        let serde = match ctx.serde_support {
            SerdeSupport::None => None,
            SerdeSupport::QuickXml | SerdeSupport::SerdeXmlRs => {
                let name = self.info.ident.name.to_string();
                let default = match self.occurs {
                    Occurs::None | Occurs::Single | Occurs::StaticList(_) => quote!(),
                    Occurs::Optional | Occurs::DynamicList => quote!(default,),
                };

                Some(quote!(#[serde(#default rename = #name)]))
            }
        };

        if let Some(ty) = self.info.is_any().then_some(()).and(ctx.any_type.as_ref()) {
            ctx.add_usings([ty.to_token_stream()]);
        }

        quote! {
            #docs
            #serde
            pub #field_ident: #target_type,
        }
    }

    fn render_variant(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let variant_ident = &self.variant_ident;
        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self.occurs.make_type(&target_type, self.need_indirection);

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ELEMENT_DOCS,
            &self.info.documentation[..],
        );

        let serde = match ctx.serde_support {
            SerdeSupport::None => None,
            SerdeSupport::QuickXml | SerdeSupport::SerdeXmlRs => {
                let name = self.info.ident.name.to_string();

                Some(quote!(#[serde(rename = #name)]))
            }
        };

        quote! {
            #docs
            #serde
            #variant_ident(#target_type),
        }
    }
}

/* Helper */

fn get_derive<I>(ctx: &Context<'_, '_>, extra: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: Display,
{
    let serde: SmallVec<[Ident2; 2]> = if ctx.serde_support == SerdeSupport::None {
        smallvec![]
    } else {
        ctx.add_usings([quote!(serde::Serialize), quote!(serde::Deserialize)]);

        smallvec![format_ident!("Serialize"), format_ident!("Deserialize")]
    };

    let extra = extra.into_iter().map(|x| format_ident!("{x}"));
    let types = ctx
        .derive
        .iter()
        .cloned()
        .chain(serde)
        .chain(extra)
        .collect::<Vec<_>>();

    if types.is_empty() {
        quote! {}
    } else {
        quote! {
            #[derive( #( #types ),* )]
        }
    }
}

fn get_dyn_type_traits(ctx: &Context<'_, '_>) -> TokenStream {
    format_traits(ctx.dyn_type_traits.iter().map(|ident| {
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
