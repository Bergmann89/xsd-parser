use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote};

use crate::{
    config::TypedefMode,
    generator::{
        data::{
            ComplexType, ComplexTypeBase, ComplexTypeContent, ComplexTypeElement, ComplexTypeEnum,
            ComplexTypeStruct, DynamicType, EnumerationType, EnumerationTypeVariant, ReferenceType,
            UnionType, UnionTypeVariant,
        },
        misc::Occurs,
        Code, Config,
    },
    schema::Namespace,
};

/* UnionType */

impl UnionType<'_> {
    pub(crate) fn render_serializer(&self, code: &mut Code<'_, '_>) {
        let Self {
            type_ident,
            variants,
            ..
        } = self;

        let xsd_parser = &code.xsd_parser_crate;
        let variants = variants
            .iter()
            .map(UnionTypeVariant::render_serializer_variant)
            .collect::<Vec<_>>();

        code.push(quote! {
            impl #xsd_parser::quick_xml::SerializeBytes for #type_ident {
                fn serialize_bytes(&self) -> Result<Option<std::borrow::Cow<'_, str>>, #xsd_parser::quick_xml::Error> {
                    match self {
                        #( #variants )*
                    }
                }
            }
        });
    }
}

impl UnionTypeVariant<'_> {
    fn render_serializer_variant(&self) -> TokenStream {
        let Self { variant_ident, .. } = self;

        quote! {
            Self::#variant_ident(x) => x.serialize_bytes(),
        }
    }
}

/* DynamicType */

impl DynamicType<'_> {
    pub(crate) fn render_serializer(&self, code: &mut Code<'_, '_>) {
        let Self { type_ident, .. } = self;

        let xsd_parser = &code.xsd_parser_crate;

        code.push(quote! {
            impl #xsd_parser::quick_xml::WithSerializer for #type_ident {
                type Serializer<'x> = #xsd_parser::quick_xml::BoxedSerializer<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: Option<&'ser str>,
                    is_root: bool
                ) -> Result<Self::Serializer<'ser>, #xsd_parser::quick_xml::Error> {
                    let _name = name;

                    self.0.serializer(None, is_root)
                }
            }
        });
    }
}

/* ReferenceType */

impl ReferenceType<'_> {
    pub(crate) fn render_serializer(&self, code: &mut Code<'_, '_>) {
        let Self {
            mode,
            occurs,
            type_ident,
            ..
        } = self;

        if matches!(mode, TypedefMode::Auto | TypedefMode::Typedef) {
            return;
        }

        let xsd_parser = code.xsd_parser_crate.clone();
        let body = match occurs {
            Occurs::None => return,
            Occurs::Single => {
                quote! {
                    self.0.serialize_bytes()
                }
            }
            Occurs::Optional => {
                quote! {
                    if let Some(inner) = &self.0 {
                        Ok(Some(inner.serialize_bytes()?))
                    } else {
                        Ok(None)
                    }
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => {
                quote! {
                    if self.0.is_empty() {
                        return Ok(None);
                    }

                    let mut data = String::new();
                    for item in &self.0 {
                        if let Some(bytes) = item.serialize_bytes()? {
                            if !data.is_empty() {
                                data.push(' ');
                            }

                            data.push_str(&bytes);
                        }
                    }

                    Ok(Some(std::borrow::Cow::Owned(data)))
                }
            }
        };

        code.push(quote! {
            impl #xsd_parser::quick_xml::SerializeBytes for #type_ident {
                fn serialize_bytes(&self) -> Result<Option<std::borrow::Cow<'_, str>>, #xsd_parser::quick_xml::Error> {
                    #body
                }
            }
        });
    }
}

/* EnumerationType */

