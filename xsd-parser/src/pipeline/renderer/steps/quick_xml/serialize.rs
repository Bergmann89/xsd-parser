use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote};

use xsd_parser_types::misc::Namespace;

use crate::config::{GeneratorFlags, TypedefMode};
use crate::models::{
    code::IdentPath,
    data::{
        ComplexBase, ComplexData, ComplexDataContent, ComplexDataElement, ComplexDataEnum,
        ComplexDataStruct, DataTypeVariant, DynamicData, EnumerationData, EnumerationTypeVariant,
        Occurs, ReferenceData, SimpleData, UnionData, UnionTypeVariant,
    },
};

use super::super::super::{
    context::{Context, ValueKey},
    MetaData, RenderStep, RenderStepType,
};

/// Implements a [`RenderStep`] that renders the code for the `quick_xml` serialization.
#[derive(Debug, Clone)]
pub struct QuickXmlSerializeRenderStep {
    /// Whether to add namespaces to the root element during serialization or not.
    pub with_namespaces: bool,

    /// Default namespace to use for the serialization.
    pub default_namespace: Option<Namespace>,
}

macro_rules! resolve_build_in {
    ($ctx:ident, $path:expr) => {
        $ctx.resolve_build_in($path)
    };
}

macro_rules! resolve_ident {
    ($ctx:ident, $path:expr) => {
        $ctx.resolve_ident_path($path)
    };
}

macro_rules! resolve_quick_xml_ident {
    ($ctx:ident, $path:expr) => {
        $ctx.resolve_quick_xml_serialize_ident_path($path)
    };
}

struct SerializerConfig;

impl ValueKey for SerializerConfig {
    type Type = QuickXmlSerializeRenderStep;
}

impl RenderStep for QuickXmlSerializeRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::ExtraImpls
    }

    fn initialize(&mut self, meta: &mut MetaData<'_>) {
        let ident = IdentPath::from_parts(
            [meta.xsd_parser_types.clone(), format_ident!("quick_xml")],
            format_ident!("WithBoxedSerializer"),
        );

        if !meta.dyn_type_traits.contains(&ident) {
            meta.dyn_type_traits.push(ident);
        }
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        ctx.set::<SerializerConfig>(self.clone());

        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) | DataTypeVariant::Custom(_) => (),
            DataTypeVariant::Union(x) => x.render_serializer(ctx),
            DataTypeVariant::Dynamic(x) => x.render_serializer(ctx),
            DataTypeVariant::Reference(x) => x.render_serializer(ctx),
            DataTypeVariant::Enumeration(x) => x.render_serializer(ctx),
            DataTypeVariant::Simple(x) => x.render_serializer(ctx),
            DataTypeVariant::Complex(x) => x.render_serializer(ctx),
        }

        ctx.unset::<SerializerConfig>();
    }
}

/* UnionData */

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

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");
        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serialize_bytes = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeBytes");

        let code = quote! {
            impl #serialize_bytes for #type_ident {
                fn serialize_bytes(&self) -> #result<#option<#cow<'_, str>>, #error> {
                    match self {
                        #( #variants )*
                    }
                }
            }
        };

        ctx.current_module().append(code);
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

/* DynamicData */

impl DynamicData<'_> {
    pub(crate) fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        let Self { type_ident, .. } = self;

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let with_serializer = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::WithSerializer");
        let boxed_serializer =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::BoxedSerializer");

        let code = quote! {
            impl #with_serializer for #type_ident {
                type Serializer<'x> = #boxed_serializer<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: #option<&'ser str>,
                    is_root: bool
                ) -> #result<Self::Serializer<'ser>, #error> {
                    let _name = name;

                    self.0.serializer(None, is_root)
                }
            }
        };

        ctx.current_module().append(code);
    }
}

