use std::mem::swap;

use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote};

use crate::config::TypedefMode;
use crate::models::code::IdentPath;
use crate::models::{
    data::{
        ComplexBase, ComplexData, ComplexDataContent, ComplexDataElement, ComplexDataEnum,
        ComplexDataStruct, DataTypeVariant, DynamicData, EnumerationData, EnumerationTypeVariant,
        Occurs, ReferenceData, UnionData, UnionTypeVariant,
    },
    schema::Namespace,
};

use super::super::super::{Context, MetaData, RenderStep};

/// Implements a [`RenderStep`] that renders the code for the `quick_xml` serialization.
#[derive(Debug)]
pub struct QuickXmlSerializeRenderStep;

impl RenderStep for QuickXmlSerializeRenderStep {
    fn initialize(&mut self, meta: &mut MetaData<'_>) {
        let ident = IdentPath::from_parts(
            [meta.xsd_parser_crate.clone(), format_ident!("quick_xml")],
            format_ident!("WithBoxedSerializer"),
        );

        if !meta.dyn_type_traits.contains(&ident) {
            meta.dyn_type_traits.push(ident);
        }
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) | DataTypeVariant::Custom(_) => (),
            DataTypeVariant::Union(x) => x.render_serializer(ctx),
            DataTypeVariant::Dynamic(x) => x.render_serializer(ctx),
            DataTypeVariant::Reference(x) => x.render_serializer(ctx),
            DataTypeVariant::Enumeration(x) => x.render_serializer(ctx),
            DataTypeVariant::Complex(x) => x.render_serializer(ctx),
        }
    }
}

/* UnionType */

impl UnionData<'_> {
    pub(crate) fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            ..
        } = self;

        let variants = variants
            .iter()
            .map(UnionTypeVariant::render_serializer_variant)
            .collect::<Vec<_>>();

        let usings = [
            "std::borrow::Cow",
            "xsd_parser::quick_xml::Error",
            "xsd_parser::quick_xml::SerializeBytes",
        ];
        let code = quote! {
            impl SerializeBytes for #type_ident {
                fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
                    match self {
                        #( #variants )*
                    }
                }
            }
        };

        ctx.module().usings(usings).append(code);
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

impl DynamicData<'_> {
    pub(crate) fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        let Self { type_ident, .. } = self;

        let usings = [
            "xsd_parser::quick_xml::Error",
            "xsd_parser::quick_xml::WithSerializer",
            "xsd_parser::quick_xml::BoxedSerializer",
        ];
        let code = quote! {
            impl WithSerializer for #type_ident {
                type Serializer<'x> = BoxedSerializer<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: Option<&'ser str>,
                    is_root: bool
                ) -> Result<Self::Serializer<'ser>, Error> {
                    let _name = name;

                    self.0.serializer(None, is_root)
                }
            }
        };

        ctx.module().usings(usings).append(code);
    }
}

/* ReferenceType */

impl ReferenceData<'_> {
    pub(crate) fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            mode,
            occurs,
            type_ident,
            ..
        } = self;

        if matches!(mode, TypedefMode::Auto | TypedefMode::Typedef) {
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

                    Ok(Some(Cow::Owned(data)))
                }
            }
        };

        let usings = [
            "std::borrow::Cow",
            "xsd_parser::quick_xml::Error",
            "xsd_parser::quick_xml::SerializeBytes",
        ];
        let code = quote! {
            impl SerializeBytes for #type_ident {
                fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
                    #body
                }
            }
        };

        ctx.module().usings(usings).append(code);
    }
}

/* EnumerationType */

impl EnumerationData<'_> {
    pub(crate) fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            ..
        } = self;

        let variants = variants
            .iter()
            .map(EnumerationTypeVariant::render_serializer_variant);

        let usings = [
            "std::borrow::Cow",
            "xsd_parser::quick_xml::Error",
            "xsd_parser::quick_xml::SerializeBytes",
        ];
        let code = quote! {
            impl SerializeBytes for #type_ident {
                fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
                    match self {
                        #( #variants )*
                    }
                }
            }
        };

        ctx.module().usings(usings).append(code);
    }
}

