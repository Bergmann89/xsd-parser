use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote};
use tracing::instrument;

use crate::schema::Namespace;

use super::super::super::data::{
    ComplexTypeData, DynamicData, EnumVariantData, EnumerationData, ReferenceData, UnionData,
    UnionVariantData,
};
use super::super::super::misc::{Occurs, StateFlags, TypeMode, TypedefMode};
use super::super::super::Generator;
use super::{
    ComplexTypeImpl, ComplexTypeImplBase, ComplexTypeImplComplex, ComplexTypeImplSimple,
    DynamicTypeImpl, ElementImpl, QuickXmlRenderer,
};

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
            .map(UnionVariantData::render_serializer_variant)
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
        let type_ref = data.current_type_ref_mut();
        if type_ref.add_flag_checked(StateFlags::HAS_QUICK_XML_SERIALIZE) {
            return;
        }

        let code = DynamicTypeImpl::new(data).render_serializer();

        data.add_code(code);
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
                                data.push(' ');
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
        let variants = variants
            .iter()
            .map(EnumVariantData::render_serializer_variant);

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

/* DynamicTypeImpl */

impl DynamicTypeImpl<'_, '_, '_> {
    fn render_serializer(&mut self) -> TokenStream {
        let type_ident = self.type_ident;
        let xsd_parser = &self.xsd_parser_crate;

        quote! {
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
        }
    }
}

/* ComplexTypeImpl */

impl ComplexTypeImpl<'_, '_, '_> {
    fn render_serializer(&mut self) -> (TokenStream, TokenStream) {
        let with_serializer = self.render_with_serializer();
        let serializer_types = self.render_serializer_types();
        let serializer_impls = self.render_serializer_impls();

        let code = quote! {
            #with_serializer
        };

        let serializer_code = quote! {
            #serializer_types
            #serializer_impls

        };

        (code, serializer_code)
    }

    fn render_with_serializer(&self) -> TokenStream {
        match self {
            ComplexTypeImpl::Simple { simple } => simple.render_with_serializer(),
            ComplexTypeImpl::Complex { complex } => complex.render_with_serializer(),
            ComplexTypeImpl::ComplexWithContent { complex, content } => {
                let complex = complex.render_with_serializer();
                let content = content.render_with_serializer();

                quote! {
                    #complex
                    #content
                }
            }
        }
    }

    fn render_serializer_types(&self) -> TokenStream {
        match self {
            ComplexTypeImpl::Simple { simple } => {
                let serializer = simple.render_serializer_type();
                let state = simple.render_serializer_state_type();

                quote! {
                    #serializer
                    #state
                }
            }
            ComplexTypeImpl::Complex { complex } => {
                let serializer = complex.render_serializer_type();
                let state = complex.render_serializer_state_type();

                quote! {
                    #serializer
                    #state
                }
            }
            ComplexTypeImpl::ComplexWithContent { complex, content } => {
                let serializer = complex.render_serializer_type();
                let state = complex.render_serializer_state_type();
                let content_serializer = content.render_serializer_type();
                let content_state = content.render_serializer_state_type();

                quote! {
                    #serializer
                    #state
                    #content_serializer
                    #content_state
                }
            }
        }
    }

    fn render_serializer_impls(&self) -> TokenStream {
        match self {
            ComplexTypeImpl::Simple { simple } => simple.render_serializer_iterator_impl(),
            ComplexTypeImpl::Complex { complex } => complex.render_serializer_iterator_impl(),
            ComplexTypeImpl::ComplexWithContent { complex, content } => {
                let complex = complex.render_serializer_iterator_impl();
                let content = content.render_serializer_iterator_impl();

                quote! {
                    #complex
                    #content
                }
            }
        }
    }
}

/* UnionVariantData */

impl UnionVariantData<'_> {
    fn render_serializer_variant(&self) -> TokenStream {
        let UnionVariantData { variant_ident, .. } = self;

        quote! {
            Self::#variant_ident(x) => x.serialize_bytes(),
        }
    }
}