/* ReferenceData */

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
                let string = resolve_build_in!(ctx, "::alloc::string::String");

                let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");

                quote! {
                    if self.0.is_empty() {
                        return Ok(None);
                    }

                    let mut data = #string::new();
                    for item in &self.0 {
                        if let Some(bytes) = item.serialize_bytes()? {
                            if !data.is_empty() {
                                data.push(' ');
                            }

                            data.push_str(&bytes);
                        }
                    }

                    Ok(Some(#cow::Owned(data)))
                }
            }
        };

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");
        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serialize_bytes = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeBytes");

        let code = quote! {
            impl #serialize_bytes for #type_ident {
                fn serialize_bytes(&self) -> #result<#option<#cow<'_, str>>, #error> {
                    #body
                }
            }
        };

        ctx.current_module().append(code);
    }
}

/* EnumerationData */

impl EnumerationData<'_> {
    pub(crate) fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            ..
        } = self;

        let variants = variants.iter().map(|x| x.render_serializer_variant(ctx));

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");
        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serialize_bytes = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeBytes");

        let code = quote! {
            impl #serialize_bytes for #type_ident {
                fn serialize_bytes(&self) -> #result<#option<#cow<'_, str>>, #error> {
                    match self {
                        #( #variants )*
                    }
                }
            }
        };

        ctx.current_module().append(code);
    }
}

impl EnumerationTypeVariant<'_> {
    fn render_serializer_variant(&self, ctx: &Context<'_, '_>) -> TokenStream {
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
            let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");

            quote! {
                Self::#variant_ident => Ok(Some(#cow::Borrowed(#s_name))),
            }
        }
    }
}

/* SimpleData */

impl SimpleData<'_> {
    pub(crate) fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        let Self { type_ident, .. } = self;

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");
        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serialize_bytes = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeBytes");

        let body = if let Some(digits) = self.meta.constrains.fraction_digits {
            let format = format!("{{inner:.0{digits}}}");

            quote! {
                let Self(inner) = self;

                Ok(Some(#cow::Owned(format!(#format))))
            }
        } else if self.meta.is_list {
            let string = resolve_build_in!(ctx, "::alloc::string::String");

            quote! {
                if self.0.is_empty() {
                    return Ok(None);
                }

                let mut data = #string::new();
                for item in &self.0 {
                    if let Some(bytes) = item.serialize_bytes()? {
                        if !data.is_empty() {
                            data.push(' ');
                        }

                        data.push_str(&bytes);
                    }
                }

                Ok(Some(#cow::Owned(data)))
            }
        } else {
            quote! {
                self.0.serialize_bytes()
            }
        };

        let code = quote! {
            impl #serialize_bytes for #type_ident {
                fn serialize_bytes(&self) -> #result<#option<#cow<'_, str>>, #error> {
                    #body
                }
            }
        };

        ctx.current_module().append(code);
    }
}

/* ComplexData */

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

