use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use tracing::instrument;

use crate::generator::misc::TypeMode;
use crate::schema::Namespace;

use super::super::super::data::{
    ComplexTypeData, DynamicData, EnumVariantData, EnumerationData, ReferenceData, UnionData,
    UnionVariantData,
};
use super::super::super::misc::{Occurs, StateFlags, TypedefMode};
use super::{AttributeImpl, ComplexTypeImpl, ElementImpl, QuickXmlRenderer};

/* Serialize */

impl QuickXmlRenderer {
    #[instrument(level = "trace", skip(self))]
    pub fn render_union_serialize(&mut self, data: &mut UnionData<'_, '_>) {
        let UnionData {
            inner, variants, ..
        } = data;

        let xsd_parser = inner.xsd_parser_crate.clone();
        let type_ref = inner.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_SERIALIZE) {
            return;
        }

        let type_ident = &type_ref.type_ident;

        let variants = variants
            .iter()
            .map(|data| {
                let UnionVariantData { variant_ident, .. } = data;

                quote! {
                    Self::#variant_ident(x) => x.serialize_bytes(),
                }
            })
            .collect::<Vec<_>>();

        let code = quote! {
            impl #xsd_parser::quick_xml::SerializeBytes for #type_ident {
                fn serialize_bytes(&self) -> Result<Option<std::borrow::Cow<'_, str>>, #xsd_parser::quick_xml::Error> {
                    match self {
                        #( #variants )*
                    }
                }
            }
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_dynamic_serialize(&mut self, data: &mut DynamicData<'_, '_>) {
        crate::unimplemented!();
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_reference_serialize(&mut self, data: &mut ReferenceData<'_, '_>) {
        let ReferenceData {
            mode,
            occurs,
            inner,
            type_ident,
            ..
        } = data;

        let xsd_parser = inner.xsd_parser_crate.clone();
        let type_ref = inner.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_SERIALIZE)
            || matches!(mode, TypedefMode::Auto | TypedefMode::Typedef)
        {
            return;
        }

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
                                data.push_str(" ");
                            }

                            data.push_str(&bytes);
                        }
                    }

                    Ok(Some(std::borrow::Cow::Owned(data)))
                }
            }
        };

        let code = quote! {
            impl #xsd_parser::quick_xml::SerializeBytes for #type_ident {
                fn serialize_bytes(&self) -> Result<Option<std::borrow::Cow<'_, str>>, #xsd_parser::quick_xml::Error> {
                    #body
                }
            }
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_enumeration_serialize(&mut self, data: &mut EnumerationData<'_, '_>) {
        let EnumerationData {
            inner, variants, ..
        } = data;

        let xsd_parser = inner.xsd_parser_crate.clone();
        let type_ref = inner.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_SERIALIZE) {
            return;
        }

        let type_ident = &type_ref.type_ident;

        let variants = variants.iter().map(|data| {
            let EnumVariantData {
                var,
                target_type,
                variant_ident,
            } = data;

            if target_type.is_some() {
                quote! {
                    Self::#variant_ident(x) => x.serialize_bytes(),
                }
            } else {
                let name = Literal::string(&var.ident.name.to_string());

                quote! {
                    Self::#variant_ident => Ok(Some(std::borrow::Cow::Borrowed(#name))),
                }
            }
        });

        let code = quote! {
            impl #xsd_parser::quick_xml::SerializeBytes for #type_ident {
                fn serialize_bytes(&self) -> Result<Option<std::borrow::Cow<'_, str>>, #xsd_parser::quick_xml::Error> {
                    match self {
                        #( #variants )*
                    }
                }
            }
        };

        data.add_code(code);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn render_complex_type_serialize(&mut self, data: &mut ComplexTypeData<'_, '_>) {
        let type_ref = data.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_SERIALIZE) {
            return;
        }

        let (code, serializer_code) = ComplexTypeImpl::new(data).render_serializer();

        data.add_code(code);
        data.add_quick_xml_serialize_code(serializer_code);
    }
}

/* ComplexTypeImpl */