/* EnumVariantData */

impl EnumVariantData<'_> {
    fn render_serializer_variant(&self) -> TokenStream {
        let EnumVariantData {
            var,
            target_type,
            variant_ident,
        } = self;

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
    }
}

/* ComplexTypeImplBase */

impl ComplexTypeImplBase<'_, '_, '_> {
    fn render_state_init_enum(&self, elements: &[ElementImpl<'_, '_>]) -> TokenStream {
        let variants = elements.iter().map(|element| {
            let type_ident = &self.type_ident;
            let variant_ident = &element.variant_ident;
            let init = element.render_serializer_state_init(
                self,
                &self.serializer_state_ident,
                &quote!(x),
            );

            quote! {
                #type_ident::#variant_ident(x) => #init,
            }
        });

        quote! {
            match self.value {
                #( #variants )*
            }
        }
    }

    fn render_state_init_struct(
        &self,
        final_state: &TokenStream,
        elements: &[ElementImpl<'_, '_>],
    ) -> TokenStream {
        if let Some(first) = elements.first() {
            let field_ident = &first.field_ident;
            let init = first.render_serializer_state_init(
                self,
                &self.serializer_state_ident,
                &quote!(&self.value.#field_ident),
            );

            quote!(#init;)
        } else {
            quote! {
                self.state = #final_state;
            }
        }
    }

    fn render_state_init_simple(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let state_ident = &self.serializer_state_ident;

        quote! {
            self.state = #state_ident::Content__(
                #xsd_parser::quick_xml::ContentSerializer::new(&self.value.content, None, false)
            );
        }
    }

    fn render_state_init_content(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let state_ident = &self.serializer_state_ident;

        match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => quote! {
                self.state = #state_ident::Content__(
                    #xsd_parser::quick_xml::WithSerializer::serializer(&self.value.content, None, false)?
                );
            },
            Occurs::Optional | Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                self.state = #state_ident::Content__(
                    #xsd_parser::quick_xml::IterSerializer::new(
                        &self.value.content,
                        None,
                        false
                    )
                );
            },
        }
    }

    fn render_state_handler_enum(
        &self,
        final_state: &TokenStream,
        elements: &[ElementImpl<'_, '_>],
    ) -> TokenStream {
        let state_ident = &self.serializer_state_ident;

        let variants = elements.iter().map(|element| {
            let variant_ident = &element.variant_ident;

            quote! {
                #state_ident::#variant_ident(x) => {
                    match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = #final_state,
                    }
                }
            }
        });

        quote! {
            #( #variants )*
        }
    }

    fn render_state_handler_struct(
        &self,
        final_state: &TokenStream,
        elements: &[ElementImpl<'_, '_>],
    ) -> TokenStream {
        let state_ident = &self.serializer_state_ident;

        let variants = (0..).take(self.elements.len()).map(|i| {
            let element = &elements[i];
            let variant_ident = &element.variant_ident;

            let next = if let Some(next) = elements.get(i + 1) {
                let field_ident = &next.field_ident;
                let init = next.render_serializer_state_init(
                    self,
                    &self.serializer_state_ident,
                    &quote!(&self.value.#field_ident),
                );

                quote!(#init,)
            } else {
                quote! {
                    self.state = #final_state,
                }
            };

            quote! {
                #state_ident::#variant_ident(x) => match x.next().transpose()? {
                    Some(event) => return Ok(Some(event)),
                    None => #next
                }
            }
        });

        quote! {
            #( #variants )*
        }
    }

    fn render_state_handler_content(&self, final_state: &TokenStream) -> TokenStream {
        let state_ident = &self.serializer_state_ident;

        quote! {
            #state_ident::Content__(x) => match x.next().transpose()? {
                Some(event) => return Ok(Some(event)),
                None => self.state = #final_state,
            }
        }
    }
}