impl ComplexBase<'_> {
    fn render_with_serializer(&self, ctx: &mut Context<'_, '_>, forward_root: bool) {
        let Self {
            type_ident,
            serializer_ident,
            ..
        } = self;

        let body = if let Some(tag_name) = &self.element_tag() {
            let config = ctx.get_ref::<SerializerConfig>();
            let tag_name = tag_name.get_for_default_namespace(&config.default_namespace);

            self.render_with_serializer_for_element(ctx, &tag_name)
        } else {
            self.render_with_serializer_for_content(ctx, forward_root)
        };

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let with_serializer = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::WithSerializer");

        let code = quote! {
            impl #with_serializer for #type_ident {
                type Serializer<'x> = quick_xml_serialize::#serializer_ident<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: #option<&'ser str>,
                    is_root: bool
                ) -> #result<Self::Serializer<'ser>, #error> {
                    #body
                }
            }
        };

        ctx.current_module().append(code);
    }

    fn render_with_serializer_for_element(
        &self,
        ctx: &Context<'_, '_>,
        tag_name: &str,
    ) -> TokenStream {
        let Self {
            serializer_ident,
            serializer_state_ident,
            ..
        } = self;

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        quote! {
            Ok(quick_xml_serialize::#serializer_ident {
                value: self,
                state: #box_::new(quick_xml_serialize::#serializer_state_ident::Init__),
                name: name.unwrap_or(#tag_name),
                is_root,
            })
        }
    }

    fn render_with_serializer_for_content(
        &self,
        ctx: &Context<'_, '_>,
        forward_root: bool,
    ) -> TokenStream {
        let Self {
            serializer_ident,
            serializer_state_ident,
            ..
        } = self;

        let drop_root = (!forward_root).then(|| quote!(let _is_root = is_root;));
        let forward_root = forward_root.then(|| quote!(is_root,));

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        quote! {
            let _name = name;
            #drop_root

            Ok(quick_xml_serialize::#serializer_ident {
                value: self,
                state: #box_::new(quick_xml_serialize::#serializer_state_ident::Init__),
                #forward_root
            })
        }
    }

    fn render_serializer_type(&self, ctx: &mut Context<'_, '_>, forward_root: bool) {
        let Self {
            type_ident,
            serializer_ident,
            serializer_state_ident,
            ..
        } = self;

        let name = self.represents_element().then(|| {
            quote! {
                pub(super) name: &'ser str,
            }
        });
        let is_root = forward_root.then(|| {
            quote! {
                pub(super) is_root: bool,
            }
        });

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        let code = quote! {
            #[derive(Debug)]
            pub struct #serializer_ident<'ser> {
                pub(super) value: &'ser super::#type_ident,
                pub(super) state: #box_<#serializer_state_ident<'ser>>,
                #name
                #is_root
            }
        };

        ctx.quick_xml_serialize().append(code);
    }

    fn render_serializer_handle_state_end(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let serializer_state_ident = &self.serializer_state_ident;

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let bytes_end = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::BytesEnd");

        quote! {
            #serializer_state_ident::End__ => {
                *self.state = #serializer_state_ident::Done__;

                return Ok(Some(
                    #event::End(
                        #bytes_end::new(self.name))
                    )
                );
            }
        }
    }

    fn render_serializer_xmlns(&self, ctx: &Context<'_, '_>) -> Vec<TokenStream> {
        let _self = self;

        let config = ctx.get_ref::<SerializerConfig>();
        if !config.with_namespaces {
            return Vec::new();
        }

        let xsi = ctx
            .check_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT)
            .then(|| {
                let namespace =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::misc::Namespace");

                quote!(bytes.push_attribute((&b"xmlns:xsi"[..], &#namespace::XSI[..]));)
            });

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
                    if matches!((&config.default_namespace, &module.namespace), (a, b) if a == b) {
                        b"xmlns"
                    } else {
                        buffer = format!("xmlns:{prefix}");
                        buffer.as_bytes()
                    }
                } else {
                    b"xmlns"
                };
                let xmlns = Literal::byte_string(xmlns);

                Some(quote! {
                    bytes.push_attribute((&#xmlns[..], &#ns_const[..]));
                })
            })
            .chain(xsi)
            .collect::<Vec<_>>()
    }
}