impl EnumerationType<'_> {
    pub(crate) fn render_serializer(&self, code: &mut Code<'_, '_>) {
        let Self {
            type_ident,
            variants,
            ..
        } = self;

        let xsd_parser = &code.xsd_parser_crate;
        let variants = variants
            .iter()
            .map(EnumerationTypeVariant::render_serializer_variant);

        code.push(quote! {
            impl #xsd_parser::quick_xml::SerializeBytes for #type_ident {
                fn serialize_bytes(&self) -> Result<Option<std::borrow::Cow<'_, str>>, #xsd_parser::quick_xml::Error> {
                    match self {
                        #( #variants )*
                    }
                }
            }
        });
    }
}

impl EnumerationTypeVariant<'_> {
    fn render_serializer_variant(&self) -> TokenStream {
        let Self {
            info,
            target_type,
            variant_ident,
        } = self;

        if target_type.is_some() {
            quote! {
                Self::#variant_ident(x) => x.serialize_bytes(),
            }
        } else {
            let name = info.ident.name.to_string();
            let name = Literal::string(&name);

            quote! {
                Self::#variant_ident => Ok(Some(std::borrow::Cow::Borrowed(#name))),
            }
        }
    }
}

/* ComplexType */

impl ComplexType<'_> {
    pub(crate) fn render_serializer(&self, code: &mut Code<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_serializer(code);

                if let Some(content_type) = content_type {
                    content_type.render_serializer(code);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_serializer(code);

                if let Some(content_type) = content_type {
                    content_type.render_serializer(code);
                }
            }
        }
    }
}

impl ComplexTypeBase {
    fn render_with_serializer(&self, code: &mut Code<'_, '_>) {
        let Self {
            type_ident,
            serializer_ident,
            ..
        } = self;
        let xsd_parser = &code.xsd_parser_crate;

        let body = if let Some(tag_name) = &self.tag_name {
            self.render_with_serializer_for_element(tag_name)
        } else {
            self.render_with_serializer_for_content()
        };

        code.push(quote! {
            impl #xsd_parser::quick_xml::WithSerializer for #type_ident {
                type Serializer<'x> = quick_xml_serialize::#serializer_ident<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: Option<&'ser str>,
                    is_root: bool
                ) -> Result<Self::Serializer<'ser>, #xsd_parser::quick_xml::Error> {
                    #body
                }
            }
        });
    }

    fn render_with_serializer_for_element(&self, tag_name: &str) -> TokenStream {
        let Self {
            serializer_ident,
            serializer_state_ident,
            ..
        } = self;

        quote! {
            Ok(quick_xml_serialize::#serializer_ident {
                value: self,
                state: quick_xml_serialize::#serializer_state_ident::Init__,
                name: name.unwrap_or(#tag_name),
                is_root,
            })
        }
    }

    fn render_with_serializer_for_content(&self) -> TokenStream {
        let Self {
            serializer_ident,
            serializer_state_ident,
            ..
        } = self;

        quote! {
            let _name = name;
            let _is_root = is_root;

            Ok(quick_xml_serialize::#serializer_ident {
                value: self,
                state: quick_xml_serialize::#serializer_state_ident::Init__,
            })
        }
    }

    fn render_serializer_type(&self, code: &mut Code<'_, '_>) {
        let Self {
            type_ident,
            serializer_ident,
            serializer_state_ident,
            ..
        } = self;
        let extra = self.represents_element().then(|| {
            quote! {
                pub(super) name: &'ser str,
                pub(super) is_root: bool,
            }
        });

        code.push_quick_xml_serialize(quote! {
            #[derive(Debug)]
            pub struct #serializer_ident<'ser> {
                pub(super) value: &'ser super::#type_ident,
                pub(super) state: #serializer_state_ident<'ser>,
                #extra
            }
        });
    }
}