/* ComplexTypeImplSimple */

impl ComplexTypeImplSimple<'_, '_, '_> {
    fn render_with_serializer(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let type_ident = &self.type_ident;
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        quote! {
            impl #xsd_parser::quick_xml::WithSerializer for #type_ident {
                type Serializer<'x> = quick_xml_serialize::#serializer_ident<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: Option<&'ser str>,
                    is_root: bool
                ) -> Result<Self::Serializer<'ser>, #xsd_parser::quick_xml::Error> {
                    let _name = name;
                    let _is_root = is_root;

                    Ok(quick_xml_serialize::#serializer_ident {
                        value: self,
                        state: quick_xml_serialize::#serializer_state_ident::Init__,
                    })
                }
            }
        }
    }

    fn render_serializer_type(&self) -> TokenStream {
        let type_ident = &self.type_ident;
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        quote! {
            #[derive(Debug)]
            pub struct #serializer_ident<'ser> {
                pub(super) value: &'ser super::#type_ident,
                pub(super) state: #serializer_state_ident<'ser>,
            }
        }
    }

    fn render_serializer_state_type(&self) -> TokenStream {
        let state_ident = &self.serializer_state_ident;

        let variants = self
            .elements
            .iter()
            .map(|f| f.render_serializer_state_variant(self));

        quote! {
            #[derive(Debug)]
            pub(super) enum #state_ident<'ser> {
                Init__,
                #( #variants )*
                Done__,
                Phantom__(&'ser ()),
            }
        }
    }

    fn render_serializer_iterator_impl(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        let final_state = quote!(#serializer_state_ident::Done__);

        let state_init = match self.type_mode {
            TypeMode::Simple => self.render_state_init_simple(),
            TypeMode::Choice => self.render_state_init_enum(&self.elements),
            TypeMode::All | TypeMode::Sequence => {
                self.render_state_init_struct(&final_state, &self.elements)
            }
        };

        let state_variants = match self.type_mode {
            TypeMode::Simple => self.render_state_handler_content(&final_state),
            TypeMode::Choice => self.render_state_handler_enum(&final_state, &self.elements),
            TypeMode::All | TypeMode::Sequence => {
                self.render_state_handler_struct(&final_state, &self.elements)
            }
        };

        quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self) -> Result<Option<#xsd_parser::quick_xml::Event<'ser>>, #xsd_parser::quick_xml::Error>
                {
                    loop {
                        match &mut self.state {
                            #serializer_state_ident::Init__ => { #state_init },
                            #state_variants
                            #serializer_state_ident::Done__ => return Ok(None),
                            #serializer_state_ident::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }

            impl<'ser> core::iter::Iterator for #serializer_ident<'ser> {
                type Item = Result<#xsd_parser::quick_xml::Event<'ser>, #xsd_parser::quick_xml::Error>;

                fn next(&mut self) -> Option<Self::Item> {
                    self.next_event().transpose()
                }
            }
        }
    }
}

/* ComplexTypeImplComplex */

impl ComplexTypeImplComplex<'_, '_, '_> {
    fn render_with_serializer(&self) -> TokenStream {
        let tag_name = &self.tag_name;
        let type_ident = &self.type_ident;
        let xsd_parser = &self.xsd_parser_crate;
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        quote! {
            impl #xsd_parser::quick_xml::WithSerializer for #type_ident {
                type Serializer<'x> = quick_xml_serialize::#serializer_ident<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: Option<&'ser str>,
                    is_root: bool
                ) -> Result<Self::Serializer<'ser>, #xsd_parser::quick_xml::Error> {
                    Ok(quick_xml_serialize::#serializer_ident {
                        name: name.unwrap_or(#tag_name),
                        value: self,
                        is_root,
                        state: quick_xml_serialize::#serializer_state_ident::Init__,
                    })
                }
            }
        }
    }

    fn render_serializer_type(&self) -> TokenStream {
        let type_ident = &self.type_ident;
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        quote! {
            #[derive(Debug)]
            pub struct #serializer_ident<'ser> {
                pub(super) name: &'ser str,
                pub(super) value: &'ser super::#type_ident,
                pub(super) is_root: bool,
                pub(super) state: #serializer_state_ident<'ser>,
            }
        }
    }

    fn render_serializer_state_type(&self) -> TokenStream {
        let state_ident = &self.serializer_state_ident;

        let state_variants = self
            .elements
            .iter()
            .map(|f| f.render_serializer_state_variant(self));
        let state_content = self
            .content_occurs
            .make_serializer_type(&self.xsd_parser_crate, &self.content_type)
            .map(|ty| {
                quote! {
                    Content__(#ty),
                }
            });
        let state_end = self.has_content.then(|| {
            quote! {
                End__,
            }
        });

        quote! {
            #[derive(Debug)]
            pub(super) enum #state_ident<'ser> {
                Init__,
                #( #state_variants )*
                #state_content
                #state_end
                Done__,
                Phantom__(&'ser ()),
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn render_serializer_iterator_impl(&self) -> TokenStream {
        let xsd_parser = &self.xsd_parser_crate;
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        /* Attributes */

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

        let attributes = self.attributes.iter().map(|attrib| {
            let attrib_name = &attrib.tag_name;
            let field_ident = &attrib.field_ident;

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

        /* Start Event */

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

        let event = if self.has_content {
            format_ident!("Start")
        } else {
            format_ident!("Empty")
        };

        let emit_start_event = quote! {
            #bytes_ctor
            #( #attributes )*
            return Ok(Some(#xsd_parser::quick_xml::Event::#event(bytes)))
        };

        /* State Handling */

        let final_state = if self.has_content {
            quote!(#serializer_state_ident::End__)
        } else {
            quote!(#serializer_state_ident::Done__)
        };

        let handle_state_init = match self.type_mode {
            TypeMode::Choice if self.flatten_content => self.render_state_init_enum(&self.elements),
            TypeMode::All | TypeMode::Sequence if self.flatten_content => {
                self.render_state_init_struct(&final_state, &self.elements)
            }
            TypeMode::Simple => self.render_state_init_simple(),
            _ => self.render_state_init_content(),
        };

        let handle_state_variants = match self.type_mode {
            TypeMode::Choice if self.flatten_content => {
                self.render_state_handler_enum(&final_state, &self.elements)
            }
            TypeMode::All | TypeMode::Sequence if self.flatten_content => {
                self.render_state_handler_struct(&final_state, &self.elements)
            }
            _ => self.render_state_handler_content(&final_state),
        };

        let handle_state_end = self.has_content.then(|| {
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

        /* Final Code */

        quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self) -> Result<Option<#xsd_parser::quick_xml::Event<'ser>>, #xsd_parser::quick_xml::Error>
                {
                    loop {
                        match &mut self.state {
                            #serializer_state_ident::Init__ => {
                                #handle_state_init
                                #emit_start_event
                            }
                            #handle_state_variants
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
        }
    }
}

/* ElementImpl */

impl ElementImpl<'_, '_> {
    fn render_serializer_state_variant(&self, generator: &Generator<'_>) -> TokenStream {
        let target_type = &self.target_type;
        let variant_ident = &self.variant_ident;
        let xsd_parser = &generator.xsd_parser_crate;

        let serializer = self.occurs.make_serializer_type(xsd_parser, target_type);

        quote! {
            #variant_ident(#serializer),
        }
    }

    fn render_serializer_state_init(
        &self,
        generator: &Generator<'_>,
        state_ident: &Ident2,
        value: &TokenStream,
    ) -> TokenStream {
        let xsd_parser = &generator.xsd_parser_crate;

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

/* Occurs */

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