impl ComplexDataEnum<'_> {
    fn serializer_need_end_state(&self) -> bool {
        self.represents_element()
    }

    fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        self.render_with_serializer(ctx, !self.is_content);
        self.render_serializer_type(ctx, !self.is_content);
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
            let init = element.render_serializer_enum_state_init(
                ctx,
                &self.serializer_state_ident,
                !self.is_content,
            );

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

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");
        let iterator = resolve_build_in!(ctx, "::core::iter::Iterator");

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");

        let code = quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self) -> #result<#option<#event<'ser>>, #error> {
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

            impl<'ser> #iterator for #serializer_ident<'ser> {
                type Item = #result<#event<'ser>, #error>;

                fn next(&mut self) -> #option<Self::Item> {
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

        ctx.quick_xml_serialize().append(code);
    }

    fn render_serializer_impl_start_event(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let bytes_start =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::BytesStart");

        let xmlns = self.render_serializer_xmlns(ctx);
        let bytes_ctor = if xmlns.is_empty() {
            quote! {
                let bytes = #bytes_start::new(self.name);
            }
        } else {
            quote! {
                let mut bytes = #bytes_start::new(self.name);
                if self.is_root {
                    #( #xmlns )*
                }
            }
        };

        quote! {
            #bytes_ctor
            return Ok(Some(#event::Start(bytes)))
        }
    }
}

impl ComplexDataStruct<'_> {
    fn serializer_need_end_state(&self) -> bool {
        self.represents_element() && self.has_content()
    }

    fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        self.render_with_serializer(ctx, self.represents_element());
        self.render_serializer_type(ctx, self.represents_element());
        self.render_serializer_state_type(ctx);
        self.render_serializer_impl(ctx);
    }

    fn render_serializer_state_type(&self, ctx: &mut Context<'_, '_>) {
        let state_ident = &self.serializer_state_ident;

        let state_variants = self
            .elements()
            .iter()
            .map(|x| x.render_serializer_state_variant(ctx));
        let state_content = self
            .content()
            .and_then(|x| x.render_serializer_state_variant(ctx));
        let state_end = self.serializer_need_end_state().then(|| {
            quote! {
                End__,
            }
        });

        let code = quote! {
            #[derive(Debug)]
            pub(super) enum #state_ident<'ser> {
                Init__,
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
        let handle_state_init = if let Some(first) = elements.first() {
            let init = first.render_serializer_struct_state_init(ctx, serializer_state_ident);

            quote!(#init;)
        } else if let Some(content) = &self.content() {
            let init = content.render_serializer_state_init(ctx, serializer_state_ident);

            quote!(#init;)
        } else {
            quote!(*self.state = #final_state;)
        };

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

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");
        let iterator = resolve_build_in!(ctx, "::core::iter::Iterator");

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");

        let code = quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self) -> #result<#option<#event<'ser>>, #error>
                {
                    loop {
                        match &mut *self.state {
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

            impl<'ser> #iterator for #serializer_ident<'ser> {
                type Item = #result<#event<'ser>, #error>;

                fn next(&mut self) -> #option<Self::Item> {
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

        ctx.quick_xml_serialize().append(code);
    }

    fn render_serializer_impl_start_event(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xmlns = self.render_serializer_xmlns(ctx);
        let attributes = self.attributes.iter().map(|attrib| {
            let attrib_name = attrib.tag_name.get(true);
            let field_ident = &attrib.ident;

            if attrib.meta.is_any() {
                quote! {
                    bytes.extend_attributes(self.value.#field_ident.attributes());
                }
            } else if attrib.is_option {
                let write_attrib_opt = resolve_quick_xml_ident!(
                    ctx,
                    "::xsd_parser_types::quick_xml::write_attrib_opt"
                );

                quote! {
                    #write_attrib_opt(&mut bytes, #attrib_name, &self.value.#field_ident)?;
                }
            } else {
                let write_attrib =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::write_attrib");

                quote! {
                    #write_attrib(&mut bytes, #attrib_name, &self.value.#field_ident)?;
                }
            }
        });

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let bytes_start =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::BytesStart");

        let bytes_mut = self.has_attributes().then(|| quote!(mut));
        let bytes_ctor = if xmlns.is_empty() {
            quote! {
                let #bytes_mut bytes = #bytes_start::new(self.name);
            }
        } else {
            quote! {
                let mut bytes = #bytes_start::new(self.name);
                if self.is_root {
                    #( #xmlns )*
                }
            }
        };

        let variant = if self.has_content() {
            format_ident!("Start")
        } else {
            format_ident!("Empty")
        };

        quote! {
            #bytes_ctor
            #( #attributes )*
            return Ok(Some(#event::#variant(bytes)))
        }
    }
}

impl ComplexDataContent<'_> {
    fn render_serializer_state_variant(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let serializer = self.occurs.make_serializer_type(
            ctx,
            &ctx.resolve_type_for_serialize_module(&self.target_type),
            false,
        )?;

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
                let with_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithSerializer");

                quote! {
                    *self.state = #state_ident::Content__(
                        #with_serializer::serializer(&self.value.content, None, false)?
                    )
                }
            }
            Occurs::Optional => {
                let iter_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::IterSerializer");

                quote! {
                    *self.state = #state_ident::Content__(
                        #iter_serializer::new(
                            self.value.content.as_ref(),
                            None,
                            false
                        )
                    )
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => {
                let iter_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::IterSerializer");

                quote! {
                    *self.state = #state_ident::Content__(
                        #iter_serializer::new(
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
        let serializer = self
            .occurs
            .make_serializer_type(ctx, &target_type, self.need_indirection);

        quote! {
            #variant_ident(#serializer),
        }
    }

    fn render_serializer_enum_state_init(
        &self,
        ctx: &Context<'_, '_>,
        state_ident: &Ident2,
        forward_root: bool,
    ) -> TokenStream {
        let value = match self.occurs {
            Occurs::None => unreachable!(),
            Occurs::Single if self.need_indirection => quote!(&**x),
            Occurs::Single => quote!(x),
            Occurs::Optional if self.need_indirection => quote!(x.as_deref()),
            Occurs::Optional => quote!(x.as_ref()),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(&x[..]),
        };

        self.render_serializer_state_init(ctx, state_ident, &value, forward_root)
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
                quote!(self.value.#field_ident.as_deref())
            }
            Occurs::Optional => quote!(self.value.#field_ident.as_ref()),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(&self.value.#field_ident[..]),
        };

        self.render_serializer_state_init(ctx, state_ident, &value, false)
    }

    fn render_serializer_state_init(
        &self,
        ctx: &Context<'_, '_>,
        state_ident: &Ident2,
        value: &TokenStream,
        forward_root: bool,
    ) -> TokenStream {
        let config = ctx.get_ref::<SerializerConfig>();
        let field_name = self
            .tag_name
            .get_for_default_namespace(&config.default_namespace);
        let variant_ident = &self.variant_ident;

        let is_root = if forward_root {
            quote!(self.is_root)
        } else {
            quote!(false)
        };

        let element_name = self
            .meta()
            .is_any()
            .then(|| quote!(None))
            .unwrap_or_else(|| quote!(Some(#field_name)));

        match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => {
                let with_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithSerializer");

                quote! {
                    *self.state = #state_ident::#variant_ident(
                        #with_serializer::serializer(#value, #element_name, #is_root)?
                    )
                }
            }
            Occurs::StaticList(_) if self.need_indirection => {
                let deref_iter =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DerefIter");
                let iter_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::IterSerializer");

                quote! {
                    *self.state = #state_ident::#variant_ident(
                        #iter_serializer::new(
                            #deref_iter::new(#value),
                            #element_name,
                            #is_root
                        )
                    )
                }
            }
            Occurs::Optional | Occurs::DynamicList | Occurs::StaticList(_) => {
                let iter_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::IterSerializer");

                quote! {
                    *self.state = #state_ident::#variant_ident(
                        #iter_serializer::new(
                            #value,
                            #element_name,
                            #is_root
                        )
                    )
                }
            }
        }
    }
}

impl Occurs {
    fn make_serializer_type(
        &self,
        ctx: &Context<'_, '_>,
        target_type: &TokenStream,
        need_indirection: bool,
    ) -> Option<TokenStream> {
        match self {
            Occurs::None => None,
            Occurs::Single => {
                let with_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithSerializer");

                Some(quote!(<#target_type as #with_serializer>::Serializer<'ser>))
            }
            Occurs::Optional => {
                let option = resolve_build_in!(ctx, "::core::option::Option");

                let iter_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::IterSerializer");

                Some(quote!(#iter_serializer<'ser, #option<&'ser #target_type>, #target_type>))
            }
            Occurs::StaticList(..) if need_indirection => {
                let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

                let deref_iter =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DerefIter");
                let iter_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::IterSerializer");

                Some(
                    quote!(#iter_serializer<'ser, #deref_iter<&'ser [#box_<#target_type>]>, #target_type>),
                )
            }
            Occurs::DynamicList | Occurs::StaticList(..) => {
                let iter_serializer =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::IterSerializer");

                Some(quote!(#iter_serializer<'ser, &'ser [#target_type], #target_type>))
            }
        }
    }
}
