use std::fmt::Display;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote, ToTokens};
use smallvec::{smallvec, SmallVec};

use crate::{
    config::{GeneratorFlags, SerdeSupport, TypedefMode},
    generator::{
        data::{
            ComplexType, ComplexTypeAttribute, ComplexTypeContent, ComplexTypeElement,
            ComplexTypeEnum, ComplexTypeStruct, DynamicType, EnumerationType,
            EnumerationTypeVariant, ReferenceType, UnionType, UnionTypeVariant,
        },
        misc::{IdentPath, Occurs},
        Code, Config, DynTypeTraits,
    },
    schema::xs::Use,
};

/* UnionType */

impl UnionType<'_> {
    pub(crate) fn render_types(&self, code: &mut Code<'_, '_>) {
        let Self {
            type_ident,
            trait_impls,
            variants,
            ..
        } = self;
        let derive = get_derive(code, Option::<String>::None);
        let trait_impls = render_trait_impls(code, type_ident, trait_impls);
        let variants = variants.iter().map(|x| x.render_variant(code));

        code.push(quote! {
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        });
    }
}

/* UnionTypeVariant */

impl UnionTypeVariant<'_> {
    fn render_variant(&self, code: &Code<'_, '_>) -> TokenStream {
        let Self {
            variant_ident,
            target_type,
            ..
        } = self;

        let target_type = code.resolve_type_for_module(target_type);

        quote! {
            #variant_ident ( #target_type ),
        }
    }
}

/* DynamicType */