impl EnumerationTypeVariant<'_> {
    fn render_serializer_variant(&self) -> TokenStream {
        let Self {
            s_name,
            target_type,
            variant_ident,
            ..
        } = self;

        if target_type.is_some() {
            quote! {
                Self::#variant_ident(x) => x.serialize_bytes(),
            }
        } else {
            quote! {
                Self::#variant_ident => Ok(Some(Cow::Borrowed(#s_name))),
            }
        }
    }
}

/* ComplexType */

impl ComplexData<'_> {
    pub(crate) fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_serializer(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_serializer(ctx);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_serializer(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_serializer(ctx);
                }
            }
        }
    }
}

impl ComplexBase {
    fn render_with_serializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            serializer_ident,
            ..
        } = self;

        let body = if let Some(tag_name) = &self.element_tag() {
            self.render_with_serializer_for_element(tag_name)
        } else {
            self.render_with_serializer_for_content()
        };

        let usings = [
            "xsd_parser::quick_xml::Error",
            "xsd_parser::quick_xml::WithSerializer",
        ];
        let code = quote! {
            impl WithSerializer for #type_ident {
                type Serializer<'x> = quick_xml_serialize::#serializer_ident<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: Option<&'ser str>,
                    is_root: bool
                ) -> Result<Self::Serializer<'ser>, Error> {
                    #body
                }
            }
        };

        ctx.module().usings(usings).append(code);
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
                state: Box::new(quick_xml_serialize::#serializer_state_ident::Init__),
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
                state: Box::new(quick_xml_serialize::#serializer_state_ident::Init__),
            })
        }
    }

    fn render_serializer_type(&self, ctx: &mut Context<'_, '_>) {
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

        let code = quote! {
            #[derive(Debug)]
            pub struct #serializer_ident<'ser> {
                pub(super) value: &'ser super::#type_ident,
                pub(super) state: Box<#serializer_state_ident<'ser>>,
                #extra
            }
        };

        ctx.quick_xml_serialize().append(code);
    }

    fn render_serializer_handle_state_end(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let serializer_state_ident = &self.serializer_state_ident;

        ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::BytesEnd"]);

        quote! {
            #serializer_state_ident::End__ => {
                *self.state = #serializer_state_ident::Done__;

                return Ok(Some(
                    Event::End(
                        BytesEnd::new(self.name))
                    )
                );
            }
        }
    }

    fn render_serializer_xmlns(&self, ctx: &Context<'_, '_>) -> Vec<TokenStream> {
        let _self = self;

        ctx.types
            .meta
            .types
            .modules
            .values()
            .filter_map(|module| {
                let ns = module.namespace.as_ref()?;
                if *ns == Namespace::XS || *ns == Namespace::XML {
                    return None;
                }

                let ns_const = ctx.resolve_type_for_serialize_module(&module.make_ns_const());

                let buffer;
                let xmlns = if let Some(prefix) = &module.prefix {
                    buffer = format!("xmlns:{prefix}");
                    buffer.as_bytes()
                } else {
                    b"xmlns"
                };
                let xmlns = Literal::byte_string(xmlns);

                Some(quote! {
                    bytes.push_attribute((&#xmlns[..], &#ns_const[..]));
                })
            })
            .collect::<Vec<_>>()
    }
}

impl ComplexDataEnum<'_> {
    fn serializer_need_end_state(&self) -> bool {
        self.represents_element()
    }

    fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        self.render_with_serializer(ctx);
        self.render_serializer_type(ctx);
        self.render_serializer_state_type(ctx);
        self.render_serializer_impl(ctx);
    }

    fn render_serializer_state_type(&self, ctx: &mut Context<'_, '_>) {
        let serializer_state_ident = &self.serializer_state_ident;

        let state_variants = self
            .elements
            .iter()
            .map(|x| x.render_serializer_state_variant(ctx));
        let state_end = self.represents_element().then(|| {
            quote! {
                End__,
            }
        });

        let code = quote! {
            #[derive(Debug)]
            pub(super) enum #serializer_state_ident<'ser> {
                Init__,
                #( #state_variants )*
                #state_end
                Done__,
                Phantom__(&'ser ()),
            }
        };

        ctx.quick_xml_serialize().append(code);
    }

    fn render_serializer_impl(&self, ctx: &mut Context<'_, '_>) {
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        let emit_start_event = self
            .serializer_need_end_state()
            .then(|| self.render_serializer_impl_start_event(ctx));

        let final_state = if self.serializer_need_end_state() {
            quote!(#serializer_state_ident::End__)
        } else {
            quote!(#serializer_state_ident::Done__)
        };

        let variants_init = self.elements.iter().map(|element| {
            let type_ident = &self.type_ident;
            let variant_ident = &element.variant_ident;
            let init = element.render_serializer_enum_state_init(ctx, &self.serializer_state_ident);

            quote! {
                super::#type_ident::#variant_ident(x) => #init,
            }
        });

        let handle_state_init = quote! {
            match self.value {
                #( #variants_init )*
            }
        };

        let handle_state_variants = self.elements.iter().map(|element| {
            let variant_ident = &element.variant_ident;

            quote! {
                #serializer_state_ident::#variant_ident(x) => {
                    match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = #final_state,
                    }
                }
            }
        });

        let handle_state_end = self
            .serializer_need_end_state()
            .then(|| self.render_serializer_handle_state_end(ctx));

        let usings = [
            "core::iter::Iterator",
            "xsd_parser::quick_xml::Event",
            "xsd_parser::quick_xml::Error",
        ];
        let code = quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
                    loop {
                        match &mut *self.state {
                            #serializer_state_ident::Init__ => {
                                #handle_state_init
                                #emit_start_event
                            }
                            #( #handle_state_variants )*
                            #handle_state_end
                            #serializer_state_ident::Done__ => return Ok(None),
                            #serializer_state_ident::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }

            impl<'ser> Iterator for #serializer_ident<'ser> {
                type Item = Result<Event<'ser>, Error>;

                fn next(&mut self) -> Option<Self::Item> {
                    match self.next_event() {
                        Ok(Some(event)) => Some(Ok(event)),
                        Ok(None) => None,
                        Err(error) => {
                            *self.state = #serializer_state_ident::Done__;

                            Some(Err(error))
                        }
                    }
                }
            }
        };

        ctx.quick_xml_serialize().usings(usings).append(code);
    }

    fn render_serializer_impl_start_event(&self, ctx: &Context<'_, '_>) -> TokenStream {
        ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::BytesStart"]);

        let xmlns = self.render_serializer_xmlns(ctx);
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

        quote! {
            #bytes_ctor
            return Ok(Some(Event::Start(bytes)))
        }
    }
}

