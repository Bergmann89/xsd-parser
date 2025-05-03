#![allow(clippy::redundant_closure_for_method_calls)]

use std::collections::HashSet;
use std::ops::Not;

use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote};

use crate::{
    config::TypedefMode,
    generator::{
        context::ValueKey,
        data::{
            ComplexType, ComplexTypeAttribute, ComplexTypeBase, ComplexTypeContent,
            ComplexTypeElement, ComplexTypeEnum, ComplexTypeStruct, DerivedType, DynamicType,
            EnumerationType, EnumerationTypeVariant, ReferenceType, StructMode, TypeData,
            UnionType, UnionTypeVariant,
        },
        misc::Occurs,
        renderer::Renderer,
        Context,
    },
    schema::{xs::Use, MaxOccurs},
    types::{ComplexInfo, ElementMode, Ident, TypeVariant, Types},
};

/// Implements a [`Renderer`] that renders the code for the `quick_xml` deserialization.
#[derive(Debug)]
pub struct QuickXmlDeserializeRenderer {
    /// Whether to box the deserializer or not.
    ///
    /// Boxing the deserializer will reduce the stack usage, but may decrease
    /// the performance.
    pub boxed_deserializer: bool,
}

struct BoxedDeserializer;

impl ValueKey for BoxedDeserializer {
    type Type = bool;
}

impl Renderer for QuickXmlDeserializeRenderer {
    fn render_type(&mut self, ctx: &mut Context<'_, '_>, ty: &TypeData<'_>) {
        ctx.set::<BoxedDeserializer>(self.boxed_deserializer);

        match ty {
            TypeData::BuildIn(_) => (),
            TypeData::Union(x) => x.render_deserializer(ctx),
            TypeData::Dynamic(x) => x.render_deserializer(ctx),
            TypeData::Reference(x) => x.render_deserializer(ctx),
            TypeData::Enumeration(x) => x.render_deserializer(ctx),
            TypeData::Complex(x) => x.render_deserializer(ctx),
        }

        ctx.unset::<BoxedDeserializer>();
    }
}

/* UnionType */

impl UnionType<'_> {
    pub(crate) fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            ..
        } = self;

        let xsd_parser = &ctx.xsd_parser_crate;
        let variants = variants
            .iter()
            .map(|var| var.render_deserializer_variant(ctx));

        let usings = [
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::ErrorKind),
            quote!(#xsd_parser::quick_xml::DeserializeBytes),
            quote!(#xsd_parser::quick_xml::XmlReader),
        ];
        let code = quote! {
            impl DeserializeBytes for #type_ident {
                fn deserialize_bytes<R>(
                    reader: &R,
                    bytes: &[u8],
                ) -> Result<Self, Error>
                where
                    R: XmlReader
                {
                    let mut errors = Vec::new();

                    #( #variants )*

                    Err(reader.map_error(ErrorKind::InvalidUnion(errors.into())))
                }
            }
        };

        ctx.module().usings(usings).append(code);
    }
}

impl UnionTypeVariant<'_> {
    fn render_deserializer_variant(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let Self {
            variant_ident,
            target_type,
            ..
        } = self;

        let target_type = ctx.resolve_type_for_module(target_type);

        quote! {
            match #target_type::deserialize_bytes(reader, bytes) {
                Ok(value) => return Ok(Self::#variant_ident(value)),
                Err(error) => errors.push(Box::new(error)),
            }
        }
    }
}

/* DynamicType */

impl DynamicType<'_> {
    pub(crate) fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        self.render_with_deserializer(ctx);
        self.render_deserializer_types(ctx);
        self.render_deserializer_impls(ctx);
    }

    fn render_with_deserializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            deserializer_ident,
            ..
        } = self;
        let xsd_parser = &ctx.xsd_parser_crate;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_type = if boxed_deserializer {
            quote!(Box<quick_xml_deserialize::#deserializer_ident>)
        } else {
            quote!(quick_xml_deserialize::#deserializer_ident)
        };

        let usings = [quote!(#xsd_parser::quick_xml::WithDeserializer)];
        let code = quote! {
            impl WithDeserializer for #type_ident {
                type Deserializer = #deserializer_type;
            }
        };

        ctx.module().usings(usings).append(code);
    }

    fn render_deserializer_types(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            derived_types,
            deserializer_ident,
            ..
        } = self;

        let xsd_parser = &ctx.xsd_parser_crate;

        let variants = derived_types.iter().map(|x| {
            let target_type = ctx.resolve_type_for_deserialize_module(&x.target_type);
            let variant_ident = &x.variant_ident;

            quote! {
                #variant_ident(<#target_type as WithDeserializer>::Deserializer),
            }
        });

        let usings = [quote!(#xsd_parser::quick_xml::WithDeserializer)];
        let code = quote! {
            #[derive(Debug)]
            pub enum #deserializer_ident {
                #( #variants )*
            }
        };

        ctx.quick_xml_deserialize().usings(usings).append(code);
    }

    fn render_deserializer_impls(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            derived_types,
            deserializer_ident,
            ..
        } = self;

        let xsd_parser = &ctx.xsd_parser_crate;

        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_type = if boxed_deserializer {
            quote!(Box<#deserializer_ident>)
        } else {
            quote!(#deserializer_ident)
        };
        let boxed_deserializer_ident =
            boxed_deserializer_ident(boxed_deserializer, deserializer_ident);
        let deref_self = boxed_deserializer.then(|| quote!(*));

        let variants_init = derived_types
            .iter()
            .map(|x| x.render_deserializer_init(ctx, type_ident, deserializer_ident));
        let variants_next = derived_types
            .iter()
            .map(|x| x.render_deserializer_next(ctx, type_ident, deserializer_ident));
        let variants_finish = derived_types.iter().map(|x| {
            let variant_ident = &x.variant_ident;

            quote! {
                #boxed_deserializer_ident::#variant_ident(x) => Ok(super::#type_ident(Box::new(x.finish(reader)?))),
            }
        });

        let usings = [
            quote!(#xsd_parser::quick_xml::Event),
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::Deserializer),
            quote!(#xsd_parser::quick_xml::DeserializerEvent),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
            quote!(#xsd_parser::quick_xml::DeserializerResult),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ];
        let code = quote! {
            impl<'de> Deserializer<'de, super::#type_ident> for #deserializer_type {
                fn init<R>(
                    reader: &R,
                    event: Event<'de>,
                ) -> DeserializerResult<'de, super::#type_ident>
                where
                    R: DeserializeReader,
                {
                    let Some(type_name) = reader.get_dynamic_type_name(&event)? else {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::None,
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    };
                    let type_name = type_name.into_owned();

                    #( #variants_init )*

                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::None,
                        event: DeserializerEvent::Break(event),
                        allow_any: false,
                    })
                }

                fn next<R>(
                    self,
                    reader: &R,
                    event: Event<'de>
                ) -> DeserializerResult<'de, super::#type_ident>
                where
                    R: DeserializeReader
                {
                    #xsd_parser::print_stack_info!();

                    match #deref_self self {
                        #( #variants_next )*
                    }
                }

                fn finish<R>(
                    self,
                    reader: &R
                ) -> Result<super::#type_ident, Error>
                where
                    R: DeserializeReader
                {
                    match #deref_self self {
                        #( #variants_finish )*
                    }
                }
            }
        };

        ctx.quick_xml_deserialize().usings(usings).append(code);
    }
}

impl DerivedType {
    fn render_deserializer_init(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
        deserializer_ident: &Ident2,
    ) -> TokenStream {
        let Self {
            ident,
            b_name,
            target_type,
            variant_ident,
            ..
        } = self;

        let xsd_parser = &ctx.xsd_parser_crate;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let boxed_deserializer_ident =
            boxed_deserializer_ident(boxed_deserializer, deserializer_ident);
        let deserialize_mapper = do_box(
            boxed_deserializer,
            quote!(#boxed_deserializer_ident::#variant_ident(x)),
        );
        let target_type = ctx.resolve_type_for_deserialize_module(target_type);

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::QName),
            quote!(#xsd_parser::quick_xml::WithDeserializer),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
        ]);

        let body = quote! {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = <#target_type as WithDeserializer>::Deserializer::init(reader, event)?;

            return Ok(DeserializerOutput {
                artifact: artifact.map(
                    |x| super::#type_ident(Box::new(x)),
                    |x| #deserialize_mapper,
                ),
                event,
                allow_any,
            });
        };

        if let Some(module) = ident.ns.and_then(|ns| ctx.types.modules.get(&ns)) {
            let ns_name = ctx.resolve_type_for_deserialize_module(&module.make_ns_const());

            quote! {
                if matches!(reader.resolve_local_name(QName(&type_name), &#ns_name), Some(#b_name)) {
                    #body
                }
            }
        } else {
            quote! {
                if type_name == #b_name {
                    #body
                }
            }
        }
    }

    fn render_deserializer_next(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
        deserializer_ident: &Ident2,
    ) -> TokenStream {
        let Self { variant_ident, .. } = self;

        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let boxed_deserializer_ident =
            boxed_deserializer_ident(boxed_deserializer, deserializer_ident);
        let deserialize_mapper = do_box(
            boxed_deserializer,
            quote!(#boxed_deserializer_ident::#variant_ident(x)),
        );

        quote! {
            #boxed_deserializer_ident::#variant_ident(x) => {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = x.next(reader, event)?;

                Ok(DeserializerOutput {
                    artifact: artifact.map(
                        |x| super::#type_ident(Box::new(x)),
                        |x| #deserialize_mapper,
                    ),
                    event,
                    allow_any,
                })
            },
        }
    }
}