impl DynamicType<'_> {
    pub(crate) fn render_types(&self, code: &mut Code<'_, '_>) {
        let Self {
            type_ident,
            trait_ident,
            sub_traits,
            ..
        } = self;

        let xsd_crate = &code.xsd_parser_crate;

        let derive = get_derive(code, Option::<String>::None);
        let trait_impls = render_trait_impls(code, type_ident, &[]);
        let dyn_traits = sub_traits.as_ref().map_or_else(
            || get_dyn_type_traits(code, [quote!(#xsd_crate::AsAny)]),
            |traits| format_traits(traits.iter().map(|x| code.resolve_type_for_module(x))),
        );

        code.push(quote! {
            #derive
            pub struct #type_ident(pub Box<dyn #trait_ident>);

            pub trait #trait_ident: #dyn_traits { }

            #( #trait_impls )*
        });
    }
}

/* ReferenceType */

impl ReferenceType<'_> {
    pub(crate) fn render_types(&self, code: &mut Code<'_, '_>) {
        let Self {
            mode,
            occurs,
            type_ident,
            target_type,
            trait_impls,
            ..
        } = self;

        let target_type = code.resolve_type_for_module(target_type);

        code.push(match mode {
            TypedefMode::Auto => crate::unreachable!(),
            TypedefMode::Typedef => {
                let target_type = occurs.make_type(&target_type, false);

                quote! { pub type #type_ident = #target_type; }
            }
            TypedefMode::NewType => {
                let target_type = occurs.make_type(&target_type, false);
                let extra_derive =
                    matches!(occurs, Occurs::Optional | Occurs::DynamicList).then_some("Default");
                let derive = get_derive(code, extra_derive);
                let trait_impls = render_trait_impls(code, type_ident, trait_impls);

                quote! {
                    #derive
                    pub struct #type_ident(pub #target_type);

                    #( #trait_impls )*
                }
            }
        });
    }
}

/* EnumerationType */

impl EnumerationType<'_> {
    pub(crate) fn render_types(&self, code: &mut Code<'_, '_>) {
        let Self {
            type_ident,
            variants,
            trait_impls,
            ..
        } = self;

        let derive = get_derive(code, Option::<String>::None);
        let trait_impls = render_trait_impls(code, type_ident, trait_impls);

        let variants = variants
            .iter()
            .map(|d| d.render_variant(code))
            .collect::<Vec<_>>();

        code.push(quote! {
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        });
    }
}

impl EnumerationTypeVariant<'_> {
    fn render_variant(&self, code: &Code<'_, '_>) -> TokenStream {
        let Self {
            info,
            variant_ident,
            target_type,
        } = self;

        let serde = if code.serde_support == SerdeSupport::None {
            None
        } else if info.type_.is_some() {
            Some(quote!(#[serde(other)]))
        } else {
            let name = format!("{}", info.ident.name);

            Some(quote!(#[serde(rename = #name)]))
        };

        let target_type = target_type.as_ref().map(|target_type| {
            let target_type = code.resolve_type_for_module(target_type);

            quote!((#target_type))
        });

        quote! {
            #serde
            #variant_ident #target_type,
        }
    }
}

/* ComplexType */

impl ComplexType<'_> {
    pub(crate) fn render_types(&self, code: &mut Code<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_type(code);

                if let Some(content_type) = content_type {
                    content_type.render_types(code);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_type(code);

                if let Some(content_type) = content_type {
                    content_type.render_types(code);
                }
            }
        };
    }
}

impl ComplexTypeEnum<'_> {
    fn render_type(&self, code: &mut Code<'_, '_>) {
        let derive = get_derive(code, Option::<String>::None);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(code, type_ident, &self.trait_impls);

        let variants = self.elements.iter().map(|x| x.render_variant(code));

        code.push(quote! {
            #derive
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        });
    }
}

impl ComplexTypeStruct<'_> {
    fn render_type(&self, code: &mut Code<'_, '_>) {
        let derive = get_derive(code, Option::<String>::None);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(code, type_ident, &self.trait_impls);

        let attributes = self
            .attributes
            .iter()
            .map(|x| x.render_field(code, type_ident));
        let fields = self.elements().iter().map(|x| x.render_field(code));
        let content = self.content().as_ref().and_then(|x| x.render_field(code));

        code.push(quote! {
            #derive
            pub struct #type_ident {
                #( #attributes )*
                #( #fields )*
                #content
            }

            #( #trait_impls )*
        });
    }
}

impl ComplexTypeContent {
    fn render_field(&self, code: &Code<'_, '_>) -> Option<TokenStream> {
        let target_type = code.resolve_type_for_module(&self.target_type);
        let target_type = self.occurs.make_type(&target_type, false)?;
        let serde_default = (self.min_occurs == 0).then(|| quote!(default,));
        let serde = match (code.serde_support, self.is_simple) {
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

impl ComplexTypeAttribute<'_> {
    fn render_field(&self, code: &Code<'_, '_>, type_ident: &Ident2) -> TokenStream {
        let field_ident = &self.ident;
        let target_type = code.resolve_type_for_module(&self.target_type);
        let default = if self.default_value.is_some() {
            let default_path = format!("{type_ident}::default_{field_ident}");

            quote!(default = #default_path,)
        } else if self.info.use_ == Use::Optional {
            quote!(default,)
        } else {
            quote!()
        };

        let serde = match code.serde_support {
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

impl ComplexTypeElement<'_> {
    fn render_field(&self, code: &Code<'_, '_>) -> TokenStream {
        let field_ident = &self.field_ident;
        let target_type = code.resolve_type_for_module(&self.target_type);
        let target_type = self
            .occurs
            .make_type(&target_type, self.need_indirection)
            .unwrap();

        let serde = match code.serde_support {
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

        quote! {
            #serde
            pub #field_ident: #target_type,
        }
    }

    fn render_variant(&self, code: &Code<'_, '_>) -> TokenStream {
        let variant_ident = &self.variant_ident;
        let target_type = code.resolve_type_for_module(&self.target_type);
        let target_type = self.occurs.make_type(&target_type, self.need_indirection);

        let serde = match code.serde_support {
            SerdeSupport::None => None,
            SerdeSupport::QuickXml | SerdeSupport::SerdeXmlRs => {
                let name = self.info.ident.name.to_string();

                Some(quote!(#[serde(rename = #name)]))
            }
        };

        quote! {
            #serde
            #variant_ident(#target_type),
        }
    }
}

/* Helper */

fn get_derive<I>(config: &Config<'_>, extra: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: Display,
{
    let serde: SmallVec<[Ident2; 2]> = if config.serde_support == SerdeSupport::None {
        smallvec![]
    } else {
        smallvec![format_ident!("Serialize"), format_ident!("Deserialize")]
    };

    let extra = extra.into_iter().map(|x| format_ident!("{x}"));
    let types = config
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

fn get_dyn_type_traits<I>(config: &Config<'_>, extra: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: Into<TokenStream>,
{
    let xsd_parser = &config.xsd_parser_crate;

    let serde: SmallVec<[TokenStream; 2]> = if config.serde_support == SerdeSupport::None {
        smallvec![]
    } else {
        smallvec![quote!(serde::Serialize), quote!(serde::DeserializeOwned)]
    };

    format_traits(match &config.dyn_type_traits {
        DynTypeTraits::Custom(x) => x.clone(),
        DynTypeTraits::Auto => config
            .derive
            .iter()
            .map(|x| match x.to_string().as_str() {
                "Debug" => quote!(core::fmt::Debug),
                "Hash" => quote!(core::has::Hash),
                _ => quote!(#x),
            })
            .chain(serde)
            .chain(
                config
                    .check_flags(GeneratorFlags::QUICK_XML_SERIALIZE)
                    .then(|| quote!(#xsd_parser::quick_xml::WithBoxedSerializer)),
            )
            .chain(extra.into_iter().map(Into::into))
            .collect::<Vec<_>>(),
    })
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
    code: &Code<'_, '_>,
    type_ident: &'a Ident2,
    trait_data: &'a [IdentPath],
) -> impl Iterator<Item = TokenStream> + 'a {
    let trait_with_namespace = code
        .check_flags(GeneratorFlags::WITH_NAMESPACE_TRAIT)
        .then(|| render_trait_with_namespace(code, type_ident))
        .flatten();

    trait_data
        .iter()
        .map(move |trait_ident| {
            let trait_ident = trait_ident.ident();

            quote! {
                impl #trait_ident for #type_ident { }
            }
        })
        .chain(trait_with_namespace)
}

fn render_trait_with_namespace(code: &Code<'_, '_>, type_ident: &Ident2) -> Option<TokenStream> {
    let ns = code.current_ns()?;
    let module = code.types.modules.get(&ns)?;
    let xsd_parser = &code.xsd_parser_crate;

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
