use proc_macro2::{Ident as Ident2, TokenStream};
use quote::quote;

use crate::config::{RendererFlags, TypedefMode};
use crate::models::{
    data::{
        ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement, ComplexDataEnum,
        ComplexDataStruct, CustomData, DynamicData, EnumerationData, EnumerationTypeVariant,
        Occurs, ReferenceData, UnionData, UnionTypeVariant,
    },
    schema::xs::Use,
};

use super::super::super::{Context, DataTypeVariant, RenderStep};
use super::super::{format_traits, render_trait_impls};
use super::{get_derive, get_dyn_type_traits};

/// Implements a [`RenderStep`] that renders rust types of the types defined in
/// the schema with `serde-xml-rs <= 0.7` support.
#[derive(Debug)]
pub struct SerdeXmlRsV7TypesRenderStep;

impl RenderStep for SerdeXmlRsV7TypesRenderStep {
    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) => (),
            DataTypeVariant::Custom(x) => x.render_type_serde_xml_rs_v7(ctx),
            DataTypeVariant::Union(x) => x.render_type_serde_xml_rs_v7(ctx),
            DataTypeVariant::Dynamic(x) => x.render_type_serde_xml_rs_v7(ctx),
            DataTypeVariant::Reference(x) => x.render_type_serde_xml_rs_v7(ctx),
            DataTypeVariant::Enumeration(x) => x.render_type_serde_xml_rs_v7(ctx),
            DataTypeVariant::Complex(x) => x.render_type_serde_xml_rs_v7(ctx),
        }
    }
}

/* CustomType */

impl CustomData<'_> {
    fn render_type_serde_xml_rs_v7(&self, ctx: &mut Context<'_, '_>) {
        let Some(include) = self.meta.include() else {
            return;
        };

        ctx.add_usings([include]);
    }
}

/* UnionType */

impl UnionData<'_> {
    fn render_type_serde_xml_rs_v7(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            trait_impls,
            variants,
            ..
        } = self;
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let trait_impls = render_trait_impls(type_ident, trait_impls);
        let variants = variants
            .iter()
            .map(|x| x.render_variant_serde_xml_rs_v7(ctx));

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
    fn render_variant_serde_xml_rs_v7(&self, ctx: &Context<'_, '_>) -> TokenStream {
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
    fn render_type_serde_xml_rs_v7(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            trait_ident,
            sub_traits,
            ..
        } = self;

        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
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
    fn render_type_serde_xml_rs_v7(&self, ctx: &mut Context<'_, '_>) {
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
    fn render_type_serde_xml_rs_v7(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            trait_impls,
            ..
        } = self;

        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let trait_impls = render_trait_impls(type_ident, trait_impls);

        let variants = variants
            .iter()
            .map(|d| d.render_variant_serde_xml_rs_v7(ctx))
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
    fn render_variant_serde_xml_rs_v7(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let Self {
            meta,
            s_name,
            variant_ident,
            target_type,
            ..
        } = self;

        let docs = ctx.render_docs(RendererFlags::RENDER_VARIANT_DOCS, &meta.documentation[..]);

        let serde = if meta.type_.is_some() {
            quote!(#[serde(other)])
        } else {
            quote!(#[serde(rename = #s_name)])
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
    fn render_type_serde_xml_rs_v7(&self, ctx: &mut Context<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_type_serde_xml_rs_v7(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_type_serde_xml_rs_v7(ctx);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_type_serde_xml_rs_v7(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_type_serde_xml_rs_v7(ctx);
                }
            }
        }
    }
}

impl ComplexDataEnum<'_> {
    fn render_type_serde_xml_rs_v7(&self, ctx: &mut Context<'_, '_>) {
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(type_ident, &self.trait_impls);

        let variants = self
            .elements
            .iter()
            .map(|x| x.render_variant_serde_xml_rs_v7(ctx));

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
    fn render_type_serde_xml_rs_v7(&self, ctx: &mut Context<'_, '_>) {
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(type_ident, &self.trait_impls);

        let attributes = self
            .attributes
            .iter()
            .map(|x| x.render_field_serde_xml_rs_v7(ctx, type_ident));
        let fields = self
            .elements()
            .iter()
            .map(|x| x.render_field_serde_xml_rs_v7(ctx));
        let content = self
            .content()
            .as_ref()
            .and_then(|x| x.render_field_serde_xml_rs_v7(ctx));

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
    fn render_field_serde_xml_rs_v7(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self.occurs.make_type(&target_type, false)?;

        let default = (self.min_occurs == 0).then(|| quote!(default,));

        Some(quote! {
            #[serde(#default rename = "$value")]
            pub content: #target_type,
        })
    }
}

impl ComplexDataAttribute<'_> {
    fn render_field_serde_xml_rs_v7(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
    ) -> TokenStream {
        let Self {
            s_name,
            ident: field_ident,
            ..
        } = self;

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = if self.is_option {
            quote!(Option<#target_type>)
        } else {
            target_type
        };

        let default = if self.default_value.is_some() {
            let default_path = format!("{type_ident}::default_{field_ident}");

            quote!(default = #default_path,)
        } else if self.meta.use_ == Use::Optional {
            quote!(default,)
        } else {
            quote!()
        };

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ATTRIBUTE_DOCS,
            &self.meta.documentation[..],
        );

        quote! {
            #docs
            #[serde(#default rename = #s_name)]
            pub #field_ident: #target_type,
        }
    }
}

impl ComplexDataElement<'_> {
    fn render_field_serde_xml_rs_v7(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let Self {
            s_name,
            field_ident,
            ..
        } = self;

        let name = if self.meta().is_text() {
            "#text"
        } else {
            s_name
        };

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self
            .occurs
            .make_type(&target_type, self.need_indirection)
            .unwrap();

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ELEMENT_DOCS,
            &self.meta().documentation[..],
        );

        let default = match self.occurs {
            Occurs::None | Occurs::Single | Occurs::StaticList(_) => quote!(),
            Occurs::Optional | Occurs::DynamicList => quote!(default,),
        };

        quote! {
            #docs
            #[serde(#default rename = #name)]
            pub #field_ident: #target_type,
        }
    }

    fn render_variant_serde_xml_rs_v7(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let Self {
            s_name,
            variant_ident,
            ..
        } = self;

        let name = if self.meta().is_text() {
            "#text"
        } else {
            s_name
        };

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self.occurs.make_type(&target_type, self.need_indirection);

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ELEMENT_DOCS,
            &self.meta().documentation[..],
        );

        quote! {
            #docs
            #[serde(rename = #name)]
            #variant_ident(#target_type),
        }
    }
}