/* ReferenceType */

impl ReferenceType<'_> {
    pub(crate) fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            mode,
            occurs,
            type_ident,
            target_type,
            ..
        } = self;

        if matches!(mode, TypedefMode::Auto | TypedefMode::Typedef) {
            return;
        }

        let target_type = ctx.resolve_type_for_module(target_type);
        let xsd_parser = &ctx.xsd_parser_crate;
        let body = match occurs {
            Occurs::None => return,
            Occurs::Single => {
                quote! {
                    Ok(Self(#target_type::deserialize_bytes(reader, bytes)?))
                }
            }
            Occurs::Optional => {
                quote! {
                    Ok(Self(Some(#target_type::deserialize_bytes(reader, bytes)?)))
                }
            }
            Occurs::DynamicList => {
                quote! {
                    Ok(Self(bytes
                        .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                        .map(|bytes| #target_type::deserialize_bytes(reader, bytes))
                        .collect::<Result<Vec<_>, _>>()?
                    ))
                }
            }
            Occurs::StaticList(size) => {
                ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

                quote! {
                    let arr: [Option<#target_type>; #size];
                    let parts = bytes
                        .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                        .map(|bytes| #target_type::deserialize_bytes(reader, bytes));
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

        let usings = [
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::DeserializeBytes),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
        ];
        let code = quote! {
            impl DeserializeBytes for #type_ident {
                fn deserialize_bytes<R>(
                    reader: &R,
                    bytes: &[u8],
                ) -> Result<Self, Error>
                where
                    R: DeserializeReader
                {
                    #body
                }
            }
        };

        ctx.module().usings(usings).append(code);
    }
}

/* EnumerationType */

impl EnumerationType<'_> {
    pub(crate) fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            ..
        } = self;

        let xsd_parser = &ctx.xsd_parser_crate;

        let mut other = None;
        let variants = variants
            .iter()
            .filter_map(|v| v.render_deserializer_variant(ctx, &mut other))
            .collect::<Vec<_>>();

        let other = other.unwrap_or_else(|| {
            ctx.add_usings([
                quote!(#xsd_parser::quick_xml::ErrorKind),
                quote!(#xsd_parser::quick_xml::RawByteStr),
            ]);

            quote! {
                x => Err(
                    reader.map_error(
                        ErrorKind::UnknownOrInvalidValue(
                            RawByteStr::from_slice(x)
                        )
                    )
                ),
            }
        });

        let usings = [
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::DeserializeBytes),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
        ];
        let code = quote! {
            impl DeserializeBytes for #type_ident {
                fn deserialize_bytes<R>(
                    reader: &R,
                    bytes: &[u8],
                ) -> Result<Self, Error>
                where
                    R: DeserializeReader
                {
                    match bytes {
                        #( #variants )*
                        #other
                    }
                }
            }
        };

        ctx.module().usings(usings).append(code);
    }
}

impl EnumerationTypeVariant<'_> {
    fn render_deserializer_variant(
        &self,
        ctx: &Context<'_, '_>,
        other: &mut Option<TokenStream>,
    ) -> Option<TokenStream> {
        let Self {
            info,
            target_type,
            variant_ident,
        } = self;

        if let Some(target_type) = target_type {
            let target_type = ctx.resolve_type_for_module(target_type);

            *other = Some(
                quote! { x => Ok(Self::#variant_ident(#target_type::deserialize_bytes(reader, x)?)), },
            );

            return None;
        }

        let name = Literal::byte_string(info.ident.name.to_string().as_bytes());

        Some(quote! {
            #name => Ok(Self::#variant_ident),
        })
    }
}

/* ComplexType */

impl ComplexType<'_> {
    pub(crate) fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_deserializer(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_deserializer(ctx);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_deserializer(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_deserializer(ctx);
                }
            }
        }
    }
}

impl ComplexTypeBase {
    fn return_end_event(&self, ctx: &Context<'_, '_>) -> (TokenStream, TokenStream) {
        let xsd_parser = &ctx.xsd_parser_crate;
        ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::DeserializerEvent)]);

        if self.represents_element() {
            (quote!(), quote!(DeserializerEvent::None))
        } else {
            (quote!(event @), quote!(DeserializerEvent::Continue(event)))
        }
    }

    fn render_with_deserializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            deserializer_ident,
            ..
        } = self;
        let xsd_parser = &ctx.xsd_parser_crate;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_type = if boxed_deserializer {
            quote!(Box<quick_xml_deserialize::#deserializer_ident>)
        } else {
            quote!(quick_xml_deserialize::#deserializer_ident)
        };

        let usings = [quote!(#xsd_parser::quick_xml::WithDeserializer)];
        let code = quote! {
            impl WithDeserializer for #type_ident {
                type Deserializer = #deserializer_type;
            }
        };

        ctx.module().usings(usings).append(code);
    }

    fn render_deserializer_impl(
        &self,
        ctx: &mut Context<'_, '_>,
        fn_init: &TokenStream,
        fn_next: &TokenStream,
        fn_finish: &TokenStream,
        finish_mut_self: bool,
    ) {
        let xsd_parser = &ctx.xsd_parser_crate;
        let type_ident = &self.type_ident;
        let deserializer_ident = &self.deserializer_ident;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_type = if boxed_deserializer {
            quote!(Box<#deserializer_ident>)
        } else {
            quote!(#deserializer_ident)
        };
        let mut_ = finish_mut_self.then(|| quote!(mut));

        let usings = [
            quote!(#xsd_parser::quick_xml::Event),
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::Deserializer),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
            quote!(#xsd_parser::quick_xml::DeserializerResult),
        ];

        let code = quote! {
            impl<'de> Deserializer<'de, super::#type_ident> for #deserializer_type {
                fn init<R>(
                    reader: &R,
                    event: Event<'de>,
                ) -> DeserializerResult<'de, super::#type_ident>
                where
                    R: DeserializeReader,
                {
                    #fn_init
                }

                fn next<R>(
                    mut self,
                    reader: &R,
                    event: Event<'de>,
                ) -> DeserializerResult<'de, super::#type_ident>
                where
                    R: DeserializeReader,
                {
                    #xsd_parser::print_stack_info!();

                    #fn_next
                }

                fn finish<R>(#mut_ self, reader: &R) -> Result<super::#type_ident, Error>
                where
                    R: DeserializeReader,
                {
                    #fn_finish
                }
            }
        };

        ctx.quick_xml_deserialize().usings(usings).append(code);
    }

    fn render_deserializer_fn_init_for_element(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let _ctx = ctx;
        let deserializer_ident = &self.deserializer_ident;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let boxed_deserializer_ident =
            boxed_deserializer_ident(boxed_deserializer, deserializer_ident);

        quote! {
            reader.init_deserializer_from_start_event(event, #boxed_deserializer_ident::from_bytes_start)
        }
    }
}

impl ComplexTypeEnum<'_> {
    fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        self.render_with_deserializer(ctx);
        self.render_deserializer_type(ctx);
        self.render_deserializer_state_type(ctx);
        self.render_deserializer_helper(ctx);
        self.render_deserializer_impl(ctx);
    }

    fn render_deserializer_type(&self, ctx: &mut Context<'_, '_>) {
        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let code = quote! {
            #[derive(Debug)]
            pub struct #deserializer_ident {
                state: Box<#deserializer_state_ident>,
            }
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_state_type(&self, ctx: &mut Context<'_, '_>) {
        let type_ident = &self.type_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;
        let variants = self
            .elements
            .iter()
            .map(|x| x.deserializer_enum_variant_decl(ctx));

        let code = quote! {
            #[derive(Debug)]
            pub enum #deserializer_state_ident {
                Init__,
                #( #variants )*
                Done__(super::#type_ident),
                Unknown__,
            }
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_helper(&self, ctx: &mut Context<'_, '_>) {
        let represents_element = self.represents_element();
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let fn_find_suitable = self.render_deserializer_fn_find_suitable(ctx);
        let fn_from_bytes_start =
            represents_element.then(|| self.render_deserializer_fn_from_bytes_start(ctx));
        let fn_finish_state = self.render_deserializer_fn_finish_state(ctx);

        let store_elements = self
            .elements
            .iter()
            .map(|x| x.deserializer_enum_variant_fn_store(ctx));
        let handle_elements = self.elements.iter().map(|x| {
            x.deserializer_enum_variant_fn_handle(
                ctx,
                represents_element,
                &boxed_deserializer_ident(boxed_deserializer, deserializer_ident),
                deserializer_state_ident,
            )
        });

        let code = quote! {
            impl #deserializer_ident {
                #fn_find_suitable
                #fn_from_bytes_start
                #fn_finish_state

                #( #store_elements )*
                #( #handle_elements )*
            }
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_fn_find_suitable(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let allow_any = self.any_element.is_some();
        let xsd_parser = &ctx.xsd_parser_crate;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let elements = self
            .elements
            .iter()
            .filter_map(|x| x.deserializer_enum_variant_init_element(ctx))
            .collect::<Vec<_>>();
        let groups = self
            .elements
            .iter()
            .filter_map(|x| x.deserializer_enum_variant_init_group(ctx, !allow_any))
            .collect::<Vec<_>>();

        let x = if elements.is_empty() {
            quote!(_)
        } else {
            quote!(x)
        };

        let (allow_any_result, allow_any_decl) = if groups.is_empty() || allow_any {
            (quote!(#allow_any), None)
        } else {
            (
                quote!(allow_any_element),
                Some(quote!(let mut allow_any_element = false;)),
            )
        };

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            fn find_suitable<'de, R>(
                &mut self,
                reader: &R,
                event: Event<'de>,
                fallback: &mut Option<#deserializer_state_ident>,
            ) -> Result<ElementHandlerOutput<'de>, Error>
            where
                R: DeserializeReader,
            {
                let (Event::Start(#x) | Event::Empty(#x)) = &event else {
                    *self.state = fallback.take().unwrap_or(#deserializer_state_ident::Init__);

                    return Ok(ElementHandlerOutput::return_to_parent(event, #allow_any));
                };

                #allow_any_decl

                #( #elements )*
                #( #groups )*

                *self.state = fallback.take().unwrap_or(#deserializer_state_ident::Init__);

                Ok(ElementHandlerOutput::return_to_parent(event, #allow_any_result))
            }
        }
    }

    fn render_deserializer_fn_from_bytes_start(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_state_ident = &self.deserializer_state_ident;

        let self_type = if boxed_deserializer {
            quote!(Box<Self>)
        } else {
            quote!(Self)
        };

        let self_ctor = do_box(
            boxed_deserializer,
            quote! {
                Self {
                    state: Box::new(#deserializer_state_ident::Init__)
                }
            },
        );

        let attrib_loop = self.any_attribute.is_none().then(|| {
            ctx.add_quick_xml_deserialize_usings([
                quote!(#xsd_parser::quick_xml::filter_xmlns_attributes),
            ]);

            quote! {
                for attrib in filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
        });

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::BytesStart),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
        ]);

        quote! {
            fn from_bytes_start<R>(
                reader: &R,
                bytes_start: &BytesStart<'_>
            ) -> Result<#self_type, Error>
            where
                R: DeserializeReader,
            {
                #attrib_loop

                Ok(#self_ctor)
            }
        }
    }

    fn render_deserializer_fn_finish_state(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let type_ident = &self.type_ident;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let finish_elements = self.elements.iter().map(|x| {
            x.deserializer_enum_variant_finish(
                ctx,
                type_ident,
                &boxed_deserializer_ident(boxed_deserializer, deserializer_ident),
            )
        });

        ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

        quote! {
            fn finish_state<R>(reader: &R, state: #deserializer_state_ident) -> Result<super::#type_ident, Error>
            where
                R: DeserializeReader,
            {
                use #deserializer_state_ident as S;

                match state {
                    S::Init__ => Err(ErrorKind::MissingContent.into()),
                    #( #finish_elements )*
                    S::Done__(data) => Ok(data),
                    S::Unknown__ => unreachable!(),
                }
            }
        }
    }

    fn render_deserializer_impl(&self, ctx: &mut Context<'_, '_>) {
        let fn_init = self.render_deserializer_fn_init(ctx);
        let fn_next = self.render_deserializer_fn_next(ctx);
        let fn_finish = self.render_deserializer_fn_finish(ctx);

        self.base
            .render_deserializer_impl(ctx, &fn_init, &fn_next, &fn_finish, false);
    }

    fn render_deserializer_fn_init(&self, ctx: &Context<'_, '_>) -> TokenStream {
        if self.represents_element() {
            self.render_deserializer_fn_init_for_element(ctx)
        } else {
            self.render_deserializer_fn_init_for_group(ctx)
        }
    }

    fn render_deserializer_fn_init_for_group(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let _self = self;

        let xsd_parser = &ctx.xsd_parser_crate;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_ident = &self.deserializer_ident;
        let boxed_deserializer_ident =
            boxed_deserializer_ident(boxed_deserializer, deserializer_ident);
        let deserializer_state_ident = &self.deserializer_state_ident;

        let init_deserializer = do_box(
            boxed_deserializer,
            quote! {
                #boxed_deserializer_ident {
                    state: Box::new(#deserializer_state_ident::Init__),
                }
            },
        );

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            let deserializer = #init_deserializer;
            let mut output = deserializer.next(reader, event)?;

            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x) if matches!(&*x.state, #deserializer_state_ident::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };

            Ok(output)
        }
    }

    fn render_deserializer_fn_next(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_ident =
            boxed_deserializer_ident(boxed_deserializer, &self.deserializer_ident);
        let deserializer_state_ident = &self.deserializer_state_ident;
        let (event_at, return_end_event) = self.return_end_event(ctx);

        let handlers_continue = self
            .elements
            .iter()
            .map(|x| x.deserializer_enum_variant_fn_next_continue(ctx));
        let handlers_create = self
            .elements
            .iter()
            .map(|x| x.deserializer_enum_variant_fn_next_create(ctx));

        ctx.add_quick_xml_deserialize_usings([
            quote!(core::mem::replace),
            quote!(#xsd_parser::quick_xml::DeserializerEvent),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
        ]);

        quote! {
            use #deserializer_state_ident as S;

            let mut event = event;
            let mut fallback = None;

            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    #( #handlers_continue )*
                    (state, #event_at Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(#deserializer_ident::finish_state(reader, state)?),
                            event: #return_end_event,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    #( #handlers_create )*
                    (s @ S::Done__(_), event) => {
                        *self.state = s;

                        break (DeserializerEvent::Continue(event), false);
                    },
                    (S::Unknown__, _) => unreachable!(),
                }
            };

            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };

            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
    }

    fn render_deserializer_fn_finish(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_ident =
            boxed_deserializer_ident(boxed_deserializer, &self.deserializer_ident);

        quote! {
            #deserializer_ident::finish_state(reader, *self.state)
        }
    }
}

impl ComplexTypeStruct<'_> {
    fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        self.render_with_deserializer(ctx);
        self.render_deserializer_type(ctx);
        self.render_deserializer_state_type(ctx);
        self.render_deserializer_helper(ctx);
        self.render_deserializer_impl(ctx);
    }

    fn render_deserializer_type(&self, ctx: &mut Context<'_, '_>) {
        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;
        let attributes = self
            .attributes
            .iter()
            .map(|x| x.deserializer_struct_field_decl(ctx));
        let elements = self
            .elements()
            .iter()
            .map(|x| x.deserializer_struct_field_decl(ctx));
        let content = self.content().map(|x| x.deserializer_field_decl(ctx));

        let code = quote! {
            #[derive(Debug)]
            pub struct #deserializer_ident {
                #( #attributes )*
                #( #elements )*
                #content
                state: Box<#deserializer_state_ident>,
            }
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_state_type(&self, ctx: &mut Context<'_, '_>) {
        let xsd_parser = &ctx.xsd_parser_crate;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let mut use_with_deserializer = Some(quote!(#xsd_parser::quick_xml::WithDeserializer));

        let variants = match &self.mode {
            StructMode::Empty { .. } => {
                use_with_deserializer = None;

                quote! {
                    Init__,
                    Unknown__,
                }
            }
            StructMode::Content { content } => {
                let target_type = ctx.resolve_type_for_deserialize_module(&content.target_type);

                let next = content.need_next_state().then(|| {
                    quote! {
                        Next__,
                    }
                });
                let done = content.need_done_state(self.represents_element()).then(|| {
                    quote! {
                        Done__,
                    }
                });

                quote! {
                    Init__,
                    #next
                    Content__(<#target_type as WithDeserializer>::Deserializer),
                    #done
                    Unknown__,
                }
            }
            StructMode::All { elements, .. } => {
                let variants = elements.iter().map(|element| {
                    let target_type = ctx.resolve_type_for_deserialize_module(&element.target_type);
                    let variant_ident = &element.variant_ident;

                    quote! {
                        #variant_ident(<#target_type as WithDeserializer>::Deserializer),
                    }
                });

                quote! {
                    Init__,
                    Next__,
                    #( #variants )*
                    Unknown__,
                }
            }
            StructMode::Sequence { elements, .. } => {
                let variants = elements.iter().map(|element| {
                    let target_type = ctx.resolve_type_for_deserialize_module(&element.target_type);
                    let variant_ident = &element.variant_ident;

                    quote! {
                        #variant_ident(Option<<#target_type as WithDeserializer>::Deserializer>),
                    }
                });

                quote! {
                    Init__,
                    #( #variants )*
                    Done__,
                    Unknown__,
                }
            }
        };

        let code = quote! {
            #[derive(Debug)]
            enum #deserializer_state_ident {
                #variants
            }
        };

        ctx.quick_xml_deserialize()
            .usings(use_with_deserializer)
            .append(code);
    }

    fn render_deserializer_helper(&self, ctx: &mut Context<'_, '_>) {
        let type_ident = &self.type_ident;
        let represents_element = self.represents_element();
        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let fn_find_suitable = matches!(&self.mode, StructMode::All { .. })
            .then(|| self.render_deserializer_fn_find_suitable(ctx));
        let fn_from_bytes_start = self
            .represents_element()
            .then(|| self.render_deserializer_fn_from_bytes_start(ctx));
        let fn_finish_state = self.render_deserializer_fn_finish_state(ctx);

        let store_content = self
            .content()
            .map(|x| x.deserializer_struct_field_fn_store(ctx));
        let handle_content = self.content().map(|x| {
            x.deserializer_struct_field_fn_handle(
                ctx,
                type_ident,
                represents_element,
                deserializer_state_ident,
            )
        });

        let elements = self.elements();
        let store_elements = elements
            .iter()
            .map(|x| x.deserializer_struct_field_fn_store(ctx));
        let handle_elements = elements.iter().enumerate().map(|(i, x)| {
            let next = elements.get(i + 1);

            if let StructMode::All { .. } = &self.mode {
                x.deserializer_struct_field_fn_handle_all(ctx, deserializer_state_ident)
            } else {
                x.deserializer_struct_field_fn_handle_sequence(ctx, next, deserializer_state_ident)
            }
        });

        let code = quote! {
            impl #deserializer_ident {
                #fn_find_suitable
                #fn_from_bytes_start
                #fn_finish_state

                #store_content
                #handle_content

                #( #store_elements )*
                #( #handle_elements )*
            }
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_fn_find_suitable(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let allow_any = self.any_element().is_some();
        let xsd_parser = &ctx.xsd_parser_crate;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let elements = self
            .elements()
            .iter()
            .filter_map(|x| x.deserializer_struct_field_init_element(ctx));
        let groups = self
            .elements()
            .iter()
            .filter_map(|x| x.deserializer_struct_field_init_group(ctx, !allow_any))
            .collect::<Vec<_>>();

        let (allow_any_result, allow_any_decl) = if groups.is_empty() || allow_any {
            (quote!(#allow_any), None)
        } else {
            (
                quote!(allow_any_element),
                Some(quote!(let mut allow_any_element = false;)),
            )
        };

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            fn find_suitable<'de, R>(
                &mut self,
                reader: &R,
                event: Event<'de>,
                fallback: &mut Option<#deserializer_state_ident>,
            ) -> Result<ElementHandlerOutput<'de>, Error>
            where
                R: DeserializeReader,
            {
                let (Event::Start(x) | Event::Empty(x)) = &event else {
                    *self.state = fallback.take().unwrap_or(#deserializer_state_ident::Init__);

                    return Ok(ElementHandlerOutput::return_to_parent(event, #allow_any));
                };

                #allow_any_decl

                #( #elements )*
                #( #groups )*

                *self.state = fallback.take().unwrap_or(#deserializer_state_ident::Init__);

                Ok(ElementHandlerOutput::return_to_parent(event, #allow_any_result))
            }
        }
    }

    fn render_deserializer_fn_from_bytes_start(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_state_ident = &self.deserializer_state_ident;

        let attrib_var = self.attributes.iter().map(|x| x.deserializer_var_decl(ctx));
        let attrib_match = self
            .attributes
            .iter()
            .enumerate()
            .map(|(i, x)| x.deserializer_matcher(ctx, i == 0));
        let attrib_init = self
            .attributes
            .iter()
            .map(|x| x.deserializer_struct_field_init(ctx, &self.type_ident));
        let element_init = self
            .elements()
            .iter()
            .map(ComplexTypeElement::deserializer_struct_field_init);
        let content_init = self
            .content()
            .map(ComplexTypeContent::deserializer_struct_field_init);

        let raise_unexpected_attrib = self.any_attribute.is_none().then(|| {
            let body = quote! {
                reader.raise_unexpected_attrib(attrib)?;
            };

            if self.has_attributes() {
                quote! {
                    else {
                        #body
                    }
                }
            } else {
                body
            }
        });

        let need_attrib_loop = self.has_attributes() || raise_unexpected_attrib.is_some();
        let attrib_loop = need_attrib_loop.then(|| {
            ctx.add_quick_xml_deserialize_usings([
                quote!(#xsd_parser::quick_xml::filter_xmlns_attributes),
            ]);

            quote! {
                for attrib in filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;

                    #( #attrib_match )*
                    #raise_unexpected_attrib
                }
            }
        });

        let self_type = if boxed_deserializer {
            quote!(Box<Self>)
        } else {
            quote!(Self)
        };

        let self_ctor = do_box(
            boxed_deserializer,
            quote! {
                Self {
                    #( #attrib_init )*
                    #( #element_init )*
                    #content_init
                    state: Box::new(#deserializer_state_ident::Init__),
                }
            },
        );

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::BytesStart),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
        ]);

        quote! {
            fn from_bytes_start<R>(
                reader: &R,
                bytes_start: &BytesStart<'_>
            ) -> Result<#self_type, Error>
            where
                R: DeserializeReader,
            {
                #( #attrib_var )*

                #attrib_loop

                Ok(#self_ctor)
            }
        }
    }

    fn render_deserializer_fn_finish_state(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let body = match &self.mode {
            StructMode::All { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|x| x.deserializer_struct_field_finish_state_all());

                quote! {
                    use #deserializer_state_ident as S;

                    match state {
                        #( #elements )*
                        _ => (),
                    }

                    Ok(())
                }
            }
            StructMode::Sequence { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|x| x.deserializer_struct_field_finish_state_sequence());

                quote! {
                    use #deserializer_state_ident as S;

                    match state {
                        #( #elements )*
                        _ => (),
                    }

                    Ok(())
                }
            }
            StructMode::Content { .. } => {
                quote! {
                    if let #deserializer_state_ident::Content__(deserializer) = state {
                        self.store_content(deserializer.finish(reader)?)?;
                    }

                    Ok(())
                }
            }
            _ => quote! { Ok(()) },
        };

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
        ]);

        quote! {
            fn finish_state<R>(&mut self, reader: &R, state: #deserializer_state_ident) -> Result<(), Error>
            where
                R: DeserializeReader,
            {
                #body
            }
        }
    }

    fn render_deserializer_impl(&self, ctx: &mut Context<'_, '_>) {
        let fn_init = self.render_deserializer_fn_init(ctx);
        let fn_next = self.render_deserializer_fn_next(ctx);
        let fn_finish = self.render_deserializer_fn_finish(ctx);

        self.base
            .render_deserializer_impl(ctx, &fn_init, &fn_next, &fn_finish, true);
    }

    fn render_deserializer_fn_init(&self, ctx: &mut Context<'_, '_>) -> TokenStream {
        if matches!(&self.mode, StructMode::Content { content } if content.is_simple) {
            self.render_deserializer_fn_init_simple(ctx)
        } else if self.represents_element() {
            self.render_deserializer_fn_init_for_element(ctx)
        } else {
            self.render_deserializer_fn_init_for_group(ctx)
        }
    }

    fn render_deserializer_fn_init_simple(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let deserializer_ident = &self.deserializer_ident;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let boxed_deserializer_ident =
            boxed_deserializer_ident(boxed_deserializer, deserializer_ident);

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Event),
            quote!(#xsd_parser::quick_xml::DeserializerEvent),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::ContentDeserializer),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };

            #boxed_deserializer_ident::from_bytes_start(reader, x)?.next(reader, event)
        }
    }

    fn render_deserializer_fn_init_for_group(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let deserializer_ident = &self.deserializer_ident;
        let boxed_deserializer_ident =
            boxed_deserializer_ident(boxed_deserializer, deserializer_ident);
        let deserializer_state_ident = &self.deserializer_state_ident;

        let element_init = self
            .elements()
            .iter()
            .map(ComplexTypeElement::deserializer_struct_field_init);
        let content_init = self
            .content()
            .map(ComplexTypeContent::deserializer_struct_field_init);
        let init_deserializer = do_box(
            boxed_deserializer,
            quote! {
                #boxed_deserializer_ident {
                    #( #element_init )*
                    #content_init
                    state: Box::new(#deserializer_state_ident::Init__),
                }
            },
        );

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            let deserializer = #init_deserializer;
            let mut output = deserializer.next(reader, event)?;

            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x) if matches!(&*x.state, #deserializer_state_ident::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };

            Ok(output)
        }
    }

    fn render_deserializer_fn_next(&self, ctx: &Context<'_, '_>) -> TokenStream {
        match &self.mode {
            StructMode::Empty { any_element } => {
                self.render_deserializer_fn_next_empty(ctx, any_element.is_some())
            }
            StructMode::Content { content } => {
                if content.is_simple {
                    self.render_deserializer_fn_next_content_simple(ctx)
                } else {
                    self.render_deserializer_fn_next_content_complex(ctx, content)
                }
            }
            StructMode::All { .. } => self.render_deserializer_fn_next_all(ctx),
            StructMode::Sequence { .. } => self.render_deserializer_fn_next_sequence(ctx),
        }
    }

    fn render_deserializer_fn_next_empty(
        &self,
        ctx: &Context<'_, '_>,
        allow_any: bool,
    ) -> TokenStream {
        let _self = self;
        let xsd_parser = &ctx.xsd_parser_crate;
        let (_, return_end_event) = self.return_end_event(ctx);

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Event),
            quote!(#xsd_parser::quick_xml::DeserializerEvent),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
                    event: #return_end_event,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: #allow_any,
                })
            }
        }
    }

    fn render_deserializer_fn_next_content_simple(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let deserializer_state_ident = &self.deserializer_state_ident;

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::ContentDeserializer),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            use #deserializer_state_ident as S;

            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
    }

    fn render_deserializer_fn_next_content_complex(
        &self,
        ctx: &Context<'_, '_>,
        content: &ComplexTypeContent,
    ) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&content.target_type);
        let (event_at, return_end_event) = self.return_end_event(ctx);
        let deserializer_state_ident = &self.deserializer_state_ident;

        let has_done_state = content.need_done_state(self.represents_element());
        let done_handler = has_done_state.then(|| {
            quote! {
                (S::Done__, event) => {
                    *self.state = S::Done__;

                    break (DeserializerEvent::Continue(event), false);
                },
            }
        });
        let artifact_handler = if has_done_state {
            quote! {
                let artifact = match &*self.state {
                    S::Done__ => DeserializerArtifact::Data(self.finish(reader)?),
                    _ => DeserializerArtifact::Deserializer(self),
                };
            }
        } else {
            quote! {
                let artifact = DeserializerArtifact::Deserializer(self);
            }
        };

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Event),
            quote!(#xsd_parser::quick_xml::WithDeserializer),
            quote!(#xsd_parser::quick_xml::DeserializerEvent),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
        ]);

        quote! {
            use #deserializer_state_ident as S;

            let mut event = event;
            let mut fallback = None;

            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);

                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, #event_at Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: #return_end_event,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <#target_type as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    },
                    #done_handler
                    (S::Unknown__, _) => unreachable!(),
                }
            };

            #artifact_handler

            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
    }

    fn render_deserializer_fn_next_all(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let (event_at, return_end_event) = self.return_end_event(ctx);
        let deserializer_state_ident = &self.deserializer_state_ident;

        let handlers = self
            .elements()
            .iter()
            .map(|x| x.deserializer_struct_field_fn_next_all(ctx));

        quote! {
            use #deserializer_state_ident as S;

            let mut event = event;
            let mut fallback = None;

            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);

                event = match (state, event) {
                    #( #handlers )*
                    (_, #event_at Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: #return_end_event,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                        }
                    },
                    (S::Unknown__, _) => unreachable!(),
                }
            };

            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
    }

    fn render_deserializer_fn_next_sequence(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let allow_any = self.any_element().is_some();
        let xsd_parser = &ctx.xsd_parser_crate;
        let (event_at, return_end_event) = self.return_end_event(ctx);
        let deserializer_state_ident = &self.deserializer_state_ident;

        let elements = self.elements();
        let first = elements
            .first()
            .expect("`Sequence` should always have at least one element!");
        let first_ident = &first.variant_ident;

        let handlers_continue = elements
            .iter()
            .map(|x| x.deserializer_struct_field_fn_next_sequence_continue(ctx));
        let handlers_create = elements.iter().enumerate().map(|(i, x)| {
            let next = elements.get(i + 1);

            x.deserializer_struct_field_fn_next_sequence_create(ctx, next, allow_any)
        });

        ctx.add_quick_xml_deserialize_usings([
            quote!(core::mem::replace),
            quote!(#xsd_parser::quick_xml::Event),
            quote!(#xsd_parser::quick_xml::WithDeserializer),
            quote!(#xsd_parser::quick_xml::DeserializerEvent),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        let init_set_any = allow_any.then(|| {
            quote! {
                allow_any_element = true;
            }
        });
        let done_allow_any = if allow_any {
            quote!(true)
        } else {
            quote!(allow_any_element)
        };

        quote! {
            use #deserializer_state_ident as S;

            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;

            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);

                event = match (state, event) {
                    #( #handlers_continue )*
                    (_, #event_at Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }

                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: #return_end_event,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        #init_set_any

                        fallback.get_or_insert(S::Init__);

                        *self.state = #deserializer_state_ident::#first_ident(None);

                        event
                    },
                    #( #handlers_create )*
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), #done_allow_any);
                    },
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };

            if let Some(fallback) = fallback {
                *self.state = fallback;
            }

            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
    }

    fn render_deserializer_fn_finish(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let type_ident = &self.type_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let attributes = self
            .attributes
            .iter()
            .map(ComplexTypeAttribute::deserializer_struct_field_finish);
        let elements = self
            .elements()
            .iter()
            .map(|x| x.deserializer_struct_field_finish(ctx));
        let content = self
            .content()
            .map(|x| x.deserializer_struct_field_finish(ctx));

        ctx.add_quick_xml_deserialize_usings([quote!(core::mem::replace)]);

        quote! {
            let state = replace(&mut *self.state, #deserializer_state_ident::Unknown__);
            self.finish_state(reader, state)?;

            Ok(super::#type_ident {
                #( #attributes )*
                #( #elements )*
                #content
            })
        }
    }
}

impl ComplexTypeContent {
    fn need_next_state(&self) -> bool {
        !self.is_simple
    }

    fn need_done_state(&self, represents_element: bool) -> bool {
        !self.is_simple && !represents_element && self.max_occurs.is_bounded()
    }

    fn deserializer_field_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let target_type = match self.occurs {
            Occurs::Single | Occurs::Optional => quote!(Option<#target_type>),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(Vec<#target_type>),
            e => crate::unreachable!("{:?}", e),
        };

        quote! {
            content: #target_type,
        }
    }

    fn deserializer_struct_field_init(&self) -> TokenStream {
        match self.occurs {
            Occurs::None => quote!(),
            Occurs::Single | Occurs::Optional => quote!(content: None,),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(content: Vec::new(),),
        }
    }

    fn deserializer_struct_field_finish(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;

        let convert = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => {
                ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

                quote! {
                    self.content.ok_or_else(|| ErrorKind::MissingContent)?
                }
            }
            Occurs::Optional | Occurs::DynamicList => {
                quote! { self.content }
            }
            Occurs::StaticList(sz) => {
                ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

                quote! {
                    self.content.try_into().map_err(|vec: Vec<_>| ErrorKind::InsufficientSize {
                        min: #sz,
                        max: #sz,
                        actual: vec.len(),
                    })?
                }
            }
        };

        quote! {
            content: #convert,
        }
    }

    fn deserializer_struct_field_fn_store(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let body = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single | Occurs::Optional => {
                ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

                quote! {
                    if self.content.is_some() {
                        Err(ErrorKind::DuplicateContent)?;
                    }

                    self.content = Some(value);
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                self.content.push(value);
            },
        };

        ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::Error)]);

        quote! {
            fn store_content(&mut self, value: #target_type) -> Result<(), Error> {
                #body

                Ok(())
            }
        }
    }

    fn deserializer_struct_field_fn_handle(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
        represents_element: bool,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        if self.is_simple {
            self.deserializer_struct_field_fn_handle_simple(
                ctx,
                type_ident,
                deserializer_state_ident,
            )
        } else {
            self.deserializer_struct_field_fn_handle_complex(
                ctx,
                represents_element,
                deserializer_state_ident,
            )
        }
    }

    fn deserializer_struct_field_fn_handle_simple(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);
        let boxed_deserializer = ctx.get::<BoxedDeserializer>();
        let self_type = boxed_deserializer.then(|| quote!(: Box<Self>));

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::DeserializeReader),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerResult),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            fn handle_content<'de, R>(
                mut self #self_type,
                reader: &R,
                output: DeserializerOutput<'de, #target_type>,
            ) -> DeserializerResult<'de, super::#type_ident>
            where
                R: DeserializeReader,
            {
                use #deserializer_state_ident as S;

                let DeserializerOutput { artifact, event, allow_any } = output;

                match artifact {
                    DeserializerArtifact::None => Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::None,
                        event,
                        allow_any,
                    }),
                    DeserializerArtifact::Data(data) => {
                        self.store_content(data)?;
                        let data = self.finish(reader)?;

                        Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(data),
                            event,
                            allow_any,
                        })
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state = S::Content__(deserializer);

                        Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Deserializer(self),
                            event,
                            allow_any,
                        })
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn deserializer_struct_field_fn_handle_complex(
        &self,
        ctx: &Context<'_, '_>,
        represents_element: bool,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::DeserializeReader),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        // Handler for `DeserializerArtifact::Data`
        let data_handler = match (represents_element, self.occurs, self.max_occurs) {
            (_, Occurs::None, _) => unreachable!(),
            // Return instantly if we have received the expected value
            (false, Occurs::Single | Occurs::Optional, _) => quote! {
                *self.state = #deserializer_state_ident::Done__;

                ElementHandlerOutput::Break {
                    event,
                    allow_any,
                }
            },
            // Finish the deserialization if the expected max value has been reached.
            // Continue if not.
            (false, Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => {
                quote! {
                    if self.content.len() < #max {
                        *self.state = #deserializer_state_ident::Next__;

                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        *self.state = #deserializer_state_ident::Done__;

                        ElementHandlerOutput::Break {
                            event,
                            allow_any,
                        }
                    }
                }
            }
            // Value is unbound, continue in any case
            (_, _, _) => quote! {
                *self.state = #deserializer_state_ident::Next__;

                ElementHandlerOutput::from_event(event, allow_any)
            },
        };

        // Handler for `DeserializerArtifact::Deserializer`
        let deserializer_handler = match (self.occurs, self.max_occurs) {
            (Occurs::None, _) => unreachable!(),
            // If we only expect one element we never initialize a new deserializer
            // we only continue the deserialization process for `End` events (because
            // they may finish this deserializer).
            (Occurs::Single | Occurs::Optional, _) => quote! {
                *self.state = #deserializer_state_ident::Content__(deserializer);

                ElementHandlerOutput::from_event_end(event, allow_any)
            },
            // If we expect multiple elements we only try to initialize a new
            // deserializer if the maximum has not been reached yet.
            // The `+1` is for the data that is contained in the yet unfinished
            // deserializer.
            (Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => {
                quote! {
                    let can_have_more = self.content.len().saturating_add(1) < #max;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };

                    match (can_have_more, &ret) {
                        (true, ElementHandlerOutput::Continue { .. })  => {
                            fallback.get_or_insert(#deserializer_state_ident::Content__(deserializer));

                            *self.state = #deserializer_state_ident::Next__;
                        }
                        (false, _ ) | (_, ElementHandlerOutput::Break { .. }) => {
                            *self.state = #deserializer_state_ident::Content__(deserializer);
                        }
                    }

                    ret
                }
            }
            // Unbound, we can try a new deserializer in any case.
            (Occurs::DynamicList | Occurs::StaticList(_), _) => quote! {
                let ret = ElementHandlerOutput::from_event(event, allow_any);

                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = #deserializer_state_ident::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(#deserializer_state_ident::Content__(deserializer));

                        *self.state = #deserializer_state_ident::Next__;
                    }
                }

                ret
            },
        };

        quote! {
            fn handle_content<'de, R>(
                &mut self,
                reader: &R,
                output: DeserializerOutput<'de, #target_type>,
                fallback: &mut Option<#deserializer_state_ident>,
            ) -> Result<ElementHandlerOutput<'de>, Error>
            where
                R: DeserializeReader,
            {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;

                if artifact.is_none() {
                    *self.state = fallback.take().unwrap_or(#deserializer_state_ident::Next__);

                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }

                if let Some(fallback) = fallback.take() {
                    self.finish_state(reader, fallback)?;
                }

                Ok(match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        self.store_content(data)?;

                        #data_handler
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        #deserializer_handler
                    }
                })
            }
        }
    }
}