impl ComplexDataStruct<'_> {
    fn serializer_need_end_state(&self) -> bool {
        self.represents_element() && self.has_content()
    }

    fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        self.render_with_serializer(ctx);
        self.render_serializer_type(ctx);
        self.render_serializer_state_type(ctx);
        self.render_serializer_impl(ctx);
    }

    fn render_serializer_state_type(&self, ctx: &mut Context<'_, '_>) {
        let state_ident = &self.serializer_state_ident;

        let text_before = self.is_mixed.then(|| quote!(TextBefore__,));
        let state_variants = self
            .elements()
            .iter()
            .map(|x| x.render_serializer_state_variant(ctx));
        let state_content = self
            .content()
            .map(|x| x.render_serializer_state_variant(ctx));
        let state_end = self.serializer_need_end_state().then(|| {
            quote! {
                End__,
            }
        });

        let code = quote! {
            #[derive(Debug)]
            pub(super) enum #state_ident<'ser> {
                Init__,
                #text_before
                #( #state_variants )*
                #state_content
                #state_end
                Done__,
                Phantom__(&'ser ()),
            }
        };

        ctx.quick_xml_serialize().append(code);
    }

    #[allow(clippy::too_many_lines)]
    fn render_serializer_impl(&self, ctx: &mut Context<'_, '_>) {
        let serializer_ident = &self.serializer_ident;
        let serializer_state_ident = &self.serializer_state_ident;

        let emit_start_event = self
            .represents_element()
            .then(|| self.render_serializer_impl_start_event(ctx));

        let final_state = if self.serializer_need_end_state() {
            quote!(#serializer_state_ident::End__)
        } else {
            quote!(#serializer_state_ident::Done__)
        };

        let elements = self.elements();
        let mut handle_state_init = if let Some(first) = elements.first() {
            let init = first.render_serializer_struct_state_init(ctx, serializer_state_ident);

            quote!(#init;)
        } else if let Some(content) = &self.content() {
            let init = content.render_serializer_state_init(ctx, serializer_state_ident);

            quote!(#init;)
        } else {
            quote!(*self.state = #final_state;)
        };

        let handle_text_before = self.is_mixed.then(|| {
            let mut init_state = quote! {
                *self.state = #serializer_state_ident::TextBefore__;
            };

            swap(&mut init_state, &mut handle_state_init);
            ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::BytesText"]);

            quote! {
                #serializer_state_ident::TextBefore__ => {
                    #init_state

                    if let Some(text) = &self.value.text_before {
                        return Ok(Some(Event::Text(BytesText::from_escaped(text))));
                    }
                }
            }
        });

        let handle_state_variants = (0..).take(elements.len()).map(|i| {
            let element = &elements[i];
            let variant_ident = &element.variant_ident;

            let next = if let Some(next) = elements.get(i + 1) {
                let init = next.render_serializer_struct_state_init(ctx, serializer_state_ident);

                quote!(#init,)
            } else {
                quote! {
                    *self.state = #final_state,
                }
            };

            quote! {
                #serializer_state_ident::#variant_ident(x) => match x.next().transpose()? {
                    Some(event) => return Ok(Some(event)),
                    None => #next
                }
            }
        });

        let handle_state_content = self.content().map(|_| {
            quote! {
                #serializer_state_ident::Content__(x) => match x.next().transpose()? {
                    Some(event) => return Ok(Some(event)),
                    None => *self.state = #final_state,
                }
            }
        });

        let handle_state_end = self
            .serializer_need_end_state()
            .then(|| self.render_serializer_handle_state_end(ctx));

        let usings = [
            "core::iter::Iterator",
            "xsd_parser::quick_xml::Event",
            "xsd_parser::quick_xml::Error",
        ];
        let code = quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error>
                {
                    loop {
                        match &mut *self.state {
                            #serializer_state_ident::Init__ => {
                                #handle_state_init
                                #emit_start_event
                            }
                            #handle_text_before
                            #( #handle_state_variants )*
                            #handle_state_content
                            #handle_state_end
                            #serializer_state_ident::Done__ => return Ok(None),
                            #serializer_state_ident::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }

            impl<'ser> Iterator for #serializer_ident<'ser> {
                type Item = Result<Event<'ser>, Error>;

                fn next(&mut self) -> Option<Self::Item> {
                    match self.next_event() {
                        Ok(Some(event)) => Some(Ok(event)),
                        Ok(None) => None,
                        Err(error) => {
                            *self.state = #serializer_state_ident::Done__;

                            Some(Err(error))
                        }
                    }
                }
            }
        };

        ctx.quick_xml_serialize().usings(usings).append(code);
    }

    fn render_serializer_impl_start_event(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xmlns = self.render_serializer_xmlns(ctx);
        let attributes = self.attributes.iter().map(|attrib| {
            let attrib_name = &attrib.tag_name;
            let field_ident = &attrib.ident;

            if attrib.meta.is_any() {
                quote! {
                    bytes.extend_attributes(self.value.#field_ident.attributes());
                }
            } else if attrib.is_option {
                ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::write_attrib_opt"]);

                quote! {
                    write_attrib_opt(&mut bytes, #attrib_name, &self.value.#field_ident)?;
                }
            } else {
                ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::write_attrib"]);

                quote! {
                    write_attrib(&mut bytes, #attrib_name, &self.value.#field_ident)?;
                }
            }
        });

        ctx.add_quick_xml_serialize_usings([
            "xsd_parser::quick_xml::Event",
            "xsd_parser::quick_xml::BytesStart",
        ]);

        let bytes_mut = self.has_attributes().then(|| quote!(mut));
        let bytes_ctor = if xmlns.is_empty() {
            quote! {
                let #bytes_mut bytes = BytesStart::new(self.name);
            }
        } else {
            quote! {
                let mut bytes = BytesStart::new(self.name);
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
            return Ok(Some(Event::#event(bytes)))
        }
    }
}

impl ComplexDataContent {
    fn render_serializer_state_variant(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let serializer = self
            .occurs
            .make_serializer_type(&ctx.resolve_type_for_serialize_module(&self.target_type))?;

        Some(quote! {
            Content__(#serializer),
        })
    }

    fn render_serializer_state_init(
        &self,
        ctx: &Context<'_, '_>,
        state_ident: &Ident2,
    ) -> TokenStream {
        match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => {
                ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::WithSerializer"]);

                quote! {
                    *self.state = #state_ident::Content__(
                        WithSerializer::serializer(&self.value.content, None, false)?
                    )
                }
            }
            Occurs::Optional => {
                ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::IterSerializer"]);

                quote! {
                    *self.state = #state_ident::Content__(
                        IterSerializer::new(
                            self.value.content.as_ref(),
                            None,
                            false
                        )
                    )
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => {
                ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::IterSerializer"]);

                quote! {
                    *self.state = #state_ident::Content__(
                        IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false
                        )
                    )
                }
            }
        }
    }
}

impl ComplexDataElement<'_> {
    fn render_serializer_state_variant(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let target_type = ctx.resolve_type_for_serialize_module(&self.target_type);
        let variant_ident = &self.variant_ident;
        let serializer = self.occurs.make_serializer_type(&target_type);

        quote! {
            #variant_ident(#serializer),
        }
    }

    fn render_serializer_enum_state_init(
        &self,
        ctx: &Context<'_, '_>,
        state_ident: &Ident2,
    ) -> TokenStream {
        let value = match self.occurs {
            Occurs::None => unreachable!(),
            Occurs::Single if self.need_indirection => quote!(&**x),
            Occurs::Single => quote!(x),
            Occurs::Optional if self.need_indirection => quote!(x.as_ref().map(|x| &**x)),
            Occurs::Optional => quote!(x.as_ref()),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(&x[..]),
        };

        self.render_serializer_state_init(ctx, state_ident, &value)
    }

    fn render_serializer_struct_state_init(
        &self,
        ctx: &Context<'_, '_>,
        state_ident: &Ident2,
    ) -> TokenStream {
        let field_ident = &self.field_ident;

        let value = match self.occurs {
            Occurs::None => unreachable!(),
            Occurs::Single if self.need_indirection => quote!(&*self.value.#field_ident),
            Occurs::Single => quote!(&self.value.#field_ident),
            Occurs::Optional if self.need_indirection => {
                quote!(self.value.#field_ident.as_ref().map(|x| &**x))
            }
            Occurs::Optional => quote!(self.value.#field_ident.as_ref()),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(&self.value.#field_ident[..]),
        };

        self.render_serializer_state_init(ctx, state_ident, &value)
    }

    fn render_serializer_state_init(
        &self,
        ctx: &Context<'_, '_>,
        state_ident: &Ident2,
        value: &TokenStream,
    ) -> TokenStream {
        let field_name = &self.tag_name;
        let variant_ident = &self.variant_ident;

        match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => {
                ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::WithSerializer"]);

                quote! {
                    *self.state = #state_ident::#variant_ident(
                        WithSerializer::serializer(#value, Some(#field_name), false)?
                    )
                }
            }
            Occurs::Optional | Occurs::DynamicList | Occurs::StaticList(_) => {
                ctx.add_quick_xml_serialize_usings(["xsd_parser::quick_xml::IterSerializer"]);

                quote! {
                    *self.state = #state_ident::#variant_ident(
                        IterSerializer::new(
                            #value,
                            Some(#field_name),
                            false
                        )
                    )
                }
            }
        }
    }
}

impl Occurs {
    fn make_serializer_type(&self, target_type: &TokenStream) -> Option<TokenStream> {
        match self {
            Occurs::None => None,
            Occurs::Single => Some(quote!(<#target_type as WithSerializer>::Serializer<'ser>)),
            Occurs::Optional => {
                Some(quote!(IterSerializer<'ser, Option<&'ser #target_type>, #target_type>))
            }
            Occurs::DynamicList | Occurs::StaticList(..) => {
                Some(quote!(IterSerializer<'ser, &'ser [#target_type], #target_type>))
            }
        }
    }
}