impl ComplexTypeImpl<'_, '_, '_> {
    fn render_serializer(&mut self) -> (TokenStream, TokenStream) {
        let tag_name = &self.tag_name;
        let type_ident = self.type_ident;
        let xsd_parser = &self.xsd_parser_crate;
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        let state_type = self.render_serializer_state_type();

        let fn_next = self.render_serializer_fn_next();

        let code = quote! {
            impl #xsd_parser::quick_xml::WithSerializer for #type_ident {
                type Serializer<'x> = quick_xml_serialize::#serializer_ident<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: Option<&'ser str>,
                    is_root: bool
                ) -> Result<Self::Serializer<'ser>, #xsd_parser::quick_xml::Error> {
                    quick_xml_serialize::#serializer_ident::new(self, name, is_root)
                }
            }
        };

        let serializer_code = quote! {
            #[derive(Debug)]
            pub struct #serializer_ident<'ser> {
                name: &'ser str,
                value: &'ser super::#type_ident,
                is_root: bool,
                state: #serializer_state_ident<'ser>,
            }

            #state_type

            impl<'ser> #serializer_ident<'ser> {
                pub(super) fn new(
                    value: &'ser super::#type_ident,
                    name: Option<&'ser str>,
                    is_root: bool
                ) -> Result<Self, #xsd_parser::quick_xml::Error> {
                    let name = name.unwrap_or(#tag_name);

                    Ok(Self {
                        name,
                        value,
                        is_root,
                        state: #serializer_state_ident::Init__,
                    })
                }
            }

            impl<'ser> core::iter::Iterator for #serializer_ident<'ser> {
                type Item = Result<#xsd_parser::quick_xml::Event<'ser>, #xsd_parser::quick_xml::Error>;

                #fn_next
            }
        };

        (code, serializer_code)
    }

    /* State Type */

    fn render_serializer_state_type(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let state_ident = &self.serializer_state_ident;

        let variants = self
            .elements
            .iter()
            .map(|f| {
                let target_type = &f.target_type;
                let variant_ident = &f.variant_ident;
                let is_complex = f.target_is_complex;

                let serializer = match f.occurs {
                    Occurs::None => unreachable!(),
                    Occurs::Single if is_complex => quote!(<#target_type as #xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
                    Occurs::Single => quote!(#xsd_parser::quick_xml::ContentSerializer<'ser, #target_type>),
                    Occurs::Optional => quote!(#xsd_parser::quick_xml::IterSerializer<'ser, Option<#target_type>, #target_type>),
                    Occurs::DynamicList => quote!(#xsd_parser::quick_xml::IterSerializer<'ser, Vec<#target_type>, #target_type>),
                    Occurs::StaticList(sz) => quote!(#xsd_parser::quick_xml::IterSerializer<'ser, [#target_type; #sz], #target_type>),
                };

                quote! {
                    #variant_ident(#serializer),
                }
            });

        let need_end_state = self.is_static_complex && !self.elements.is_empty();
        let state_end = need_end_state.then(|| quote!(End__,));

        quote! {
            #[derive(Debug)]
            enum #state_ident<'ser> {
                Init__,
                #( #variants )*
                #state_end
                Done__,
                Phantom__(&'ser ()),
            }
        }
    }

    /* fn next */

    #[allow(clippy::too_many_lines)]
    fn render_serializer_fn_next(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let type_ident = &self.type_ident;
        let state_ident = &self.serializer_state_ident;

        let need_end_state = self.is_static_complex && !self.elements.is_empty();

        let end_state = if need_end_state {
            quote!(#state_ident::End__)
        } else {
            quote!(#state_ident::Done__)
        };

        let (handle_state_init, handle_state_variants) = match self.target_mode {
            TypeMode::Choice => self.render_serializer_fn_next_enum(&end_state),
            TypeMode::All | TypeMode::Sequence => self.render_serializer_fn_next_struct(&end_state),
        };

        let xmlns = self
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

        let has_attributes = !self.attributes.is_empty();
        let attributes = self.attributes.iter().map(|attrib| {
            let attrib_name = &attrib.tag_name;
            let field_ident = &attrib.field_ident;

            let serialize_attrib = if attrib.is_option {
                quote! {
                    value.#field_ident.as_ref().map(SerializeBytes::serialize_bytes).transpose()?.flatten()
                }
            } else {
                quote! {
                    SerializeBytes::serialize_bytes(&value.#field_ident)?
                }
            };

            quote! {
                if let Some(val) = #serialize_attrib {
                    bytes.push_attribute((#attrib_name, val));
                }
            }
        });

        let fn_build_attributes = has_attributes.then(|| {
            quote! {
                fn build_attributes<'a>(
                    mut bytes: BytesStart<'a>,
                    value: &'a super::#type_ident
                ) -> Result<BytesStart<'a>, Error> {
                    use #xsd_parser::quick_xml::SerializeBytes;

                    #( #attributes )*

                    Ok(bytes)
                }
            }
        });

        let bytes_ctor = if xmlns.is_empty() {
            quote! {
                let bytes = BytesStart::new(self.name);
            }
        } else {
            quote! {
                let mut bytes = BytesStart::new(self.name);
                if self.is_root {
                    #( #xmlns )*
                }
            }
        };

        let event = if self.elements.is_empty() {
            format_ident!("Empty")
        } else {
            format_ident!("Start")
        };

        let emit_start_event = if !self.is_static_complex {
            None
        } else if has_attributes {
            Some(quote! {
                #bytes_ctor
                match build_attributes(bytes, &self.value) {
                    Ok(bytes) => return Some(Ok(Event::#event(bytes))),
                    Err(error) => {
                        self.state = #state_ident::Done__;

                        return Some(Err(error));
                    }
                }
            })
        } else {
            Some(quote! {
                #bytes_ctor
                return Some(Ok(Event::#event(bytes)))
            })
        };

        let handle_state_end = need_end_state.then(|| {
            quote! {
                #state_ident::End__ => {
                    self.state = #state_ident::Done__;

                    return Some(Ok(Event::End(BytesEnd::new(self.name))));
                }
            }
        });

        quote! {
            fn next(&mut self) -> Option<Self::Item> {
                use #xsd_parser::quick_xml::{Event, Serializer, WithSerializer, BytesEnd, BytesStart, Error};

                #fn_build_attributes

                loop {
                    match &mut self.state {
                        #state_ident::Init__ => {
                            #handle_state_init
                            #emit_start_event
                        }
                        #handle_state_variants
                        #handle_state_end
                        #state_ident::Done__ => return None,
                        #state_ident::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
    }

    fn render_serializer_fn_next_enum(
        &self,
        state_end: &TokenStream,
    ) -> (TokenStream, TokenStream) {
        let state_ident = &self.serializer_state_ident;
        let content_ident = &self.content_ident;

        let content_field = if self.flatten_content {
            quote!(&self.value)
        } else {
            quote!(&self.value.content)
        };

        let variants_init = self.elements.iter().map(|element| {
            let variant_ident = &element.variant_ident;

            let init = self.render_serializer_fn_next_init_serializer(element, &quote!(x));

            quote! {
                #content_ident::#variant_ident(x) => #init
            }
        });

        let variants_state = self.elements.iter().map(|element| {
            let variant_ident = &element.variant_ident;

            quote! {
                #state_ident::#variant_ident(x) => {
                    match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = #state_ident::Done__;
                            return Some(Err(error))
                        }
                        None => self.state = #state_end,
                    }
                }
            }
        });

        let handle_state_init = quote! {
            match #content_field {
                #( #variants_init )*
            }
        };

        let handle_state_variants = quote! {
            #( #variants_state )*
        };

        (handle_state_init, handle_state_variants)
    }

    fn render_serializer_fn_next_struct(
        &self,
        state_end: &TokenStream,
    ) -> (TokenStream, TokenStream) {
        let state_ident = &self.serializer_state_ident;

        let variants_state = (0..).take(self.elements.len()).map(|i| {
            let element = &self.elements[i];
            let variant_ident = &element.variant_ident;

            let next = if let Some(next) = self.elements.get(i + 1) {
                let field_ident = &next.field_ident;

                self.render_serializer_fn_next_init_serializer(
                    next,
                    &quote!(&self.value.#field_ident),
                )
            } else {
                quote! {
                    self.state = #state_end,
                }
            };

            quote! {
                #state_ident::#variant_ident(x) => match x.next() {
                    Some(Ok(event)) => return Some(Ok(event)),
                    Some(Err(error)) => {
                        self.state = #state_ident::Done__;
                        return Some(Err(error))
                    }
                    None => #next
                }
            }
        });

        let handle_state_init = if let Some(first) = self.elements.first() {
            let field_ident = &first.field_ident;

            self.render_serializer_fn_next_init_serializer(first, &quote!(&self.value.#field_ident))
        } else {
            quote! {
                self.state = #state_end;
            }
        };

        let handle_state_variants = quote! {
            #( #variants_state )*
        };

        (handle_state_init, handle_state_variants)
    }

    fn render_serializer_fn_next_init_serializer(
        &self,
        element: &ElementImpl<'_, '_>,
        value: &TokenStream,
    ) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let state_ident = &self.serializer_state_ident;

        let field_name = &element.tag_name;
        let is_complex = element.target_is_complex;
        let variant_ident = &element.variant_ident;

        match element.occurs {
            Occurs::None => unreachable!(),
            Occurs::Single if is_complex => quote! {
                match WithSerializer::serializer(#value, Some(#field_name), false) {
                    Ok(serializer) => self.state = #state_ident::#variant_ident(serializer),
                    Err(error) => {
                        self.state = #state_ident::Done__;

                        return Some(Err(error));
                    }
                }
            },
            Occurs::Single => quote! {
                match #xsd_parser::quick_xml::ContentSerializer::new(#value, Some(#field_name), false) {
                    Ok(serializer) => self.state = #state_ident::#variant_ident(serializer),
                    Err(error) => {
                        self.state = #state_ident::Done__;

                        return Some(Err(error));
                    }
                }
            },
            Occurs::Optional | Occurs::DynamicList | Occurs::StaticList(_) => quote! {{
                self.state = #state_ident::#variant_ident(
                    #xsd_parser::quick_xml::IterSerializer::new(
                        #value,
                        Some(#field_name),
                        false
                    )
                );
            }},
        }
    }
}

/* AttributeImpl */

impl AttributeImpl<'_, '_> {}

/* ElementImpl */

impl ElementImpl<'_, '_> {}
