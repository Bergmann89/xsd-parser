use std::collections::BTreeSet;
use std::ops::Not;

use inflector::Inflector;
use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote};
use tracing::instrument;

use crate::schema::xs::Use;
use crate::schema::NamespaceId;
use crate::types::{ElementMode, Module, Types};

use super::super::super::data::{
    ComplexTypeData, DynamicData, EnumVariantData, EnumerationData, ReferenceData,
    SimpleContentData, UnionData, UnionVariantData,
};
use super::super::super::misc::{Occurs, StateFlags, TypeMode, TypedefMode};
use super::{AttributeImpl, ComplexTypeImpl, DynamicTypeImpl, ElementImpl, QuickXmlRenderer};

impl QuickXmlRenderer {
    #[instrument(level = "trace", skip(self))]
    pub fn render_union_deserialize(&mut self, data: &mut UnionData<'_, '_>) {
        let UnionData {
            inner, variants, ..
        } = data;

        let xsd_parser = inner.xsd_parser_crate.clone();
        let type_ref = inner.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_DESERIALIZE) {
            return;
        }

        let type_ident = &type_ref.type_ident;

        let variants = variants
            .iter()
            .map(|data| {
                let UnionVariantData {
                    variant_ident,
                    target_type,
                    ..
                } = data;

                quote! {
                    match #target_type::deserialize_bytes(reader, bytes) {
                        Ok(value) => return Ok(Self::#variant_ident(value)),
                        Err(error) => errors.push(Box::new(error)),
                    }
                }
            })
            .collect::<Vec<_>>();

        let code = quote! {
            impl #xsd_parser::quick_xml::DeserializeBytes for #type_ident {
                fn deserialize_bytes<R>(
                    reader: &R,
                    bytes: &[u8],
                ) -> Result<Self, #xsd_parser::quick_xml::Error>
                where
                    R: #xsd_parser::quick_xml::XmlReader
                {
                    use #xsd_parser::quick_xml::{Error, ErrorKind};

                    let mut errors = Vec::new();

                    #( #variants )*

                    Err(Error::from(ErrorKind::InvalidUnion(errors.into())))
                }
            }
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_dynamic_deserialize(&mut self, data: &mut DynamicData<'_, '_>) {
        let type_ref = data.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_DESERIALIZE) {
            return;
        }

        let (code, deserializer_code) = DynamicTypeImpl::new(data).render_deserializer();

