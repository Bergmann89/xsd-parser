use std::collections::BTreeSet;
use std::ops::Not;

use inflector::Inflector;
use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote};
use tracing::instrument;

use crate::schema::NamespaceId;
use crate::types::{Module, Types};

use super::super::super::data::{
    ComplexTypeData, DynamicData, EnumVariantData, EnumerationData, ReferenceData, UnionData,
    UnionVariantData,
};
use super::super::super::misc::{Occurs, StateFlags, TypedefMode};
use super::{ComplexTypeImpl, DynamicTypeImpl, QuickXmlRenderer};

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
        let _self = self;

        (quote!(), quote!())
    }
}

/* Misc */

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