impl ComplexTypeAttribute<'_> {
    fn deserializer_matcher(&self, ctx: &Context<'_, '_>, is_first: bool) -> TokenStream {
        let b_name = &self.b_name;
        let field_ident = &self.ident;

        let else_ = is_first.not().then(|| quote!(else));

        if let Some(module) = self.info.ident.ns.and_then(|ns| ctx.types.modules.get(&ns)) {
            let ns_name = ctx.resolve_type_for_deserialize_module(&module.make_ns_const());

            quote! {
                #else_ if matches!(reader.resolve_local_name(attrib.key, &#ns_name), Some(#b_name)) {
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

    fn deserializer_var_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let field_ident = &self.ident;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        quote!(let mut #field_ident: Option<#target_type> = None;)
    }

    fn deserializer_struct_field_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let field_ident = &self.ident;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let target_type = if self.is_option {
            quote!(Option<#target_type>)
        } else {
            target_type
        };

        quote! {
            #field_ident: #target_type,
        }
    }

    fn deserializer_struct_field_init(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
    ) -> TokenStream {
        let field_ident = &self.ident;
        let xsd_parser = &ctx.xsd_parser_crate;

        let convert = if self.default_value.is_some() {
            let default_fn_ident = format_ident!("default_{field_ident}");

            Some(quote! { .unwrap_or_else(super::#type_ident::#default_fn_ident) })
        } else if self.info.use_ == Use::Required {
            let name = &self.s_name;

            ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

            Some(
                quote! { .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute(#name.into())))? },
            )
        } else {
            None
        };

        quote! {
            #field_ident: #field_ident #convert,
        }
    }

    fn deserializer_struct_field_finish(&self) -> TokenStream {
        let field_ident = &self.ident;

        quote! {
            #field_ident: self.#field_ident,
        }
    }
}

impl ComplexTypeElement<'_> {
    fn store_ident(&self) -> Ident2 {
        let ident = self.field_ident.to_string();
        let ident = ident.trim_start_matches('_');

        format_ident!("store_{ident}")
    }

    fn handler_ident(&self) -> Ident2 {
        let ident = self.field_ident.to_string();
        let ident = ident.trim_start_matches('_');

        format_ident!("handle_{ident}")
    }

    fn treat_as_group(&self) -> bool {
        self.info.element_mode == ElementMode::Group || self.target_is_dynamic
    }

    fn treat_as_element(&self) -> bool {
        !self.treat_as_group()
    }

    fn target_type_allows_any(&self, types: &Types) -> bool {
        fn walk(types: &Types, visit: &mut HashSet<Ident>, ident: &Ident) -> bool {
            if !visit.insert(ident.clone()) {
                return false;
            }

            match types.get_variant(ident) {
                Some(TypeVariant::All(si) | TypeVariant::Choice(si)) => {
                    if si.any.is_some() {
                        return true;
                    }

                    si.elements.iter().any(|f| walk(types, visit, &f.type_))
                }
                Some(TypeVariant::Sequence(si)) => {
                    if si.any.is_some() {
                        return true;
                    }

                    if let Some(first) = si.elements.first() {
                        return walk(types, visit, &first.type_);
                    }

                    false
                }
                Some(TypeVariant::ComplexType(ComplexInfo {
                    content: Some(content),
                    ..
                })) => walk(types, visit, content),
                _ => false,
            }
        }

        let mut visit = HashSet::new();

        walk(types, &mut visit, &self.info.type_)
    }

    fn deserializer_init_element(
        &self,
        ctx: &Context<'_, '_>,
        call_handler: &TokenStream,
    ) -> Option<TokenStream> {
        if !self.treat_as_element() {
            return None;
        }

        let b_name = &self.b_name;
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::WithDeserializer)]);

        let body = quote! {
            let output = <#target_type as WithDeserializer>::Deserializer::init(reader, event)?;

            return #call_handler;
        };

        if let Some(module) = self.info.ident.ns.and_then(|ns| ctx.types.modules.get(&ns)) {
            let ns_name = ctx.resolve_type_for_deserialize_module(&module.make_ns_const());

            Some(quote! {
                if matches!(reader.resolve_local_name(x.name(), &#ns_name), Some(#b_name)) {
                    #body
                }
            })
        } else {
            Some(quote! {
                if x.name().local_name().as_ref() == #b_name {
                    #body
                }
            })
        }
    }

    fn deserializer_init_group(
        &self,
        ctx: &Context<'_, '_>,
        handle_any: bool,
        call_handler: &TokenStream,
    ) -> Option<TokenStream> {
        if !self.treat_as_group() {
            return None;
        }

        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::WithDeserializer),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
        ]);

        let handle_continue = if handle_any {
            quote! {
                ElementHandlerOutput::Continue { event, allow_any } => {
                    allow_any_element = allow_any_element || allow_any;

                    event
                },
            }
        } else {
            quote! {
                ElementHandlerOutput::Continue { event, .. } => event,
            }
        };

        Some(quote! {
            let event = {
                let output = <#target_type as WithDeserializer>::Deserializer::init(reader, event)?;

                match #call_handler? {
                    #handle_continue
                    output => { return Ok(output); }
                }
            };
        })
    }

    fn deserializer_enum_variant_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;

        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);
        let variant_ident = &self.variant_ident;

        ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::WithDeserializer)]);

        match self.occurs {
            Occurs::Single | Occurs::Optional => quote! {
                #variant_ident(Option<#target_type>, Option<<#target_type as WithDeserializer>::Deserializer>),
            },
            Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                #variant_ident(Vec<#target_type>, Option<<#target_type as WithDeserializer>::Deserializer>),
            },
            e => crate::unreachable!("{:?}", e),
        }
    }

    fn deserializer_enum_variant_init_element(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let handler_ident = self.handler_ident();
        let call_handler =
            quote!(self.#handler_ident(reader, Default::default(), output, &mut *fallback));

        self.deserializer_init_element(ctx, &call_handler)
    }

    fn deserializer_enum_variant_init_group(
        &self,
        ctx: &Context<'_, '_>,
        handle_any: bool,
    ) -> Option<TokenStream> {
        let handler_ident = self.handler_ident();
        let call_handler =
            quote!(self.#handler_ident(reader, Default::default(), output, &mut *fallback));

        self.deserializer_init_group(ctx, handle_any, &call_handler)
    }

    fn deserializer_enum_variant_finish(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
        deserializer_ident: &Ident2,
    ) -> TokenStream {
        let name = &self.s_name;
        let xsd_parser = &ctx.xsd_parser_crate;
        let store_ident = self.store_ident();
        let variant_ident = &self.variant_ident;

        let convert = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => {
                ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

                let mut ctx = quote! {
                    values.ok_or_else(|| ErrorKind::MissingElement(#name.into()))?
                };

                if self.need_indirection {
                    ctx = quote! { Box::new(#ctx) };
                }

                ctx
            }
            Occurs::Optional if self.need_indirection => {
                quote! { values.map(Box::new) }
            }
            Occurs::Optional | Occurs::DynamicList => {
                quote! { values }
            }
            Occurs::StaticList(sz) => {
                ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

                quote! {
                    values.try_into().map_err(|vec: Vec<_>| ErrorKind::InsufficientSize {
                        min: #sz,
                        max: #sz,
                        actual: vec.len(),
                    })?
                }
            }
        };

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            S::#variant_ident(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    #deserializer_ident::#store_ident(&mut values, value)?;
                }

                Ok(super::#type_ident::#variant_ident(#convert))
            }
        }
    }

    fn deserializer_enum_variant_fn_store(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let name = &self.b_name;
        let store_ident = self.store_ident();

        match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single | Occurs::Optional => {
                ctx.add_quick_xml_deserialize_usings([
                    quote!(#xsd_parser::quick_xml::Error),
                    quote!(#xsd_parser::quick_xml::ErrorKind),
                    quote!(#xsd_parser::quick_xml::RawByteStr),
                ]);

                quote! {
                    fn #store_ident(values: &mut Option<#target_type>, value: #target_type) -> Result<(), Error> {
                        if values.is_some() {
                            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(#name)))?;
                        }

                        *values = Some(value);

                        Ok(())
                    }
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => {
                ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::Error)]);

                quote! {
                    fn #store_ident(values: &mut Vec<#target_type>, value: #target_type) -> Result<(), Error> {
                        values.push(value);

                        Ok(())
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn deserializer_enum_variant_fn_handle(
        &self,
        ctx: &Context<'_, '_>,
        represents_element: bool,
        deserializer_ident: &Ident2,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);
        let store_ident = self.store_ident();
        let handler_ident = self.handler_ident();
        let variant_ident = &self.variant_ident;

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
        ]);

        let values = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single | Occurs::Optional => quote!(Option<#target_type>),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(Vec<#target_type>),
        };

        // Handler for `DeserializerArtifact::Data`
        let data_handler = match (represents_element, self.occurs, self.info.max_occurs) {
            (_, Occurs::None, _) => unreachable!(),
            // Return instantly if we have received the expected value
            (false, Occurs::Single | Occurs::Optional, _) => quote! {
                let data = #deserializer_ident::finish_state(reader, #deserializer_state_ident::#variant_ident(values, None))?;
                *self.state = #deserializer_state_ident::Done__(data);

                ElementHandlerOutput::Break {
                    event,
                    allow_any,
                }
            },
            // Finish the deserialization if the expected max value has been reached.
            // Continue if not.
            (false, Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => {
                quote! {
                    if values.len() < #max {
                        *self.state = #deserializer_state_ident::#variant_ident(values, None);

                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        let data = #deserializer_ident::finish_state(#deserializer_state_ident::#variant_ident(values, None))?;
                        *self.state = #deserializer_state_ident::Done__(data);

                        ElementHandlerOutput::Break {
                            event,
                            allow_any,
                        }
                    }
                }
            }
            // Value is unbound, continue in any case
            (_, _, _) => quote! {
                *self.state = #deserializer_state_ident::#variant_ident(values, None);

                ElementHandlerOutput::from_event(event, allow_any)
            },
        };

        // Handler for `DeserializerArtifact::Deserializer`
        let deserializer_handler = match (self.occurs, self.info.max_occurs) {
            (Occurs::None, _) => unreachable!(),
            // If we only expect one element we never initialize a new deserializer
            // we only continue the deserialization process for `End` events (because
            // they may finish this deserializer).
            (Occurs::Single | Occurs::Optional, _) => quote! {
                *self.state = #deserializer_state_ident::#variant_ident(values, Some(deserializer));

                ElementHandlerOutput::from_event_end(event, allow_any)
            },
            // If we expect multiple elements we only try to initialize a new
            // deserializer if the maximum has not been reached yet.
            // The `+1` is for the data that is contained in the yet unfinished
            // deserializer.
            (Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => {
                quote! {
                                    let can_have_more = values.len().saturating_add(1) < #max;
                                    let ret = if can_have_more {
                                        ElementHandlerOutput::from_event(event, allow_any)
                                    } else {
                                        ElementHandlerOutput::from_event_end(event, allow_any)
                                    };

                                    match (can_have_more, &ret) {
                                        (true, ElementHandlerOutput::Continue { .. })  => {
                                            fallback.get_or_insert(#deserializer_state_ident::#variant_ident(Default::default(), Some(deserializer)));

                                            *self.state = #deserializer_state_ident::#variant_ident(values, None);
                                        }
                                        (false, _ ) | (_, ElementHandlerOutput::Break { .. }) => {
                                            *self.state = #deserializer_state_ident::#variant_ident(values, Some(deserializer));
                                        }
                                    }
                h
                                    ret
                                }
            }
            // Unbound, we can try a new deserializer in any case.
            (Occurs::DynamicList | Occurs::StaticList(_), _) => quote! {
                let ret = ElementHandlerOutput::from_event(event, allow_any);

                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = #deserializer_state_ident::#variant_ident(values, Some(deserializer));
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(#deserializer_state_ident::#variant_ident(Default::default(), Some(deserializer)));

                        *self.state = #deserializer_state_ident::#variant_ident(values, None);
                    }
                }

                ret
            },
        };

        quote! {
            fn #handler_ident<'de, R>(
                &mut self,
                reader: &R,
                mut values: #values,
                output: DeserializerOutput<'de, #target_type>,
                fallback: &mut Option<#deserializer_state_ident>,
            ) -> Result<ElementHandlerOutput<'de>, Error>
            where
                R: DeserializeReader,
            {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;

                if artifact.is_none() {
                    *self.state = match fallback.take() {
                        None => #deserializer_state_ident::Init__,
                        Some(#deserializer_state_ident::#variant_ident(_, Some(deserializer))) => #deserializer_state_ident::#variant_ident(values, Some(deserializer)),
                        _ => unreachable!(),
                    };

                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }

                match fallback.take() {
                    None => (),
                    Some(#deserializer_state_ident::#variant_ident(_, Some(deserializer))) => {
                        let data = deserializer.finish(reader)?;
                        #deserializer_ident::#store_ident(&mut values, data)?;
                    }
                    Some(_) => unreachable!(),
                }

                Ok(match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        #deserializer_ident::#store_ident(&mut values, data)?;

                        #data_handler
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        #deserializer_handler
                    }
                })
            }
        }
    }

    fn deserializer_enum_variant_fn_next_continue(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let matcher = quote!(Some(deserializer));
        let output = quote!(deserializer.next(reader, event));

        self.deserializer_enum_variant_fn_next(ctx, &matcher, &output)
    }

    fn deserializer_enum_variant_fn_next_create(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::WithDeserializer)]);

        let matcher = quote!(None);
        let output = quote!(<#target_type as WithDeserializer>::Deserializer::init(reader, event));

        self.deserializer_enum_variant_fn_next(ctx, &matcher, &output)
    }

    fn deserializer_enum_variant_fn_next(
        &self,
        ctx: &Context<'_, '_>,
        matcher: &TokenStream,
        output: &TokenStream,
    ) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
        ]);

        quote! {
            (S::#variant_ident(values, #matcher), event) => {
                let output = #output?;

                match self.#handler_ident(reader, values, output, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                }
            },
        }
    }

    fn deserializer_struct_field_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let field_ident = &self.field_ident;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let target_type = match self.occurs {
            Occurs::Single | Occurs::Optional => quote!(Option<#target_type>),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(Vec<#target_type>),
            e => crate::unreachable!("{:?}", e),
        };

        quote! {
            #field_ident: #target_type,
        }
    }

    fn deserializer_struct_field_init(&self) -> TokenStream {
        let occurs = self.occurs;
        let field_ident = &self.field_ident;

        match occurs {
            Occurs::None => quote!(),
            Occurs::Single | Occurs::Optional => quote!(#field_ident: None,),
            Occurs::DynamicList | Occurs::StaticList(_) => quote!(#field_ident: Vec::new(),),
        }
    }

    fn deserializer_struct_field_init_element(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let handler_ident = self.handler_ident();
        let call_handler = quote!(self.#handler_ident(reader, output, &mut *fallback));

        self.deserializer_init_element(ctx, &call_handler)
    }

    fn deserializer_struct_field_init_group(
        &self,
        ctx: &Context<'_, '_>,
        handle_any: bool,
    ) -> Option<TokenStream> {
        let handler_ident = self.handler_ident();
        let call_handler = quote!(self.#handler_ident(reader, output, &mut *fallback));

        self.deserializer_init_group(ctx, handle_any, &call_handler)
    }

    fn deserializer_struct_field_finish_state_all(&self) -> TokenStream {
        let store_ident = self.store_ident();
        let variant_ident = &self.variant_ident;

        quote! {
            S::#variant_ident(deserializer) => self.#store_ident(deserializer.finish(reader)?)?,
        }
    }

    fn deserializer_struct_field_finish_state_sequence(&self) -> TokenStream {
        let store_ident = self.store_ident();
        let variant_ident = &self.variant_ident;

        quote! {
            S::#variant_ident(Some(deserializer)) => self.#store_ident(deserializer.finish(reader)?)?,
        }
    }

    fn deserializer_struct_field_finish(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let name = &self.s_name;
        let field_ident = &self.field_ident;
        let xsd_parser = &ctx.xsd_parser_crate;

        let convert = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => {
                ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

                let mut ctx = quote! {
                    self.#field_ident.ok_or_else(|| ErrorKind::MissingElement(#name.into()))?
                };

                if self.need_indirection {
                    ctx = quote! { Box::new(#ctx) };
                }

                ctx
            }
            Occurs::Optional if self.need_indirection => {
                quote! { self.#field_ident.map(Box::new) }
            }
            Occurs::Optional | Occurs::DynamicList => {
                quote! { self.#field_ident }
            }
            Occurs::StaticList(sz) => {
                ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::ErrorKind)]);

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

    fn deserializer_struct_field_fn_store(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let name = &self.b_name;
        let field_ident = &self.field_ident;
        let store_ident = self.store_ident();

        let body = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single | Occurs::Optional => {
                ctx.add_quick_xml_deserialize_usings([
                    quote!(#xsd_parser::quick_xml::ErrorKind),
                    quote!(#xsd_parser::quick_xml::RawByteStr),
                ]);

                quote! {
                    if self.#field_ident.is_some() {
                        Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(#name)))?;
                    }

                    self.#field_ident = Some(value);
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                self.#field_ident.push(value);
            },
        };

        ctx.add_quick_xml_deserialize_usings([quote!(#xsd_parser::quick_xml::Error)]);

        quote! {
            fn #store_ident(&mut self, value: #target_type) -> Result<(), Error> {
                #body

                Ok(())
            }
        }
    }

    fn deserializer_struct_field_fn_handle_all(
        &self,
        ctx: &Context<'_, '_>,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);
        let store_ident = self.store_ident();
        let handler_ident = self.handler_ident();
        let variant_ident = &self.variant_ident;

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Event),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::DeserializerArtifact),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
        ]);

        quote! {
            fn #handler_ident<'de, R>(
                &mut self,
                reader: &R,
                output: DeserializerOutput<'de, #target_type>,
                fallback: &mut Option<#deserializer_state_ident>,
            ) -> Result<ElementHandlerOutput<'de>, Error>
            where
                R: DeserializeReader,
            {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;

                if artifact.is_none() {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);

                    *self.state = match ret {
                        ElementHandlerOutput::Continue { .. } => #deserializer_state_ident::Next__,
                        ElementHandlerOutput::Break { .. } => fallback.take().unwrap_or(#deserializer_state_ident::Next__),
                    };

                    return Ok(ret);
                }

                if let Some(fallback) = fallback.take() {
                    self.finish_state(reader, fallback)?;
                }

                Ok(match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        self.#store_ident(data)?;

                        *self.state = #deserializer_state_ident::Next__;

                        ElementHandlerOutput::from_event(event, allow_any)
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        let ret = ElementHandlerOutput::from_event(event, allow_any);

                        match &ret {
                            ElementHandlerOutput::Continue { .. } => {
                                fallback.get_or_insert(#deserializer_state_ident::#variant_ident(deserializer));

                                *self.state = #deserializer_state_ident::Next__;
                            }
                            ElementHandlerOutput::Break { .. } => {
                                *self.state = #deserializer_state_ident::#variant_ident(deserializer);
                            }
                        }

                        ret
                    }
                })
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn deserializer_struct_field_fn_handle_sequence(
        &self,
        ctx: &Context<'_, '_>,
        next: Option<&ComplexTypeElement<'_>>,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        let xsd_parser = &ctx.xsd_parser_crate;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let store_ident = self.store_ident();
        let field_ident = &self.field_ident;
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xsd_parser::quick_xml::Error),
            quote!(#xsd_parser::quick_xml::DeserializeReader),
            quote!(#xsd_parser::quick_xml::DeserializerOutput),
            quote!(#xsd_parser::quick_xml::ElementHandlerOutput),
        ]);

        let next_state = if let Some(next) = next {
            let variant_ident = &next.variant_ident;

            quote!(#deserializer_state_ident::#variant_ident(None))
        } else {
            quote!(#deserializer_state_ident::Done__)
        };

        // Handler for `DeserializerArtifact::None`: Should only be the
        // case if we try to initialize a new deserializer.
        let handler_none = match (self.occurs, self.info.min_occurs) {
            (Occurs::None, _) => unreachable!(),
            // If we do not expect any data we continue with the next state
            (_, 0) | (Occurs::Optional, _) => quote! {
                fallback.get_or_insert(#deserializer_state_ident::#variant_ident(None));

                *self.state = #next_state;

                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            },
            // If we got the expected data, we move on, otherwise we stay in the
            // current state and break.
            (Occurs::Single, _) => quote! {
                if self.#field_ident.is_some() {
                    fallback.get_or_insert(#deserializer_state_ident::#variant_ident(None));

                    *self.state = #next_state;

                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = #deserializer_state_ident::#variant_ident(None);

                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            },
            // If we did not reach the expected amount of data, we stay in the
            // current state and break, otherwise we continue with the next state.
            (Occurs::DynamicList | Occurs::StaticList(_), min) => quote! {
                if self.#field_ident.len() < #min {
                    *self.state = #deserializer_state_ident::#variant_ident(None);

                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(#deserializer_state_ident::#variant_ident(None));

                    *self.state = #next_state;

                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            },
        };

        // Handler for `DeserializerArtifact::Data`:
        let data_handler = match (self.occurs, self.info.max_occurs) {
            // If we got some data we simple move one to the next element
            (Occurs::Single | Occurs::Optional, _) => quote! {
                *self.state = #next_state;
            },
            // If we got some data and the maximum amount of elements of this
            // type is reached we move on, otherwise we stay in the current state.
            (Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => quote! {
                if self.#field_ident.len() < #max {
                    *self.state = #deserializer_state_ident::#variant_ident(None);
                } else {
                    *self.state = #next_state;
                }
            },
            // Unbounded amount. Stay in the current state in any case.
            (_, _) => quote! {
                *self.state = #deserializer_state_ident::#variant_ident(None);
            },
        };

        // Handler for `DeserializerArtifact::Deserializer:
        let min = self.info.min_occurs;
        let deserializer_handler = match self.occurs {
            // If we expect only one element we continue to the next state,
            // because the old yet unfinished deserializer already contains
            // this data.
            Occurs::Single | Occurs::Optional => quote! {
                *self.state = #next_state;
            },
            // If we have enough space for more data of the same element, we stay
            // inside the state, otherwise we continue with the next one.
            // The `+1` is for the data that is contained in the yet unfinished
            // deserializer.
            Occurs::DynamicList | Occurs::StaticList(_) if min > 0 => quote! {
                if self.#field_ident.len().saturating_add(1) < #min {
                    *self.state = #deserializer_state_ident::#variant_ident(None);
                } else {
                    *self.state = #next_state;
                }
            },
            // Infinit amount of data: Stay in the current state.
            _ => quote! {
                *self.state = #deserializer_state_ident::#variant_ident(None);
            },
        };

        quote! {
            fn #handler_ident<'de, R>(
                &mut self,
                reader: &R,
                output: DeserializerOutput<'de, #target_type>,
                fallback: &mut Option<#deserializer_state_ident>,
            ) -> Result<ElementHandlerOutput<'de>, Error>
            where
                R: DeserializeReader,
            {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;

                if artifact.is_none() {
                    #handler_none
                }

                if let Some(fallback) = fallback.take() {
                    self.finish_state(reader, fallback)?;
                }

                Ok(match artifact {
                    DeserializerArtifact::None => unreachable!(),
                    DeserializerArtifact::Data(data) => {
                        self.#store_ident(data)?;

                        #data_handler

                        ElementHandlerOutput::from_event(event, allow_any)
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        let ret = ElementHandlerOutput::from_event(event, allow_any);

                        match &ret {
                            ElementHandlerOutput::Continue { .. } => {
                                fallback.get_or_insert(#deserializer_state_ident::#variant_ident(Some(deserializer)));

                                #deserializer_handler
                            }
                            ElementHandlerOutput::Break { .. } => {
                                *self.state = #deserializer_state_ident::#variant_ident(Some(deserializer));
                            }
                        }

                        ret
                    }
                })
            }
        }
    }

    fn deserializer_struct_field_fn_next_all(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let xad_parser = &ctx.xsd_parser_crate;
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xad_parser::quick_xml::DeserializerOutput),
            quote!(#xad_parser::quick_xml::ElementHandlerOutput),
            quote!(#xad_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            (
                S::#variant_ident(deserializer),
                event
            ) => {
                let output = deserializer.next(reader, event)?;
                match self.#handler_ident(reader, output, &mut fallback)? {
                    ElementHandlerOutput::Continue { event, .. } => event,
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                }
            }
        }
    }

    fn deserializer_struct_field_fn_next_sequence_continue(
        &self,
        ctx: &Context<'_, '_>,
    ) -> TokenStream {
        let xad_parser = &ctx.xsd_parser_crate;
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();

        ctx.add_quick_xml_deserialize_usings([
            quote!(#xad_parser::quick_xml::DeserializerOutput),
            quote!(#xad_parser::quick_xml::ElementHandlerOutput),
            quote!(#xad_parser::quick_xml::DeserializerArtifact),
        ]);

        quote! {
            (
                S::#variant_ident(Some(deserializer)),
                event
            ) => {
                let output = deserializer.next(reader, event)?;
                match self.#handler_ident(reader, output, &mut fallback)? {
                    ElementHandlerOutput::Continue { event, allow_any } => {
                        allow_any_element = allow_any_element || allow_any;

                        event
                    },
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                }
            }
        }
    }

    fn deserializer_struct_field_fn_next_sequence_create(
        &self,
        ctx: &Context<'_, '_>,
        next: Option<&ComplexTypeElement<'_>>,
        allow_any: bool,
    ) -> TokenStream {
        let name = &self.b_name;
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let next_state = if let Some(next) = next {
            let variant_ident = &next.variant_ident;

            quote!(S::#variant_ident(None))
        } else {
            quote!(S::Done__)
        };

        let allow_any = allow_any || self.target_type_allows_any(ctx.types);
        let allow_any = allow_any.then(|| {
            quote! {
                allow_any_element = true;
                fallback.get_or_insert(S::#variant_ident(None));
            }
        });

        let mut body = quote! {
            let output = <#target_type as WithDeserializer>::Deserializer::init(reader, event)?;
            match self.#handler_ident(reader, output, &mut fallback)? {
                ElementHandlerOutput::Continue { event, allow_any } => {
                    allow_any_element = allow_any_element || allow_any;

                    event
                },
                ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
            }
        };

        let need_name_matcher =
            !self.target_is_dynamic && self.info.element_mode == ElementMode::Element;
        if need_name_matcher {
            let ns_name = self
                .info
                .ident
                .ns
                .as_ref()
                .and_then(|ns| ctx.types.modules.get(ns))
                .map(|module| ctx.resolve_type_for_deserialize_module(&module.make_ns_const()))
                .map_or_else(|| quote!(None), |ns_name| quote!(Some(&#ns_name)));

            body = quote! {
                if reader.check_start_tag_name(&event, #ns_name, #name) {
                    #body
                } else {
                    *self.state = #next_state;

                    #allow_any

                    event
                }
            }
        }

        quote! {
            (
                S::#variant_ident(None),
                event @ (Event::Start(_) | Event::Empty(_))
            ) => {
                #body
            }
        }
    }
}

fn do_box(is_boxed: bool, tokens: TokenStream) -> TokenStream {
    if is_boxed {
        quote!(Box::new(#tokens))
    } else {
        tokens
    }
}

fn boxed_deserializer_ident(is_boxed: bool, deserializer_ident: &Ident2) -> Ident2 {
    if is_boxed {
        deserializer_ident.clone()
    } else {
        format_ident!("Self")
    }
}