impl ComplexTypeEnum<'_> {
    fn render_serializer(&self, code: &mut Code<'_, '_>) {
        self.render_with_serializer(code);
        self.render_serializer_type(code);
        self.render_serializer_state_type(code);
        self.render_serializer_impl(code);
    }

    fn render_serializer_state_type(&self, code: &mut Code<'_, '_>) {
        let state_ident = &self.serializer_state_ident;

        let state_variants = self
            .elements
            .iter()
            .map(|x| x.render_serializer_state_variant(code));

        code.push_quick_xml_serialize(quote! {
            #[derive(Debug)]
            pub(super) enum #state_ident<'ser> {
                Init__,
                #( #state_variants )*
                Done__,
                Phantom__(&'ser ()),
            }
        });
    }

    fn render_serializer_impl(&self, code: &mut Code<'_, '_>) {
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;
        let xsd_parser = &code.xsd_parser_crate;

        let variants_init = self.elements.iter().map(|element| {
            let type_ident = &self.type_ident;
            let variant_ident = &element.variant_ident;
            let init = element.render_serializer_state_init(
                code,
                &self.serializer_state_ident,
                &quote!(x),
            );

            quote! {
                #type_ident::#variant_ident(x) => #init,
            }
        });

        let variants_state = self.elements.iter().map(|element| {
            let variant_ident = &element.variant_ident;

            quote! {
                #serializer_state_ident::#variant_ident(x) => {
                    match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = #serializer_state_ident::Done__,
                    }
                }
            }
        });

        let state_init = quote! {
            match self.value {
                #( #variants_init )*
            }
        };

        code.push_quick_xml_serialize(quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self) -> Result<Option<#xsd_parser::quick_xml::Event<'ser>>, #xsd_parser::quick_xml::Error>
                {
                    loop {
                        match &mut self.state {
                            #serializer_state_ident::Init__ => #state_init,
                            #( #variants_state )*
                            #serializer_state_ident::Done__ => return Ok(None),
                            #serializer_state_ident::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }

            impl<'ser> core::iter::Iterator for #serializer_ident<'ser> {
                type Item = Result<#xsd_parser::quick_xml::Event<'ser>, #xsd_parser::quick_xml::Error>;

                fn next(&mut self) -> Option<Self::Item> {
                    match self.next_event() {
                        Ok(Some(event)) => Some(Ok(event)),
                        Ok(None) => None,
                        Err(error) => {
                            self.state = #serializer_state_ident::Done__;

                            Some(Err(error))
                        }
                    }
                }
            }
        });
    }
}

