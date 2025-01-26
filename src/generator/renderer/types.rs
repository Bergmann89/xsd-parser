use std::fmt::Display;
use std::ops::Not;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote, ToTokens};
use smallvec::{smallvec, SmallVec};
use tracing::instrument;

use crate::schema::xs::Use;

use super::super::data::{
    AttributeData, ComplexTypeData, DynamicData, ElementData, EnumVariantData, EnumerationData,
    ReferenceData, TraitData, TypeData, UnionData, UnionVariantData,
};
use super::super::misc::{
    BoxFlags, DynTypeTraits, GenerateFlags, Occurs, StateFlags, TypeMode, TypedefMode,
};
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
        let derive = Self::get_derive(data, Option::<String>::None);
        let trait_impls = Self::render_trait_impls(&data.inner, &data.trait_impls);
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

            #( #trait_impls )*
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_dynamic(&mut self, data: &mut DynamicData<'_, '_>) {
        let type_ref = data.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_TYPE) {
            return;
        }

        let type_ident = type_ref.type_ident.clone();
        let xsd_crate = &data.xsd_parser_crate;
        let trait_ident = &data.trait_ident;
        let derive = Self::get_derive(data, Option::<String>::None);
        let trait_impls = Self::render_trait_impls(&data.inner, &[]);

        let dyn_traits = if let Some(traits) = &data.sub_traits {
            Self::format_traits(traits)
        } else {
            Self::get_dyn_type_traits(data, [quote!(#xsd_crate::AsAny)])
        };

        let code = quote! {
            #derive
            pub struct #type_ident(pub Box<dyn #trait_ident>);

            pub trait #trait_ident: #dyn_traits { }

            #( #trait_impls )*
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
                let derive = Self::get_derive(inner, extra_derive);
                let trait_impls = Self::render_trait_impls(&data.inner, &data.trait_impls);

                quote! {
                    #derive
                    pub struct #type_ident(pub #target_type);

                    #( #trait_impls )*
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

        let derive = Self::get_derive(inner, Option::<String>::None);
        let trait_impls = Self::render_trait_impls(inner, &data.trait_impls);

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

            #( #trait_impls )*
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_complex_type(&mut self, data: &mut ComplexTypeData<'_, '_>) {
        let derive = Self::get_derive(data, Option::<String>::None);
        let type_ident = data.current_type_ref().type_ident.clone();
        let trait_impls = Self::render_trait_impls(&data.inner, &data.trait_impls);
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

                #( #trait_impls )*
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

                #( #trait_impls )*
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

                #( #trait_impls )*
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

            #( #trait_impls )*
        };

        data.add_code(code);
    }

    fn get_derive<I>(generator: &Generator<'_>, extra: I) -> TokenStream
    where
        I: IntoIterator,
        I::Item: Display,
    {
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

    fn get_dyn_type_traits<I>(generator: &Generator<'_>, extra: I) -> TokenStream
    where
        I: IntoIterator,
        I::Item: Into<TokenStream>,
    {
        let xsd_parser = &generator.xsd_parser_crate;

        let serde: SmallVec<[TokenStream; 2]> = if generator.serde_support == SerdeSupport::None {
            smallvec![]
        } else {
            smallvec![quote!(serde::Serialize), quote!(serde::DeserializeOwned)]
        };

        let traits = match &generator.dyn_type_traits {
            DynTypeTraits::Custom(x) => x.clone(),
            DynTypeTraits::Auto => generator
                .derive
                .iter()
                .map(|x| match x.to_string().as_str() {
                    "Debug" => quote!(core::fmt::Debug),
                    "Hash" => quote!(core::has::Hash),
                    _ => quote!(#x),
                })
                .chain(serde)
                .chain(
                    generator
                        .check_generate_flags(GenerateFlags::QUICK_XML_SERIALIZE)
                        .then(|| quote!(#xsd_parser::quick_xml::WithBoxedSerializer)),
                )
                .chain(extra.into_iter().map(Into::into))
                .collect::<Vec<_>>(),
        };

        Self::format_traits(traits)
    }

    fn format_traits<I>(iter: I) -> TokenStream
    where
        I: IntoIterator,
        I::Item: ToTokens,
    {
        let parts =
            iter.into_iter()
                .enumerate()
                .map(|(i, x)| if i == 0 { quote!(#x) } else { quote!(+ #x) });

        quote! {
            #( #parts )*
        }
    }

    fn render_trait_impls<'a>(
        type_data: &'a TypeData<'_, '_>,
        trait_data: &'a [TraitData],
    ) -> impl Iterator<Item = TokenStream> + 'a {
        let trait_as_any = trait_data
            .is_empty()
            .not()
            .then(|| Self::render_trait_as_any(type_data));
        let trait_with_namespace = type_data
            .check_generate_flags(GenerateFlags::WITH_NAMESPACE_TRAIT)
            .then(|| Self::render_trait_with_namespace(type_data))
            .flatten();

        let type_ident = &type_data.current_type_ref().type_ident;
        trait_data
            .iter()
            .map(move |TraitData { trait_ident, .. }| {
                quote! {
                    impl #trait_ident for #type_ident { }
                }
            })
            .chain(trait_as_any)
            .chain(trait_with_namespace)
    }

    fn render_trait_as_any(data: &TypeData<'_, '_>) -> TokenStream {
        let xsd_parser = &data.xsd_parser_crate;
        let type_ident = &data.current_type_ref().type_ident;

        quote! {
            impl #xsd_parser::AsAny for #type_ident {
                fn as_any(&self) -> &dyn core::any::Any {
                    self
                }
                fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
                    self
                }
            }
        }
    }

    fn render_trait_with_namespace(data: &TypeData<'_, '_>) -> Option<TokenStream> {
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
