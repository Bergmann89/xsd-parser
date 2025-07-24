use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote};

use crate::config::{RendererFlags, TypedefMode};
use crate::models::{
    data::{
        ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement, ComplexDataEnum,
        ComplexDataStruct, CustomData, DynamicData, EnumerationData, EnumerationTypeVariant,
        Occurs, ReferenceData, UnionData, UnionTypeVariant,
    },
    meta::{MetaType, MetaTypeVariant},
    schema::xs::Use,
    Ident,
};

use super::super::super::{Context, DataTypeVariant, RenderStep};
use super::super::{format_traits, render_trait_impls};
use super::{get_derive, get_dyn_type_traits};

/// Implements a [`RenderStep`] that renders rust types of the types defined in
/// the schema with `serde-xml-rs >= 0.8` support.
#[derive(Debug)]
pub struct SerdeXmlRsV8TypesRenderStep;

impl RenderStep for SerdeXmlRsV8TypesRenderStep {
    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) => (),
            DataTypeVariant::Custom(x) => x.render_type_serde_xml_rs_v8(ctx),
            DataTypeVariant::Union(x) => x.render_type_serde_xml_rs_v8(ctx),
            DataTypeVariant::Dynamic(x) => x.render_type_serde_xml_rs_v8(ctx),
            DataTypeVariant::Reference(x) => x.render_type_serde_xml_rs_v8(ctx),
            DataTypeVariant::Enumeration(x) => x.render_type_serde_xml_rs_v8(ctx),
            DataTypeVariant::Complex(x) => x.render_type_serde_xml_rs_v8(ctx),
        }
    }
}

/* CustomType */

impl CustomData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>) {
        let Some(include) = self.meta.include() else {
            return;
        };

        ctx.add_usings([include]);
    }
}

/* Context */

impl<'types> Context<'_, 'types> {
    fn get_resolved_complex_content(&self) -> Option<(&'types Ident, &'types MetaType)> {
        let MetaTypeVariant::ComplexType(cm) = &self.data.meta.variant else {
            return None;
        };

        let content_ident = cm.content.as_ref()?;

        self.meta.types.meta.types.get_resolved(content_ident)
    }
}

/* Occurs */

impl Occurs {
    fn array_to_vec(self) -> Self {
        match self {
            Self::StaticList(_) => Self::DynamicList,
            x => x,
        }
    }
}

/* UnionType */

impl UnionData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>) {
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
            .map(|x| x.render_variant_serde_xml_rs_v8(ctx));

        let code = quote! {
            #docs
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        };

        ctx.current_module().append(code);
    }
}

/* UnionTypeVariant */

impl UnionTypeVariant<'_> {
    fn render_variant_serde_xml_rs_v8(&self, ctx: &Context<'_, '_>) -> TokenStream {
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
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>) {
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

        ctx.current_module().append(code);
    }
}

/* ReferenceType */

impl ReferenceData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            mode,
            occurs,
            type_ident,
            target_type,
            trait_impls,
            ..
        } = self;

        let docs = ctx.render_type_docs();
        let occurs = occurs.array_to_vec();
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

        ctx.current_module().append(code);
    }
}

/* EnumerationType */

impl EnumerationData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            trait_impls,
            ..
        } = self;

        let values_ident = format_enum_values_ident(ctx.ident);

        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let trait_impls = render_trait_impls(type_ident, trait_impls);

        let variants = variants
            .iter()
            .map(|d| d.render_variant_serde_xml_rs_v8(ctx))
            .collect::<Vec<_>>();

        ctx.add_usings([quote!(core::ops::Deref), quote!(core::ops::DerefMut)]);

        let code = quote! {
            #docs
            #derive
            pub struct #type_ident {
                #[serde(rename = "#text")]
                pub value: #values_ident,
            }

            impl From<#values_ident> for #type_ident {
                fn from(value: #values_ident) -> Self {
                    Self { value }
                }
            }

            impl From<#type_ident> for #values_ident {
                fn from(value: #type_ident) -> Self {
                    value.value
                }
            }

            impl Deref for #type_ident {
                type Target = #values_ident;

                fn deref(&self) -> &Self::Target {
                    &self.value
                }
            }

            impl DerefMut for #type_ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.value
                }
            }

            #derive
            pub enum #values_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        };

        ctx.current_module().append(code);
    }
}

