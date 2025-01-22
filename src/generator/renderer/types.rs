use std::fmt::Display;
use std::ops::Not;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote};
use smallvec::{smallvec, SmallVec};
use tracing::instrument;

use crate::schema::xs::Use;

use super::super::data::{
    AbstractData, AttributeData, ComplexTypeData, ElementData, EnumVariantData, EnumerationData,
    ReferenceData, TypeData, UnionData, UnionVariantData,
};
use super::super::misc::{BoxFlags, GenerateFlags, Occurs, StateFlags, TypeMode, TypedefMode};
use super::super::{Generator, SerdeSupport};

pub(crate) struct TypeRenderer;

impl TypeRenderer {
    #[instrument(level = "trace", skip(self))]
    pub fn render_union(&mut self, data: &mut UnionData<'_, '_>) {
        let type_ref = data.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_TYPE) {
            return;
        }

        let type_ident = type_ref.type_ident.clone();
        let derive = self.get_derive(data, Option::<String>::None);
        let extra_traits = Self::get_extra_traits(data);
        let variants = data.variants.iter().map(|data| {
            let UnionVariantData {
                variant_ident,
                target_type,
                ..
            } = data;

            quote! {
                #variant_ident ( #target_type ),
            }
        });

        let code = quote! {
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #extra_traits
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_abstract(&mut self, data: &mut AbstractData<'_, '_>) {
        // TODO: generate trait for the abstract type?

        let type_ref = data.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_TYPE) {
            return;
        }

        let type_ident = type_ref.type_ident.clone();
        let derive = self.get_derive(data, Option::<String>::None);
        let extra_traits = Self::get_extra_traits(data);

        let code = quote! {
            #derive
            pub struct #type_ident(Box<dyn core::any::Any>);

            #extra_traits
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_reference(&mut self, data: &mut ReferenceData<'_, '_>) {
        let ReferenceData {
            mode,
            inner,
            occurs,
            type_ident,
            target_ident,
            ..
        } = data;

        let code = match mode {
            TypedefMode::Auto => unreachable!(),
            TypedefMode::Typedef => {
                let target_type = occurs.make_type(target_ident, false);

                quote! { pub type #type_ident = #target_type; }
            }
            TypedefMode::NewType => {
                let target_type = occurs.make_type(target_ident, false);
                let extra_derive =
                    matches!(occurs, Occurs::Optional | Occurs::DynamicList).then_some("Default");
                let derive = self.get_derive(inner, extra_derive);
                let extra_traits = Self::get_extra_traits(inner);

                quote! {
                    #derive
                    pub struct #type_ident(pub #target_type);

                    #extra_traits
                }
            }
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_enumeration(&mut self, data: &mut EnumerationData<'_, '_>) {
        let EnumerationData {
            inner,
            type_ident,
            variants,
            ..
        } = data;

        let type_ref = inner.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_TYPE) {
            return;
        }

        let derive = self.get_derive(inner, Option::<String>::None);
        let extra_traits = Self::get_extra_traits(inner);
        let variants = variants
            .iter()
            .map(|d| {
                let EnumVariantData {
                    var,
                    variant_ident,
                    target_type,
                } = d;

                let serde = if inner.serde_support == SerdeSupport::None {
                    None
                } else if var.type_.is_some() {
                    Some(quote!(#[serde(other)]))
                } else {
                    let name = format!("{}", d.var.ident.name);

                    Some(quote!(#[serde(rename = #name)]))
                };

                let target_type = target_type
                    .as_ref()
                    .map(|target_type| quote!((#target_type)));

                quote! {
                    #serde
                    #variant_ident #target_type,
                }
            })
            .collect::<Vec<_>>();

        let code = quote! {
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #extra_traits
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_complex_type(&mut self, data: &mut ComplexTypeData<'_, '_>) {
        let derive = self.get_derive(data, Option::<String>::None);
        let extra_traits = Self::get_extra_traits(data);
        let type_ident = data.current_type_ref().type_ident.clone();
        let attributes = data
            .attributes
            .iter()
            .map(|attrib| attrib.render_field(&type_ident, data));

        // If the target mode for the content is `Sequence` we will generate a sequence.
        if matches!(data.target_mode, TypeMode::All | TypeMode::Sequence) {
            let elements = data
                .elements
                .iter()
                .map(|element| element.render_sequence_field(data));

            let code = quote! {
                #derive
                pub struct #type_ident {
                    #( #attributes )*
                    #( #elements )*
                }

                #extra_traits
            };

            return data.add_code(code);
        }

        // Otherwise the content should be rendered as enum. If the attributes are empty
        // we will render the enum directly.
        let elements = data
            .elements
            .iter()
            .map(|element| element.render_choice_variant(data.occurs.is_direct(), data));
        if data.attributes.is_empty() && data.check_generate_flags(GenerateFlags::FLATTEN_CONTENT) {
            let code = quote! {
                #derive
                pub enum #type_ident {
                    #( #elements )*
                }

                #extra_traits
            };

            return data.add_code(code);
        }

        // If we do not have a content at all, we render a struct with the attributes only.
        let content_ident = &data.content_ident;
        let Some(content_type) = data.occurs.make_type(&quote!(#content_ident), false) else {
            let code = quote! {
                #derive
                pub struct #type_ident {
                    #( #attributes )*
                }

                #extra_traits
            };

            return data.add_code(code);
        };

        // If we have attributes and content, we render a struct containing the
        // attributes and a enum containing the content.
        let serde = data
            .serde_support
            .is_none()
            .not()
            .then(|| quote!(#[serde(rename = "$value")]));
        let code = quote! {
            #derive
            pub struct #type_ident {
                #( #attributes )*

                #serde
                pub content: #content_type,
            }

            #derive
            pub enum #content_ident {
                #( #elements )*
            }

            #extra_traits
        };

        data.add_code(code);
    }

    fn get_extra_traits(data: &TypeData<'_, '_>) -> Option<TokenStream> {
        if !data.check_generate_flags(GenerateFlags::WITH_NAMESPACE_TRAIT) {
            return None;
        }

        let ns = data.ident.ns.as_ref()?;
        let module = data.types.modules.get(ns)?;

        let xsd_parser = &data.xsd_parser_crate;
        let type_ident = &data.current_type_ref().type_ident;

        let (prefix, namespace) = match (&module.name, &module.namespace) {
            (Some(prefix), Some(namespace)) => {
                let prefix = prefix.to_string();
                let namespace = namespace.to_string();

                (quote!(Some(#prefix)), quote!(Some(#namespace)))
            }
            (None, Some(namespace)) => {
                let namespace = namespace.to_string();

                (quote!(None), quote!(Some(#namespace)))
            }
            (_, None) => (quote!(None), quote!(None)),
        };

        let code = quote! {
            impl #xsd_parser::WithNamespace for #type_ident {
                fn prefix() -> Option<&'static str> {
                    #prefix
                }

                fn namespace() -> Option<&'static str> {
                    #namespace
                }
            }
        };

        Some(code)
    }

    fn get_derive<I>(&self, generator: &Generator<'_>, extra: I) -> TokenStream
    where
        I: IntoIterator,
        I::Item: Display,
    {
        let _self = self;

        let serde: SmallVec<[Ident2; 2]> = if generator.serde_support == SerdeSupport::None {
            smallvec![]
        } else {
            smallvec![format_ident!("Serialize"), format_ident!("Deserialize")]
        };

        let extra = extra.into_iter().map(|x| format_ident!("{x}"));
        let types = generator
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
}

impl AttributeData<'_> {
    fn render_field(&self, type_ident: &Ident2, generator: &Generator<'_>) -> TokenStream {
        let field_ident = &self.field_ident;
        let target_type = &self.target_type;
        let default = if self.default_value.is_some() {
            let default_path = format!("{type_ident}::default_{field_ident}");

            quote!(default = #default_path,)
        } else if self.attrib.use_ == Use::Optional {
            quote!(default,)
        } else {
            quote!()
        };

        let serde = match generator.serde_support {
            SerdeSupport::None => None,
            SerdeSupport::QuickXml => {
                let name = format!("@{}", self.attrib.ident.name);

                Some(quote!(#[serde(#default rename = #name)]))
            }
            SerdeSupport::SerdeXmlRs => {
                let name = format!("{}", self.attrib.ident.name);

                Some(quote!(#[serde(#default rename = #name)]))
            }
        };

        if self.is_option {
            quote! {
                #serde
                pub #field_ident: Option<#target_type>,
            }
        } else {
            quote! {
                #serde
                pub #field_ident: #target_type,
            }
        }
    }
}

impl ElementData<'_> {
    fn render_sequence_field(&self, generator: &Generator<'_>) -> TokenStream {
        let field_ident = &self.field_ident;
        let need_indirection =
            self.need_box || generator.box_flags.intersects(BoxFlags::STRUCT_ELEMENTS);
        let target_type = self
            .occurs
            .make_type(&self.target_type, need_indirection)
            .unwrap();

        let serde = match generator.serde_support {
            SerdeSupport::None => None,
            SerdeSupport::QuickXml | SerdeSupport::SerdeXmlRs => {
                let name = format!("{}", self.element.ident.name);
                let default = match self.occurs {
                    Occurs::None | Occurs::Single | Occurs::StaticList(_) => quote!(),
                    Occurs::Optional | Occurs::DynamicList => quote!(default,),
                };

                Some(quote!(#[serde(#default rename = #name)]))
            }
        };

        quote! {
            #serde
            pub #field_ident: #target_type,
        }
    }

    fn render_choice_variant(&self, is_direct: bool, data: &TypeData<'_, '_>) -> TokenStream {
        let force_box = data.box_flags.contains(BoxFlags::ENUM_ELEMENTS);
        let need_box = is_direct
            && (force_box
                || data
                    .current_type_ref()
                    .boxed_elements
                    .contains(&self.element.ident));
        let variant_ident = &self.variant_ident;
        let mut target_type = self.target_type.clone();

        if need_box {
            target_type = quote!(Box<#target_type>);
        }

        let serde = match data.serde_support {
            SerdeSupport::None => None,
            SerdeSupport::QuickXml | SerdeSupport::SerdeXmlRs => {
                let name = format!("{}", self.element.ident.name);

                Some(quote!(#[serde(rename = #name)]))
            }
        };

        quote! {
            #serde
            #variant_ident(#target_type),
        }
    }
}