impl ComplexTypeStruct<'_> {
    fn serializer_need_end_state(&self) -> bool {
        self.represents_element() && self.has_content()
    }

    fn render_serializer(&self, code: &mut Code<'_, '_>) {
        self.render_with_serializer(code);
        self.render_serializer_type(code);
        self.render_serializer_state_type(code);
        self.render_serializer_impl(code);
    }

    fn render_serializer_state_type(&self, code: &mut Code<'_, '_>) {
        let state_ident = &self.serializer_state_ident;

        let state_variants = self
            .elements
            .iter()
            .map(|x| x.render_serializer_state_variant(code));
        let state_content = self
            .content
            .as_ref()
            .map(|x| x.render_serializer_state_variant(code));
        let state_end = self.serializer_need_end_state().then(|| {
            quote! {
                End__,
            }
        });

        code.push_quick_xml_serialize(quote! {
            #[derive(Debug)]
            pub(super) enum #state_ident<'ser> {
                Init__,
                #( #state_variants )*
                #state_content
                #state_end
                Done__,
                Phantom__(&'ser ()),
            }
        });
    }

    #[allow(clippy::too_many_lines)]
    fn render_serializer_impl(&self, code: &mut Code<'_, '_>) {
        let xsd_parser = &code.xsd_parser_crate;
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        let emit_start_event = self
            .represents_element()
            .then(|| self.render_serializer_impl_start_event(code));

        let final_state = if self.serializer_need_end_state() {
            quote!(#serializer_state_ident::End__)
        } else {
            quote!(#serializer_state_ident::Done__)
        };

        let handle_state_init = if let Some(first) = self.elements.first() {
            let field_ident = &first.field_ident;
            let init = first.render_serializer_state_init(
                code,
                serializer_state_ident,
                &quote!(&self.value.#field_ident),
            );

            quote!(#init;)
        } else if let Some(content) = &self.content {
            let init = content.render_serializer_state_init(code, serializer_state_ident);

            quote!(#init;)
        } else {
            quote!(self.state = #final_state;)
        };

        let handle_state_variants = (0..).take(self.elements.len()).map(|i| {
            let element = &self.elements[i];
            let variant_ident = &element.variant_ident;

            let next = if let Some(next) = self.elements.get(i + 1) {
                let field_ident = &next.field_ident;
                let init = next.render_serializer_state_init(
                    code,
                    serializer_state_ident,
                    &quote!(&self.value.#field_ident),
                );

                quote!(#init,)
            } else if let Some(content) = &self.content {
                let init = content.render_serializer_state_init(code, serializer_state_ident);

                quote! {
                    self.state = #init,
                }
            } else {
                quote! {
                    self.state = #final_state,
                }
            };

            quote! {
                #serializer_state_ident::#variant_ident(x) => match x.next().transpose()? {
                    Some(event) => return Ok(Some(event)),
                    None => #next
                }
            }
        });

        let handle_state_content = self.content.as_ref().map(|_| {
            quote! {
                #serializer_state_ident::Content__(x) => match x.next().transpose()? {
                    Some(event) => return Ok(Some(event)),
                    None => self.state = #final_state,
                }
            }
        });

        let handle_state_end = self.serializer_need_end_state().then(|| {
            quote! {
                #serializer_state_ident::End__ => {
                    self.state = #serializer_state_ident::Done__;

                    return Ok(Some(
                        #xsd_parser::quick_xml::Event::End(
                            #xsd_parser::quick_xml::BytesEnd::new(self.name))
                        )
                    );
                }
            }
        });

        code.push_quick_xml_serialize(quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self) -> Result<Option<#xsd_parser::quick_xml::Event<'ser>>, #xsd_parser::quick_xml::Error>
                {
                    loop {
                        match &mut self.state {
                            #serializer_state_ident::Init__ => {
                                #handle_state_init
                                #emit_start_event
                            }
                            #( #handle_state_variants )*
                            #handle_state_content
                            #handle_state_end
                            #serializer_state_ident::Done__ => return Ok(None),
                            #serializer_state_ident::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }

            impl<'ser> core::iter::Iterator for #serializer_ident<'ser> {
                type Item = Result<#xsd_parser::quick_xml::Event<'ser>, #xsd_parser::quick_xml::Error>;

                fn next(&mut self) -> Option<Self::Item> {
                    match self.next_event() {
                        Ok(Some(event)) => Some(Ok(event)),
                        Ok(None) => None,
                        Err(error) => {
                            self.state = #serializer_state_ident::Done__;

                            Some(Err(error))
                        }
                    }
                }
            }
        });
    }

    fn render_serializer_impl_start_event(&self, config: &Config<'_>) -> TokenStream {
        let xsd_parser = &config.xsd_parser_crate;

        let xmlns = config
            .types
            .modules
            .values()
            .filter_map(|module| {
                let ns = module.namespace.as_ref()?;
                if *ns == Namespace::XS || *ns == Namespace::XML {
                    return None;
                }

                let name = module.name.as_ref()?;

                let ns = ns.to_string();
                let xmlns = format!("xmlns:{name}");

                Some(quote! {
                    bytes.push_attribute((#xmlns, #ns));
                })
            })
            .collect::<Vec<_>>();

        let attributes = self.attributes.iter().map(|attrib| {
            let attrib_name = &attrib.tag_name;
            let field_ident = &attrib.ident;

            if attrib.is_option {
                quote! {
                    #xsd_parser::quick_xml::write_attrib_opt(&mut bytes, #attrib_name, &self.value.#field_ident)?;
                }
            } else {
                quote! {
                    #xsd_parser::quick_xml::write_attrib(&mut bytes, #attrib_name, &self.value.#field_ident)?;
                }
            }
        });

        let bytes_mut = self.has_attributes().then(|| quote!(mut));
        let bytes_ctor = if xmlns.is_empty() {
            quote! {
                let #bytes_mut bytes = #xsd_parser::quick_xml::BytesStart::new(self.name);
            }
        } else {
            quote! {
                let mut bytes = #xsd_parser::quick_xml::BytesStart::new(self.name);
                if self.is_root {
                    #( #xmlns )*
                }
            }
        };

        let event = if self.has_content() {
            format_ident!("Start")
        } else {
            format_ident!("Empty")
        };

        quote! {
            #bytes_ctor
            #( #attributes )*
            return Ok(Some(#xsd_parser::quick_xml::Event::#event(bytes)))
        }
    }
}

impl ComplexTypeContent {
    fn render_serializer_state_variant(&self, config: &Config<'_>) -> Option<TokenStream> {
        let serializer = self
            .occurs
            .make_serializer_type(&config.xsd_parser_crate, &self.target_type)?;

        Some(quote! {
            Content__(#serializer),
        })
    }

    fn render_serializer_state_init(
        &self,
        config: &Config<'_>,
        state_ident: &Ident2,
    ) -> TokenStream {
        let xsd_parser = &config.xsd_parser_crate;

        match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => quote! {
                self.state = #state_ident::Content__(
                    #xsd_parser::quick_xml::WithSerializer::serializer(&self.value.content, None, false)?
                )
            },
            Occurs::Optional | Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                self.state = #state_ident::Content__(
                    #xsd_parser::quick_xml::IterSerializer::new(
                        &self.value.content,
                        None,
                        false
                    )
                )
            },
        }
    }
}

impl ComplexTypeElement<'_> {
    fn render_serializer_state_variant(&self, config: &Config<'_>) -> TokenStream {
        let target_type = &self.target_type;
        let variant_ident = &self.variant_ident;
        let xsd_parser = &config.xsd_parser_crate;

        let serializer = self.occurs.make_serializer_type(xsd_parser, target_type);

        quote! {
            #variant_ident(#serializer),
        }
    }

    fn render_serializer_state_init(
        &self,
        config: &Config<'_>,
        state_ident: &Ident2,
        value: &TokenStream,
    ) -> TokenStream {
        let xsd_parser = &config.xsd_parser_crate;

        let field_name = &self.tag_name;
        let variant_ident = &self.variant_ident;

        match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => quote! {
                self.state = #state_ident::#variant_ident(
                    #xsd_parser::quick_xml::WithSerializer::serializer(#value, Some(#field_name), false)?
                )
            },
            Occurs::Optional | Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                self.state = #state_ident::#variant_ident(
                    #xsd_parser::quick_xml::IterSerializer::new(
                        #value,
                        Some(#field_name),
                        false
                    )
                )
            },
        }
    }
}

impl Occurs {
    fn make_serializer_type(
        &self,
        xsd_parser: &Ident2,
        target_type: &TokenStream,
    ) -> Option<TokenStream> {
        match self {
            Occurs::None => None,
            Occurs::Single => Some(
                quote!(<#target_type as #xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
            ),
            Occurs::Optional => Some(
                quote!(#xsd_parser::quick_xml::IterSerializer<'ser, Option<#target_type>, #target_type>),
            ),
            Occurs::DynamicList => Some(
                quote!(#xsd_parser::quick_xml::IterSerializer<'ser, Vec<#target_type>, #target_type>),
            ),
            Occurs::StaticList(sz) => Some(
                quote!(#xsd_parser::quick_xml::IterSerializer<'ser, [#target_type; #sz], #target_type>),
            ),
        }
    }
}