impl EnumerationTypeVariant<'_> {
    fn render_variant_serde_xml_rs_v8(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let Self {
            meta,
            s_name,
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
            #[serde(rename = #s_name)]
            #variant_ident #target_type,
        }
    }
}

/* ComplexType */

impl ComplexData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_type_serde_xml_rs_v8(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_type_serde_xml_rs_v8(ctx);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_type_serde_xml_rs_v8(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_type_serde_xml_rs_v8(ctx);
                }
            }
        }
    }
}

impl ComplexDataEnum<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>) {
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(type_ident, &self.trait_impls);

        let variants = self
            .elements
            .iter()
            .map(|x| x.render_variant_serde_xml_rs_v8(ctx));

        let code = quote! {
            #docs
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        };

        ctx.current_module().append(code);
    }
}

impl ComplexDataStruct<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>) {
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(type_ident, &self.trait_impls);

        let attributes = self
            .attributes
            .iter()
            .map(|x| x.render_field_serde_xml_rs_v8(ctx, type_ident));
        let fields = self
            .elements()
            .iter()
            .map(|x| x.render_field_serde_xml_rs_v8(ctx));
        let content = self
            .content()
            .as_ref()
            .and_then(|x| x.render_field_serde_xml_rs_v8(ctx));

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

        ctx.current_module().append(code);
    }
}

impl ComplexDataContent<'_> {
    fn render_field_serde_xml_rs_v8(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self.occurs.array_to_vec().make_type(&target_type, false)?;

        let default =
            (self.is_empty_string_content(ctx) || self.min_occurs == 0).then(|| quote!(default,));

        match ctx
            .get_resolved_complex_content()
            .map(|(ident, meta)| (ident, &meta.variant))
        {
            Some((ident, MetaTypeVariant::Enumeration(_))) => {
                let enum_values_type = format_enum_values_ident(ident);

                Some(quote! {
                    #[serde(#default rename = "#text")]
                    pub content: #enum_values_type,
                })
            }
            Some((_, MetaTypeVariant::BuildIn(_) | MetaTypeVariant::Reference(_))) => {
                Some(quote! {
                    #[serde(#default rename = "#text")]
                    pub content: #target_type,
                })
            }
            None | Some((_, _)) => Some(quote! {
                #[serde(#default rename = "#content")]
                pub content: #target_type,
            }),
        }
    }
}

impl ComplexDataAttribute<'_> {
    fn render_field_serde_xml_rs_v8(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
    ) -> TokenStream {
        let Self {
            tag_name,
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

        let name = format!("@{tag_name}");

        quote! {
            #docs
            #[serde(#default rename = #name)]
            pub #field_ident: #target_type,
        }
    }
}

impl ComplexDataElement<'_> {
    fn render_field_serde_xml_rs_v8(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let Self {
            tag_name,
            field_ident,
            ..
        } = self;

        let name = if self.meta().is_text() {
            "#text"
        } else {
            tag_name
        };

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self
            .occurs
            .array_to_vec()
            .make_type(&target_type, self.need_indirection)
            .unwrap();

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ELEMENT_DOCS,
            &self.meta().documentation[..],
        );

        let default = match self.occurs.array_to_vec() {
            Occurs::None | Occurs::Single | Occurs::StaticList(_) => quote!(),
            Occurs::Optional | Occurs::DynamicList => quote!(default,),
        };

        quote! {
            #docs
            #[serde(#default rename = #name)]
            pub #field_ident: #target_type,
        }
    }

    fn render_variant_serde_xml_rs_v8(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let Self {
            tag_name,
            variant_ident,
            ..
        } = self;

        let name = if self.meta().is_text() {
            "#text"
        } else {
            tag_name
        };

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self
            .occurs
            .array_to_vec()
            .make_type(&target_type, self.need_indirection);

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

fn format_enum_values_ident(ident: &Ident) -> Ident2 {
    let values_ident = ident.name.to_type_name();

    format_ident!("{values_ident}Value")
}