        data.add_code(code);
        data.add_quick_xml_deserialize_code(deserializer_code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_reference_deserialize(&mut self, data: &mut ReferenceData<'_, '_>) {
        let ReferenceData {
            mode,
            occurs,
            inner,
            type_ident,
            target_ident,
            ..
        } = data;

        let xsd_parser = inner.xsd_parser_crate.clone();
        let type_ref = inner.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_DESERIALIZE)
            || matches!(mode, TypedefMode::Auto | TypedefMode::Typedef)
        {
            return;
        }

        let body = match occurs {
            Occurs::None => return,
            Occurs::Single => {
                quote! {
                    Ok(Self(#target_ident::deserialize_bytes(reader, bytes)?))
                }
            }
            Occurs::Optional => {
                quote! {
                    Ok(Self(Some(#target_ident::deserialize_bytes(reader, bytes)?)))
                }
            }
            Occurs::DynamicList => {
                quote! {
                    Ok(Self(bytes
                        .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                        .map(|bytes| #target_ident::deserialize_bytes(reader, bytes))
                        .collect::<Result<Vec<_>, _>>()?
                    ))
                }
            }
            Occurs::StaticList(size) => {
                quote! {
                    use #xsd_parser::quick_xml::ErrorKind;

                    let arr: [Option<#target_ident>; #size];
                    let parts = bytes
                        .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                        .map(|bytes| #target_ident::deserialize_bytes(reader, bytes));
                    let mut index = 0;

                    for part in parts {
                        if index >= #size {
                            return Err(reader.map_error(ErrorKind::InsufficientSize {
                                min: #size,
                                max: #size,
                                actual: index,
                            }));
                        }

                        arr[index] = Some(part?);

                        index += 1;
                    }

                    if index < #size {
                        return Err(reader.map_error(ErrorKind::InsufficientSize {
                            min: #size,
                            max: #size,
                            actual: index,
                        }));
                    }

                    Ok(Self(arr.map(|x| x.unwrap())))
                }
            }
        };

        let code = quote! {
            impl #xsd_parser::quick_xml::DeserializeBytes for #type_ident {
                fn deserialize_bytes<R>(
                    reader: &R,
                    bytes: &[u8],
                ) -> Result<Self, #xsd_parser::quick_xml::Error>
                where
                    R: #xsd_parser::quick_xml::XmlReader
                {
                    #body
                }
            }
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_enumeration_deserialize(&mut self, data: &mut EnumerationData<'_, '_>) {
        let EnumerationData {
            inner, variants, ..
        } = data;

        let xsd_parser = inner.xsd_parser_crate.clone();
        let type_ref = inner.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_DESERIALIZE) {
            return;
        }

        let type_ident = &type_ref.type_ident;

        let mut other = None;
        let variants = variants.iter().filter_map(|data| {
            let EnumVariantData { var, target_type, variant_ident } =  data;

            if let Some(target_type) = target_type {
                other = Some(
                    quote! { x => Ok(Self::#variant_ident(#target_type::deserialize_bytes(reader, x)?)), },
                );

                return None
            }

            let name = Literal::byte_string(var.ident.name.to_string().as_bytes());

            Some(quote! {
                #name => Ok(Self::#variant_ident),
            })
        }).collect::<Vec<_>>();

        let other = other.unwrap_or_else(|| {
            quote! {
                x => {
                    use #xsd_parser::quick_xml::{ErrorKind, RawByteStr};

                    Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x))))
                },
            }
        });

        let code = quote! {
            impl #xsd_parser::quick_xml::DeserializeBytes for #type_ident {
                fn deserialize_bytes<R>(
                    reader: &R,
                    bytes: &[u8],
                ) -> Result<Self, #xsd_parser::quick_xml::Error>
                where
                    R: #xsd_parser::quick_xml::XmlReader
                {
                    match bytes {
                        #( #variants )*
                        #other
                    }
                }
            }
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_complex_type_deserialize(&mut self, data: &mut ComplexTypeData<'_, '_>) {
        let type_ref = data.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_DESERIALIZE) {
            return;
        }

        let (code, deserializer_code) = ComplexTypeImpl::new(data).render_deserializer();

        data.add_code(code);
        data.add_quick_xml_deserialize_code(deserializer_code);
    }
}

/* DynamicTypeImpl */

impl DynamicTypeImpl<'_, '_, '_> {
    #[allow(clippy::too_many_lines)]
    fn render_deserializer(&mut self) -> (TokenStream, TokenStream) {
        let type_ident = self.type_ident;
        let xsd_parser = &self.xsd_parser_crate;
        let deserializer_ident = &self.deserializer_ident;

        let variants = self.derived_types.iter().map(|x| {
            let type_ident = &x.target_ident;
            let variant_ident = &x.variant_ident;

            quote! {
                #variant_ident(<super::#type_ident as #xsd_parser::quick_xml::WithDeserializer>::Deserializer),
            }
        });

        let variants_init = self
            .derived_types
            .iter()
            .map(|x| {
                let b_name = &x.b_name;
                let target_ident = &x.target_ident;
                let variant_ident = &x.variant_ident;

                let handler = quote! {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = <super::#target_ident as #xsd_parser::quick_xml::WithDeserializer>::Deserializer::init(reader, event)?;

                    return Ok(DeserializerOutput {
                        data: data.map(|x| super::#type_ident(Box::new(x))),
                        deserializer: deserializer.map(Self::#variant_ident),
                        event,
                        allow_any,
                    })
                };

                if let Some(module) = x.ident.ns.and_then(|ns| self.types.modules.get(&ns)) {
                    let ns_name = make_ns_const(module);

                    quote! {
                        if matches!(reader.resolve_local_name(name, #ns_name), Some(#b_name)) {
                            #handler
                        }
                    }
                } else {
                    quote! {
                        if name.as_ref() == #b_name {
                            #handler
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let namespace_consts = unique_namespaces(
            self.types,
            self.derived_types.iter().filter_map(|x| x.ident.ns),
        );

        let impl_init = variants_init.is_empty().not().then(|| quote! {
            let attrib = b.attributes()
                .find(|attrib| {
                    let Ok(attrib) = attrib else { return false };
                    let (resolve, name) = reader.resolve(attrib.key, true);

                    matches!(resolve, ResolveResult::Unbound | ResolveResult::Bound(Namespace(b"http://www.w3.org/2001/XMLSchema-instance")))
                        && name.as_ref() == b"type"
                })
                .transpose()?;
            let name = attrib.as_ref().map(|attrib| QName(&attrib.value)).unwrap_or_else(|| b.name());

            #( #variants_init )*
        });

        let impl_next = self.derived_types.iter().map(|x| {
            let variant_ident = &x.variant_ident;

            quote! {
                Self::#variant_ident(x) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = x.next(reader, event)?;

                    let data = data.map(|x| #type_ident(Box::new(x)));
                    let deserializer = deserializer.map(|x| Self::#variant_ident(x));

                    Ok(DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    })
                },
            }
        });

        let impl_finish = self.derived_types.iter().map(|x| {
            let variant_ident = &x.variant_ident;

            quote! {
                Self::#variant_ident(x) => Ok(#type_ident(Box::new(x.finish(reader)?))),
            }
        });

        let code = quote! {
            impl #xsd_parser::quick_xml::WithDeserializer for #type_ident {
                type Deserializer = quick_xml_deserialize::#deserializer_ident;
            }
        };

        let deserializer_code = quote! {
            #[derive(Debug)]
            pub enum #deserializer_ident {
                #( #variants )*
            }

            impl<'de> #xsd_parser::quick_xml::Deserializer<'de, super::#type_ident> for #deserializer_ident {
                fn init<R>(
                    reader: &R,
                    event: #xsd_parser::quick_xml::Event<'de>
                ) -> #xsd_parser::quick_xml::DeserializerResult<'de, super::#type_ident, Self>
                where
                    R: #xsd_parser::quick_xml::XmlReader
                {
                    use #xsd_parser::quick_xml::{Event, ResolveResult, QName, Namespace, DeserializerOutput};

                    #( #namespace_consts )*

                    let (Event::Start(b) | Event::Empty(b)) = &event else {
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: None,
                            event: None,
                            allow_any: false,
                        });
                    };

                    #impl_init

                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: None,
                        event: Some(event),
                        allow_any: false,
                    })
                }

                fn next<R>(
                    self,
                    reader: &R,
                    event: #xsd_parser::quick_xml::Event<'de>
                ) -> #xsd_parser::quick_xml::DeserializerResult<'de, super::#type_ident, Self>
                where
                    R: #xsd_parser::quick_xml::XmlReader
                {
                    use #xsd_parser::quick_xml::DeserializerOutput;

                    match self {
                        #( #impl_next )*
                    }
                }

                fn finish<R>(
                    self,
                    reader: &R
                ) -> Result<super::#type_ident, #xsd_parser::quick_xml::Error>
                where
                    R: #xsd_parser::quick_xml::XmlReader
                {
                    match self {
                        #( #impl_finish )*
                    }
                }
            }
        };

        (code, deserializer_code)
    }
}

/* ComplexTypeImpl */

impl ComplexTypeImpl<'_, '_, '_> {
    fn render_deserializer(&mut self) -> (TokenStream, TokenStream) {
        let type_ident = self.type_ident;
        let xsd_parser = &self.xsd_parser_crate;
        let deserializer_ident = &self.deserializer_ident;

        let deserializer_type = self.render_deserializer_type();
        let state_type = self.render_deserializer_state_type();

        let fn_from_bytes_start = self
            .is_static_complex
            .then(|| self.render_deserializer_fn_from_bytes_start());

        let fn_init = self.render_deserializer_fn_init();
        let fn_next = self.render_deserializer_fn_next();
        let fn_finish = self.render_deserializer_fn_finish();

        let code = quote! {
            impl #xsd_parser::quick_xml::WithDeserializer for #type_ident {
                type Deserializer = quick_xml_deserialize::#deserializer_ident;
            }
        };

        let deserializer_code = quote! {
            #deserializer_type
            #state_type

            impl #deserializer_ident {
                #fn_from_bytes_start
            }

            impl<'de> #xsd_parser::quick_xml::Deserializer<'de, super::#type_ident> for #deserializer_ident {
                #fn_init
                #fn_next
                #fn_finish
            }
        };

        (code, deserializer_code)
    }

    /* Deserializer Type */

    fn render_deserializer_type(&self) -> TokenStream {
        match self.target_mode {
            TypeMode::Choice => self.render_deserializer_type_enum(),
            TypeMode::Simple | TypeMode::All | TypeMode::Sequence => {
                self.render_deserializer_type_struct()
            }
        }
    }

    fn render_deserializer_type_enum(&self) -> TokenStream {
        let content_ident = &self.content_ident;
        let deserializer_ident = &self.deserializer_ident;
        let state_ident = &self.deserializer_state_ident;
        let attributes = self
            .attributes
            .iter()
            .map(AttributeImpl::deserializer_field);

        let content = match self.occurs {
            Occurs::None => None,
            Occurs::Single | Occurs::Optional => Some(quote! {
                content: Option<super::#content_ident>,
            }),
            Occurs::DynamicList | Occurs::StaticList(_) => Some(quote! {
                content: Vec<super::#content_ident>,
            }),
        };

        let state = self.elements.is_empty().not().then(|| {
            quote! {
                state: Box<#state_ident>,
            }
        });

        quote! {
            #[derive(Debug)]
            pub struct #deserializer_ident {
                #( #attributes )*
                #content
                #state
            }
        }
    }

    fn render_deserializer_type_struct(&self) -> TokenStream {
        let deserializer_ident = &self.deserializer_ident;
        let state_ident = &self.deserializer_state_ident;
        let attributes = self
            .attributes
            .iter()
            .map(AttributeImpl::deserializer_field);
        let elements = self.elements.iter().map(ElementImpl::deserializer_field);
        let simple_content = self
            .simple_content
            .as_ref()
            .map(SimpleContentData::deserializer_field);
        let fields = elements.chain(simple_content);

        let state = if let Some(simple) = &self.simple_content {
            let xsd_crate = &self.xsd_parser_crate;
            let target_type = &simple.target_type;

            Some(quote! {
                state: Option<#xsd_crate::quick_xml::ContentDeserializer<#target_type>>,
            })
        } else if !self.elements.is_empty() {
            Some(quote! {
                state: Box<#state_ident>,
            })
        } else {
            None
        };

        quote! {
            #[derive(Debug)]
            pub struct #deserializer_ident {
                #( #attributes )*
                #( #fields )*
                #state
            }
        }
    }

    /* State Type */

    fn render_deserializer_state_type(&self) -> TokenStream {
        if self.elements.is_empty() {
            return TokenStream::new();
        }

        match self.target_mode {
            TypeMode::All | TypeMode::Choice => self.render_deserializer_state_type_choice(),
            TypeMode::Sequence => self.render_deserializer_state_type_sequence(),
            TypeMode::Simple => crate::unreachable!(),
        }
    }

    fn render_deserializer_state_type_choice(&self) -> TokenStream {
        let state_ident = &self.deserializer_state_ident;
        let xsd_parser = &self.xsd_parser_crate;

        let variants = self.elements.iter().map(|f| {
            let target_type = &f.target_type;
            let variant_ident = &f.variant_ident;

            quote! {
                #variant_ident(<#target_type as #xsd_parser::quick_xml::WithDeserializer>::Deserializer),
            }
        });

        quote! {
            #[derive(Debug)]
            enum #state_ident {
                Next__,
                #( #variants )*
            }
        }
    }

    fn render_deserializer_state_type_sequence(&self) -> TokenStream {
        let state_ident = &self.deserializer_state_ident;
        let xsd_parser = &self.xsd_parser_crate;

        let variants = self
            .elements
            .iter()
            .map(|f| {
                let target_type = &f.target_type;
                let variant_ident = &f.variant_ident;

                quote! {
                    #variant_ident(Option<<#target_type as #xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
                }
            });

        quote! {
            #[derive(Debug)]
            enum #state_ident {
                #( #variants )*
                Done__,
            }
        }
    }

    /* fn from_bytes_start */

    #[allow(clippy::too_many_lines)]
    fn render_deserializer_fn_from_bytes_start(&self) -> TokenStream {
        fn render_var(attrib: &AttributeImpl<'_, '_>) -> TokenStream {
            let field_ident = &attrib.field_ident;
            let target_type = &attrib.target_type;

            quote!(let mut #field_ident: Option<#target_type> = None;)
        }

        fn render_match(
            types: &Types,
            is_first: bool,
            attrib: &AttributeImpl<'_, '_>,
        ) -> TokenStream {
            let b_name = &attrib.b_name;
            let field_ident = &attrib.field_ident;

            let else_ = is_first.not().then(|| quote!(else));

            if let Some(module) = attrib.ident.ns.and_then(|ns| types.modules.get(&ns)) {
                let ns_name = make_ns_const(module);

                quote! {
                    #else_ if matches!(reader.resolve_local_name(attrib.key, #ns_name), Some(#b_name)) {
                        reader.read_attrib(&mut #field_ident, #b_name, &attrib.value)?;
                    }
                }
            } else {
                quote! {
                    #else_ if attrib.key.local_name().as_ref() == #b_name {
                        reader.read_attrib(&mut #field_ident, #b_name, &attrib.value)?;
                    }
                }
            }
        }

        fn render_finish(attrib: &AttributeImpl<'_, '_>, type_ident: &Ident2) -> TokenStream {
            let field_ident = &attrib.field_ident;

            let convert = if attrib.default_value.is_some() {
                let default_fn_ident = format_ident!("default_{field_ident}");

                Some(quote! { .unwrap_or_else(super:: #type_ident :: #default_fn_ident) })
            } else if attrib.use_ == Use::Required {
                let name = &attrib.s_name;

                Some(quote! { .ok_or(ErrorKind::MissingAttribute(#name.into()))? })
            } else {
                None
            };

            quote! {
                #field_ident: #field_ident #convert,
            }
        }

        fn render_init(element: &ElementImpl<'_, '_>) -> TokenStream {
            let occurs = element.occurs;
            let field_ident = &element.field_ident;

            match occurs {
                Occurs::None => quote!(),
                Occurs::Single | Occurs::Optional => quote!(#field_ident: None,),
                Occurs::DynamicList | Occurs::StaticList(_) => quote!(#field_ident: Vec::new(),),
            }
        }

        let type_ident = self.type_ident;
        let state_ident = &self.deserializer_state_ident;
        let xsd_parser = &self.xsd_parser_crate;

        let attribute_namespaces = unique_namespaces(
            self.types,
            self.attributes.iter().filter_map(|attrib| attrib.ident.ns),
        );

        let attributes_var = self.attributes.iter().map(render_var);
        let attributes_match = self
            .attributes
            .iter()
            .enumerate()
            .map(|(i, a)| render_match(self.types, i == 0, a));
        let attributes_finish = self.attributes.iter().map(|a| render_finish(a, type_ident));

        let attrib_default = if self.any_attribute.is_some() {
            None
        } else {
            let default_ = quote! {
                reader.err(
                    ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(
                            attrib.key.into_inner()
                        )
                    )
                )?;
            };

            Some(if self.attributes.is_empty() {
                default_
            } else {
                quote! {
                    else {
                        #default_
                    }
                }
            })
        };

        let need_attributes_loop = !self.attributes.is_empty() || attrib_default.is_some();
        let attributes_loop = need_attributes_loop.then(|| {
            quote! {
                for attrib in bytes_start.attributes() {
                    let attrib = attrib?;

                    if matches!(attrib.key.prefix(), Some(x) if x.as_ref() == b"xmlns") {
                        continue;
                    }

                    #( #attributes_match )*

                    #attrib_default
                }
            }
        });

        let content_finish = match self.target_mode {
            TypeMode::Simple => {
                quote! {
                    content: None,
                    state: None,
                }
            }
            TypeMode::All | TypeMode::Sequence => {
                let elements_finish = self.elements.iter().map(render_init);

                quote! {
                    #( #elements_finish )*
                }
            }
            TypeMode::Choice => match self.occurs {
                Occurs::None => quote!(),
                Occurs::Single | Occurs::Optional => quote! {
                    content: None,
                },
                Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                    content: Vec::new(),
                },
            },
        };

        let state_finish = self.elements.is_empty().not().then(|| {
            let state = match self.target_mode {
                TypeMode::Simple => crate::unreachable!(),
                TypeMode::All | TypeMode::Choice => quote!(#state_ident::Next__),
                TypeMode::Sequence => {
                    if let Some(f) = self.elements.first() {
                        let variant_ident = &f.variant_ident;

                        quote!(#state_ident::#variant_ident(None))
                    } else {
                        quote!(#state_ident::Done__)
                    }
                }
            };

            quote! {
                state: Box::new(#state),
            }
        });

        quote! {
            fn from_bytes_start<R>(
                reader: &R,
                bytes_start: &#xsd_parser::quick_xml::BytesStart<'_>
            ) -> Result<Self, #xsd_parser::quick_xml::Error>
            where
                R: #xsd_parser::quick_xml::XmlReader,
            {
                use #xsd_parser::quick_xml::ErrorKind;

                #( #attribute_namespaces )*
                #( #attributes_var )*

                #attributes_loop

                Ok(Self {
                    #( #attributes_finish )*
                    #content_finish
                    #state_finish
                })
            }
        }
    }

    /* fn init */

    fn render_deserializer_fn_init(&self) -> TokenStream {
        let type_ident = self.type_ident;
        let xsd_parser = &self.xsd_parser_crate;

        let fn_impl = if self.target_mode == TypeMode::Simple {
            self.render_deserializer_fn_init_simple()
        } else if self.is_static_complex {
            self.render_deserializer_fn_init_complex()
        } else {
            match self.target_mode {
                TypeMode::All | TypeMode::Choice => self.render_deserializer_fn_init_enum(),
                TypeMode::Sequence => self.render_deserializer_fn_init_struct(),
                TypeMode::Simple => crate::unreachable!(),
            }
        };

        quote! {
            fn init<R>(
                reader: &R,
                event: #xsd_parser::quick_xml::Event<'de>,
            ) -> #xsd_parser::quick_xml::DeserializerResult<'de, super::#type_ident, Self>
            where
                R: #xsd_parser::quick_xml::XmlReader,
            {
                #fn_impl
            }
        }
    }

    fn render_deserializer_fn_init_simple(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let expr = quote!(ContentDeserializer::init(reader, event)?);
        let handler = make_simple_content_deserializer_output_handler(&expr);

        quote! {
            use #xsd_parser::quick_xml::{Event, ContentDeserializer, DeserializerOutput};

            let (Event::Start(start) | Event::Empty(start)) = &event else {
                return Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                });
            };

            let mut this = Self::from_bytes_start(reader, &start)?;

            #handler
        }
    }

    fn render_deserializer_fn_init_complex(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;

        quote! {
            use #xsd_parser::quick_xml::{Event, DeserializerOutput};

            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;

                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;

                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
    }

    fn render_deserializer_fn_init_enum(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let state_ident = &self.deserializer_state_ident;
        let content_init = match self.occurs {
            Occurs::None => None,
            Occurs::Single | Occurs::Optional => Some(quote! {
                content: None,
            }),
            Occurs::DynamicList | Occurs::StaticList(_) => Some(quote! {
                content: Vec::new(),
            }),
        };

        quote! {
            use #xsd_parser::quick_xml::{Event, DeserializerOutput};

            let deserializer = Self {
                #content_init
                state: Box::new(#state_ident::Next__),
            };

            let is_empty = matches!(event, Event::Empty(_));

            let mut out = deserializer.next(reader, event)?;
            if out.event.is_some() {
                out.deserializer = None;
            } else if is_empty && out.data.is_none() {
                if let Some(deserializer) = out.deserializer.take() {
                    out.data = Some(deserializer.finish(reader)?);
                }
            }

            Ok(out)
        }
    }

    fn render_deserializer_fn_init_struct(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let state_ident = &self.deserializer_state_ident;
        let elements = self
            .elements
            .iter()
            .map(ElementImpl::deserializer_init_field);
        let state = if let Some(first) = self.elements.first() {
            let variant_ident = &first.variant_ident;

            quote!(#state_ident::#variant_ident(None))
        } else {
            quote!(#state_ident::Done__)
        };

        quote! {
            use #xsd_parser::quick_xml::DeserializerOutput;

            let deserializer = Self {
                #( #elements )*
                state: Box::new(#state),
            };

            let mut out = deserializer.next(reader, event)?;
            if out.event.is_some() {
                out.deserializer = None;
            }

            Ok(out)
        }
    }

    /* fn next */

    fn render_deserializer_fn_next(&self) -> TokenStream {
        let type_ident = self.type_ident;
        let xsd_parser = &self.xsd_parser_crate;

        let fn_impl = match self.target_mode {
            TypeMode::All | TypeMode::Choice => self.render_deserializer_fn_next_enum(),
            TypeMode::Sequence => self.render_deserializer_fn_next_struct(),
            TypeMode::Simple => self.render_deserializer_fn_next_simple(),
        };

        let this = if self.elements.is_empty() {
            quote!(self)
        } else {
            quote!(mut self)
        };

        quote! {
            fn next<R>(
                #this,
                reader: &R,
                event: #xsd_parser::quick_xml::Event<'de>,
            ) -> #xsd_parser::quick_xml::DeserializerResult<'de, super::#type_ident, Self>
            where
                R: #xsd_parser::quick_xml::XmlReader,
            {
                #fn_impl
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn render_deserializer_fn_next_enum(&self) -> TokenStream {
        fn render_handler(
            state_ident: &Ident2,
            element: &ElementImpl<'_, '_>,
            expr: &TokenStream,
            setter: &TokenStream,
        ) -> TokenStream {
            let variant_ident = &element.variant_ident;

            quote! {
                let DeserializerOutput { data, deserializer, event, allow_any } = #expr;

                if let Some(data) = data {
                    #setter
                }

                if let Some(deserializer) = deserializer {
                    *self.state = #state_ident::#variant_ident(deserializer);
                }

                Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event,
                    allow_any,
                })
            }
        }

        let state_ident = &self.deserializer_state_ident;
        let xsd_parser = &self.xsd_parser_crate;

        let element_namespaces = unique_namespaces(
            self.types,
            self.elements.iter().filter_map(|el| el.ident.ns),
        );

        let elements = self
            .elements
            .iter()
            .filter(|el| {
                el.element_mode == ElementMode::Element
                    && !el.is_dynamic
            })
            .enumerate()
            .map(|(index, element)| {
                let b_name = &element.b_name;
                let setter = self.render_setter(element);
                let target_type = &element.target_type;

                let else_ = (index > 0).then(|| quote!(else));

                let expr = quote!(<#target_type as WithDeserializer>::Deserializer::init(reader, event)?);
                let handler = render_handler(state_ident, element, &expr, &setter);

                if let Some(module) = element.ident.ns.and_then(|ns| self.types.modules.get(&ns)) {
                    let ns_name = make_ns_const(module);

                    quote! {
                        #else_ if matches!(reader.resolve_local_name(x.name(), #ns_name), Some(#b_name)) {
                            #handler
                        }
                    }
                } else {
                    quote! {
                        #else_ if x.name().local_name() == #b_name {
                            #handler
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let groups = self
            .elements
            .iter()
            .filter_map(|element| {
                if element.element_mode != ElementMode::Group && !element.is_dynamic {
                    return None;
                }

                let setter = self.render_setter(element);
                let target_type = &element.target_type;
                let variant_ident = &element.variant_ident;

                Some(quote! {
                    let event = {
                        let DeserializerOutput { data, deserializer, event, allow_any } = <#target_type as WithDeserializer>::Deserializer::init(reader, event)?;

                        if let Some(data) = data {
                            #setter
                        }

                        if let Some(deserializer) = deserializer {
                            *self.state = #state_ident::#variant_ident(deserializer);
                        }

                        let Some(event) = event else {
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: None,
                                allow_any,
                            })
                        };

                        if allow_any {
                            allow_any_element = true;
                        }

                        event
                    };
                })
            })
            .collect::<Vec<_>>();

        let states = self.elements.iter().map(|element| {
            let variant_ident = &element.variant_ident;
            let setter = self.render_setter(element);

            let expr = quote!(deserializer.next(reader, event)?);
            let handler = render_handler(state_ident, element, &expr, &setter);

            quote! {
                (#state_ident::#variant_ident(deserializer), _) => { #handler }
            }
        });

        let name_fallback = if groups.is_empty() {
            quote! {
                Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: Some(event),
                    allow_any: false,
                })
            }
        } else {
            quote! {{
                let mut allow_any_element = false;

                #( #groups )*

                Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: Some(event),
                    allow_any: allow_any_element,
                })
            }}
        };

        let name_fallback = if elements.is_empty() {
            name_fallback
        } else {
            quote! {
                else {
                    #name_fallback
                }
            }
        };

        let next_end_event = if self.is_static_complex {
            quote!(None)
        } else {
            quote!(Some(event))
        };

        quote! {
            use #xsd_parser::quick_xml::{Event, WithDeserializer, DeserializerOutput, RawByteStr, ErrorKind};

            #( #element_namespaces )*

            match (core::mem::replace(&mut *self.state, #state_ident::Next__), &event) {
                (#state_ident::Next__, Event::Start(x) | Event::Empty(x)) => {
                    #( #elements )*
                    #name_fallback
                }
                (#state_ident::Next__, Event::End(_)) => {
                    let data = self.finish(reader)?;

                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: #next_end_event,
                        allow_any: false,
                    })
                }
                (#state_ident::Next__, _) => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: None,
                    allow_any: false,
                }),
                #( #states )*
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn render_deserializer_fn_next_struct(&self) -> TokenStream {
        let state_ident = &self.deserializer_state_ident;
        let xsd_parser = &self.xsd_parser_crate;

        let element_namespaces = unique_namespaces(
            self.types,
            self.elements.iter().filter_map(|el| el.ident.ns),
        );

        let states = self.elements.iter().enumerate().map(|(i, element)| {
            let setter = self.render_setter(element);

            let name = &element.b_name;
            let target_type = &element.target_type;
            let variant_ident = &element.variant_ident;

            let next_state = if let Some(next) = self.elements.get(i + 1) {
                let variant_ident = &next.variant_ident;

                quote!(#state_ident::#variant_ident(None))
            } else {
                quote!(#state_ident::Done__)
            };

            let allow_any = self.any_element.is_some() || element.new_target_type_allows_any(self);
            let allow_any = allow_any.then(|| quote!{
                allow_any_fallback.get_or_insert(#state_ident::#variant_ident(None));
            });

            let module = element.ident.ns.and_then(|ns| self.types.modules.get(&ns));
            let name_matcher = match (element.is_dynamic, element.element_mode, module) {
                (false, ElementMode::Element, Some(module)) => {
                    let ns_name = make_ns_const(module);

                    quote!(Event::Start(x) | Event::Empty(x) if matches!(reader.resolve_local_name(x.name(), #ns_name), Some(#name)))
                },
                (false, ElementMode::Element, None) => quote!(Event::Start(x) | Event::Empty(x) if x.name().local_name().as_ref() == #name),
                (true, _, _) | (_, ElementMode::Group, _) => quote!(Event::Start(_) | Event::Empty(_)),
            };

            let need_fallback_matcher = element.element_mode == ElementMode::Element && !element.is_dynamic;
            let fallback_matcher = need_fallback_matcher.then(|| quote!{
                Event::Start(_) | Event::Empty(_) => {
                    *self.state = #next_state;

                    #allow_any

                    event
                }
            });

            quote!{
                (#state_ident::#variant_ident(Some(deserializer)), event) => {
                    let DeserializerOutput { data, deserializer, event, allow_any } = deserializer.next(reader, event)?;

                    if let Some(data) = data {
                        #setter
                    }

                    let event = match event {
                        Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => event,
                        event => {
                            *self.state = #state_ident::#variant_ident(deserializer);

                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: event,
                                allow_any: false,
                            })
                        }
                    };

                    if allow_any {
                        allow_any_fallback.get_or_insert(#state_ident::#variant_ident(deserializer));
                    } else if let Some(deserializer) = deserializer {
                        let data = deserializer.finish(reader)?;

                        #setter
                    }

                    *self.state = #state_ident::#variant_ident(None);

                    event
                }
                (#state_ident::#variant_ident(None), event) => match &event {
                    #name_matcher => {
                        let DeserializerOutput { data, deserializer, event, allow_any } = <#target_type as WithDeserializer>::Deserializer::init(reader, event)?;

                        if let Some(data) = data {
                            #setter
                        }

                        *self.state = #state_ident::#variant_ident(deserializer);

                        match event {
                            Some(event @ (Event::Start(_) | Event::End(_))) => {
                                *self.state = #next_state;

                                if allow_any {
                                    allow_any_fallback.get_or_insert(#state_ident::#variant_ident(None));
                                }

                                event
                            }
                            event @ (None | Some(_)) => return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event,
                                allow_any: false,
                            })
                        }
                    }
                    #fallback_matcher
                    Event::End(_) => {
                        let data = self.finish(reader)?;

                        return Ok(DeserializerOutput {
                            data: Some(data),
                            deserializer: None,
                            event: None,
                            allow_any: false,
                        });
                    }
                    _ => {
                        *self.state = #state_ident::#variant_ident(None);

                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any: false,
                        });
                    }
                }
            }
        });

        if self.elements.is_empty() {
            let allow_any = self.any_element.is_some();

            quote! {
                use #xsd_parser::quick_xml::{Event, DeserializerOutput};

                match event {
                    Event::End(_) => {
                        let data = self.finish(reader)?;

                        Ok(DeserializerOutput {
                            data: Some(data),
                            deserializer: None,
                            event: None,
                            allow_any: false,
                        })
                    }
                    _ => {
                        Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any: #allow_any,
                        })
                    }
                }
            }
        } else {
            quote! {
                use #xsd_parser::quick_xml::{Event, Deserializer, WithDeserializer, RawByteStr, ErrorKind, DeserializerOutput};

                #( #element_namespaces )*

                let mut event = event;
                let mut allow_any_fallback = None;

                loop {
                    event = match (core::mem::replace(&mut *self.state, #state_ident::Done__), event) {
                        #( #states )*
                        (#state_ident::Done__, event) => {
                            let allow_any = if let Some(fallback) = allow_any_fallback {
                                *self.state = fallback;

                                true
                            } else {
                                false
                            };

                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any,
                            });
                        }
                    }
                }
            }
        }
    }

    fn render_deserializer_fn_next_simple(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let expr = quote!(this.state.take().unwrap().next(reader, event)?);
        let handler = make_simple_content_deserializer_output_handler(&expr);

        quote! {
            use #xsd_parser::quick_xml::DeserializerOutput;

            let mut this = self;

            #handler
        }
    }

    /* fn finish */

    #[allow(clippy::too_many_lines)]
    fn render_deserializer_fn_finish(&self) -> TokenStream {
        fn render_attrib(a: &AttributeImpl<'_, '_>) -> TokenStream {
            let field_ident = &a.field_ident;

            quote! {
                #field_ident: self.#field_ident,
            }
        }

        fn render_element(element: &ElementImpl<'_, '_>) -> TokenStream {
            let name = &element.s_name;
            let field_ident = &element.field_ident;

            let convert = match element.occurs {
                Occurs::None => crate::unreachable!(),
                Occurs::Single => {
                    let mut code = quote! {
                        self.#field_ident.ok_or_else(|| ErrorKind::MissingElement(#name.into()))?
                    };

                    if element.need_box {
                        code = quote! { Box::new(#code) };
                    }

                    code
                }
                Occurs::Optional if element.need_box => {
                    quote! { self.#field_ident.map(Box::new) }
                }
                Occurs::Optional | Occurs::DynamicList => {
                    quote! { self.#field_ident }
                }
                Occurs::StaticList(sz) => {
                    quote! {
                        self.#field_ident.try_into().map_err(|vec: Vec<_>| ErrorKind::InsufficientSize {
                            min: #sz,
                            max: #sz,
                            actual: vec.len(),
                        })?
                    }
                }
            };

            quote! {
                #field_ident: #convert,
            }
        }

        let type_ident = self.type_ident;
        let xsd_parser = &self.xsd_parser_crate;

        let attributes_finish = self.attributes.iter().map(render_attrib);

        let finish_impl = match self.target_mode {
            TypeMode::Choice => {
                let content_finish = match self.occurs {
                    Occurs::None => None,
                    Occurs::Single => Some(quote! {
                        self.content.ok_or(#xsd_parser::quick_xml::ErrorKind::MissingContent)?
                    }),
                    Occurs::Optional | Occurs::DynamicList => Some(quote! {
                        self.content
                    }),
                    Occurs::StaticList(sz) => Some(quote! {
                        self.content.try_into().map_err(|vec: Vec<_>| #xsd_parser::quick_xml::ErrorKind::InsufficientSize {
                            min: #sz,
                            max: #sz,
                            actual: vec.len(),
                        })
                    }),
                };

                if self.flatten_content {
                    quote!(Ok(#content_finish))
                } else {
                    quote! {
                        Ok(super::#type_ident {
                            #( #attributes_finish )*
                            content: #content_finish,
                        })
                    }
                }
            }
            TypeMode::All | TypeMode::Sequence => {
                let elements_finish = self.elements.iter().map(render_element);

                quote! {
                    Ok(super::#type_ident {
                        #( #attributes_finish )*
                        #( #elements_finish )*
                    })
                }
            }
            TypeMode::Simple => quote! {
                Ok(super::#type_ident {
                    #( #attributes_finish )*
                    content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
                })
            },
        };

        quote! {
            fn finish<R>(self, _reader: &R) -> Result<super::#type_ident, #xsd_parser::quick_xml::Error>
            where
                R: #xsd_parser::quick_xml::XmlReader,
            {
                use #xsd_parser::quick_xml::ErrorKind;

                #finish_impl
            }
        }
    }

    fn render_setter(&self, element: &ElementImpl<'_, '_>) -> TokenStream {
        let name = &element.b_name;

        match self.target_mode {
            TypeMode::Choice => {
                let content_ident = &self.content_ident;
                let variant_ident = &element.variant_ident;

                let data = if element.need_box {
                    quote!(Box::new(data))
                } else {
                    quote!(data)
                };

                match self.occurs {
                    Occurs::Single | Occurs::Optional => quote! {
                        if self.content.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(#name)))?;
                        }

                        self.content = Some(#content_ident::#variant_ident(#data));
                    },
                    Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                        self.content.push(#content_ident::#variant_ident(data));
                    },
                    e => crate::unreachable!("{:?}", e),
                }
            }
            TypeMode::All | TypeMode::Sequence => {
                let field_ident = &element.field_ident;

                match element.occurs {
                    Occurs::Single | Occurs::Optional => quote! {
                        if self.#field_ident.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(#name)))?;
                        }

                        self.#field_ident = Some(data);
                    },
                    Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                        self.#field_ident.push(data);
                    },
                    e => crate::unreachable!("{:?}", e),
                }
            }
            TypeMode::Simple => crate::unreachable!(),
        }
    }
}

/* AttributeImpl */

impl AttributeImpl<'_, '_> {
    fn deserializer_field(&self) -> TokenStream {
        let field_ident = &self.field_ident;
        let target_type = &self.target_type;

        let target_type = if self.type_.is_build_in() {
            quote!(#target_type)
        } else {
            quote!(super::#target_type)
        };

        let target_type = if self.is_option {
            quote!(Option<#target_type>)
        } else {
            target_type
        };

        quote! {
            #field_ident: #target_type,
        }
    }
}

/* ElementImpl */

impl ElementImpl<'_, '_> {
    fn deserializer_field(&self) -> TokenStream {
        let field_ident = &self.field_ident;
        let target_type = &self.target_type;

        let target_type = if self.element.type_.is_build_in() {
            quote!(#target_type)
        } else {
            quote!(super::#target_type)
        };

        let target_type = match self.occurs {
            Occurs::Single | Occurs::Optional => quote!(Option<#target_type>),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(Vec<#target_type>),
            e => crate::unreachable!("{:?}", e),
        };

        quote! {
            #field_ident: #target_type,
        }
    }

    fn deserializer_init_field(&self) -> TokenStream {
        let field_ident = &self.field_ident;

        let field_init = match self.occurs {
            Occurs::Single | Occurs::Optional => quote!(None),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(Vec::new()),
            e => crate::unreachable!("{:?}", e),
        };

        quote! {
            #field_ident: #field_init,
        }
    }
}

impl SimpleContentData<'_> {
    fn deserializer_field(&self) -> TokenStream {
        let target_type = &self.target_type;

        let target_type = if self.ident.is_build_in() {
            quote!(#target_type)
        } else {
            quote!(super::#target_type)
        };

        quote! {
            content: Option<#target_type>,
        }
    }
}

fn unique_namespaces<I>(types: &Types, iter: I) -> impl Iterator<Item = TokenStream> + '_
where
    I: IntoIterator<Item = NamespaceId>,
{
    iter.into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter_map(|ns| {
            let module = types.modules.get(&ns)?;
            let ns = module.namespace.as_ref()?;
            let const_name = make_ns_const(module);
            let namespace = Literal::byte_string(&ns.0);

            Some(quote! {
                const #const_name: &[u8] = #namespace;
            })
        })
}

fn make_ns_const(module: &Module) -> Ident2 {
    format_ident!(
        "NS_{}",
        module
            .name
            .as_ref()
            .map_or_else(|| String::from("DEFAULT"), ToString::to_string)
            .to_screaming_snake_case()
    )
}

fn make_simple_content_deserializer_output_handler(expr: &TokenStream) -> TokenStream {
    quote! {
        let DeserializerOutput {
            data,
            deserializer,
            event,
            allow_any,
        } = #expr;

        if let Some(data) = data {
            this.content = Some(data);
            let data = this.finish(reader)?;

            Ok(DeserializerOutput {
                data: Some(data),
                deserializer: None,
                event,
                allow_any,
            })
        } else if let Some(state) = deserializer {
            this.state = Some(state);

            Ok(DeserializerOutput {
                data: None,
                deserializer: Some(this),
                event,
                allow_any,
            })
        } else {
            Ok(DeserializerOutput {
                data: None,
                deserializer: None,
                event,
                allow_any,
            })
        }
    }
}
