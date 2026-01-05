#![allow(clippy::redundant_closure_for_method_calls)]

use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::ops::Not;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote};

use crate::config::TypedefMode;
use crate::models::{
    data::{
        ComplexBase, ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement,
        ComplexDataEnum, ComplexDataStruct, ConstrainsData, DataTypeVariant, DerivedType,
        DynamicData, EnumerationData, EnumerationTypeVariant, Occurs, ReferenceData, SimpleData,
        StructMode, UnionData, UnionTypeVariant,
    },
    meta::{
        ComplexMeta, ElementMeta, ElementMetaVariant, ElementMode, MetaTypeVariant, MetaTypes,
        WhiteSpace,
    },
    schema::{xs::Use, MaxOccurs},
    TypeIdent,
};

use super::super::super::{
    context::{Context, ValueKey},
    RenderStep, RenderStepType,
};

/// Implements a [`RenderStep`] that renders the code for the `quick_xml` deserialization.
#[derive(Debug, Clone, Copy)]
pub struct QuickXmlDeserializeRenderStep {
    /// Whether to box the deserializer or not.
    ///
    /// Boxing the deserializer will reduce the stack usage, but may decrease
    /// the performance.
    pub boxed_deserializer: bool,
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
        $ctx.resolve_quick_xml_deserialize_ident_path($path)
    };
}

struct DeserializerConfig;

impl ValueKey for DeserializerConfig {
    type Type = QuickXmlDeserializeRenderStep;
}

impl RenderStep for QuickXmlDeserializeRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::ExtraImpls
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        ctx.set::<DeserializerConfig>(*self);

        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) | DataTypeVariant::Custom(_) => (),
            DataTypeVariant::Union(x) => x.render_deserializer(ctx),
            DataTypeVariant::Dynamic(x) => x.render_deserializer(ctx),
            DataTypeVariant::Reference(x) => x.render_deserializer(ctx),
            DataTypeVariant::Enumeration(x) => x.render_deserializer(ctx),
            DataTypeVariant::Simple(x) => x.render_deserializer(ctx),
            DataTypeVariant::Complex(x) => x.render_deserializer(ctx),
        }

        ctx.unset::<DeserializerConfig>();
    }
}

/* UnionData */

impl UnionData<'_> {
    pub(crate) fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            ..
        } = self;

        let validation = self.constrains.render_validation(ctx);
        let variants = variants
            .iter()
            .map(|var| var.render_deserializer_variant(ctx, validation.is_some()));

        let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");
        let u8_ = resolve_build_in!(ctx, "::core::primitive::u8");
        let result = resolve_build_in!(ctx, "::core::result::Result");

        let error = resolve_ident!(ctx, "xsd_parser_types::quick_xml::Error");
        let error_kind = resolve_ident!(ctx, "xsd_parser_types::quick_xml::ErrorKind");
        let deserialize_bytes =
            resolve_ident!(ctx, "xsd_parser_types::quick_xml::DeserializeBytes");
        let deserialize_helper =
            resolve_ident!(ctx, "xsd_parser_types::quick_xml::DeserializeHelper");

        let code = quote! {
            impl #deserialize_bytes for #type_ident {
                fn deserialize_bytes(
                    helper: &mut #deserialize_helper,
                    bytes: &[#u8_],
                ) -> #result<Self, #error> {
                    let mut errors = #vec::new();

                    #validation

                    #( #variants )*

                    Err(#error::from(#error_kind::InvalidUnion(errors.into())))
                }
            }
        };

        ctx.current_module().append(code);
    }
}

impl UnionTypeVariant<'_> {
    fn render_deserializer_variant(&self, ctx: &Context<'_, '_>, as_str: bool) -> TokenStream {
        let Self {
            variant_ident,
            target_type,
            ..
        } = self;

        let target_type = ctx.resolve_type_for_module(target_type);

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        if as_str {
            quote! {
                match #target_type::deserialize_str(helper, s) {
                    Ok(value) => return Ok(Self::#variant_ident(value)),
                    Err(error) => errors.push(#box_::new(error)),
                }
            }
        } else {
            quote! {
                match #target_type::deserialize_bytes(helper, bytes) {
                    Ok(value) => return Ok(Self::#variant_ident(value)),
                    Err(error) => errors.push(#box_::new(error)),
                }
            }
        }
    }
}

/* DynamicData */

impl DynamicData<'_> {
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

        let config = ctx.get_ref::<DeserializerConfig>();
        let deserializer_type = if config.boxed_deserializer {
            let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

            quote!(#box_<quick_xml_deserialize::#deserializer_ident>)
        } else {
            quote!(quick_xml_deserialize::#deserializer_ident)
        };

        let with_deserializer =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");

        let code = quote! {
            impl #with_deserializer for #type_ident {
                type Deserializer = #deserializer_type;
            }
        };

        ctx.current_module().append(code);
    }

    fn render_deserializer_types(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            derived_types,
            deserializer_ident,
            ..
        } = self;

        let variants = derived_types.iter().map(|x| {
            let target_type = ctx.resolve_type_for_deserialize_module(&x.target_type);
            let variant_ident = &x.variant_ident;

            let with_deserializer =
                resolve_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");

            quote! {
                #variant_ident(<#target_type as #with_deserializer>::Deserializer),
            }
        });

        let code = quote! {
            #[derive(Debug)]
            pub enum #deserializer_ident {
                #( #variants )*
            }
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_impls(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            derived_types,
            deserializer_ident,
            ..
        } = self;

        let result = resolve_build_in!(ctx, "::core::result::Result");

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Deserializer");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");
        let deserializer_event =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerEvent");
        let deserializer_result =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerResult");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");

        let config = ctx.get_ref::<DeserializerConfig>();
        let deserializer_type = if config.boxed_deserializer {
            let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

            quote!(#box_<#deserializer_ident>)
        } else {
            quote!(#deserializer_ident)
        };
        let boxed_deserializer_ident =
            boxed_deserializer_ident(config.boxed_deserializer, deserializer_ident);
        let deref_self = config.boxed_deserializer.then(|| quote!(*));

        let variants_init = derived_types
            .iter()
            .map(|x| x.render_deserializer_init(ctx, type_ident, deserializer_ident));
        let variants_next = derived_types
            .iter()
            .map(|x| x.render_deserializer_next(ctx, type_ident, deserializer_ident));
        let variants_finish = derived_types.iter().map(|x| {
            let variant_ident = &x.variant_ident;

            let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");
            ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

            quote! {
                #boxed_deserializer_ident::#variant_ident(x) => Ok(super::#type_ident(#box_::new(x.finish(helper)?))),
            }
        });

        let code = quote! {
            impl<'de> #deserializer<'de, super::#type_ident> for #deserializer_type {
                fn init(
                    helper: &mut #deserialize_helper,
                    event: #event<'de>,
                ) -> #deserializer_result<'de, super::#type_ident> {
                    let Some(type_name) = helper.get_dynamic_type_name(&event)? else {
                        return Ok(#deserializer_output {
                            artifact: #deserializer_artifact::None,
                            event: #deserializer_event::None,
                            allow_any: false,
                        });
                    };
                    let type_name = type_name.into_owned();

                    #( #variants_init )*

                    Ok(#deserializer_output {
                        artifact: #deserializer_artifact::None,
                        event: #deserializer_event::Break(event),
                        allow_any: false,
                    })
                }

                fn next(
                    self,
                    helper: &mut #deserialize_helper,
                    event: #event<'de>
                ) -> #deserializer_result<'de, super::#type_ident> {
                    match #deref_self self {
                        #( #variants_next )*
                    }
                }

                fn finish(
                    self,
                    helper: &mut #deserialize_helper
                ) -> #result<super::#type_ident, #error> {
                    match #deref_self self {
                        #( #variants_finish )*
                    }
                }
            }
        };

        ctx.quick_xml_deserialize().append(code);
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

        let config = ctx.get_ref::<DeserializerConfig>();
        let boxed_deserializer_ident =
            boxed_deserializer_ident(config.boxed_deserializer, deserializer_ident);
        let deserialize_mapper = ctx.do_box(
            config.boxed_deserializer,
            quote!(#boxed_deserializer_ident::#variant_ident(x)),
        );
        let target_type = ctx.resolve_type_for_deserialize_module(target_type);

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        let qname = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::QName");
        let with_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");

        ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

        let body = quote! {
            let #deserializer_output {
                artifact,
                event,
                allow_any,
            } = <#target_type as #with_deserializer>::init(helper, event)?;

            return Ok(#deserializer_output {
                artifact: artifact.map(
                    |x| super::#type_ident(#box_::new(x)),
                    |x| #deserialize_mapper,
                ),
                event,
                allow_any,
            });
        };

        if let Some(path) = ctx
            .types
            .meta
            .types
            .modules
            .get(&ident.ns)
            .and_then(|x| x.make_ns_const())
        {
            let ns_name = ctx.resolve_type_for_deserialize_module(&path);

            quote! {
                if matches!(helper.resolve_local_name(#qname(&type_name), &#ns_name), Some(#b_name)) {
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

        let config = ctx.get_ref::<DeserializerConfig>();
        let boxed_deserializer_ident =
            boxed_deserializer_ident(config.boxed_deserializer, deserializer_ident);
        let deserialize_mapper = ctx.do_box(
            config.boxed_deserializer,
            quote!(#boxed_deserializer_ident::#variant_ident(x)),
        );

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");

        quote! {
            #boxed_deserializer_ident::#variant_ident(x) => {
                let #deserializer_output {
                    artifact,
                    event,
                    allow_any,
                } = x.next(helper, event)?;

                Ok(#deserializer_output {
                    artifact: artifact.map(
                        |x| super::#type_ident(#box_::new(x)),
                        |x| #deserialize_mapper,
                    ),
                    event,
                    allow_any,
                })
            },
        }
    }
}

/* ReferenceData */

impl ReferenceData<'_> {
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

        let result = resolve_build_in!(ctx, "::core::result::Result");

        let body = match occurs {
            Occurs::None => return,
            Occurs::Single => {
                quote! {
                    Ok(Self(#target_type::deserialize_bytes(helper, bytes)?))
                }
            }
            Occurs::Optional => {
                quote! {
                    Ok(Self(Some(#target_type::deserialize_bytes(helper, bytes)?)))
                }
            }
            Occurs::DynamicList => {
                quote! {
                    Ok(Self(helper.deserialize_list(bytes)?))
                }
            }
            Occurs::StaticList(_) => {
                quote! {
                    Ok(Self(helper.deserialize_arr(bytes)?))
                }
            }
        };

        let u8_ = resolve_build_in!(ctx, "::core::primitive::u8");

        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserialize_bytes =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeBytes");
        let deserialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");

        let code = quote! {
            impl #deserialize_bytes for #type_ident {
                fn deserialize_bytes(
                    helper: &mut #deserialize_helper,
                    bytes: &[#u8_],
                ) -> #result<Self, #error> {
                    #body
                }
            }
        };

        ctx.current_module().append(code);
    }
}

/* EnumerationData */

impl EnumerationData<'_> {
    pub(crate) fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            variants,
            ..
        } = self;

        let mut other = None;
        let validation = self.constrains.render_validation(ctx);
        let variants = variants
            .iter()
            .filter_map(|v| v.render_deserializer_variant(ctx, &mut other))
            .collect::<Vec<_>>();

        let u8_ = resolve_build_in!(ctx, "::core::primitive::u8");
        let result = resolve_build_in!(ctx, "::core::result::Result");

        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let error_kind = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::ErrorKind");
        let raw_byte_str = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::RawByteStr");
        let deserialize_bytes =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeBytes");
        let deserialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");

        let other = other.unwrap_or_else(|| {
            quote! {
                x => Err(
                    #error::from(
                        #error_kind::UnknownOrInvalidValue(
                            #raw_byte_str::from_slice(x)
                        )
                    )
                ),
            }
        });

        let code = quote! {
            impl #deserialize_bytes for #type_ident {
                fn deserialize_bytes(
                    helper: &mut #deserialize_helper,
                    bytes: &[#u8_],
                ) -> #result<Self, #error> {
                    #validation

                    match bytes {
                        #( #variants )*
                        #other
                    }
                }
            }
        };

        ctx.current_module().append(code);
    }
}

impl EnumerationTypeVariant<'_> {
    fn render_deserializer_variant(
        &self,
        ctx: &Context<'_, '_>,
        other: &mut Option<TokenStream>,
    ) -> Option<TokenStream> {
        let Self {
            b_name,
            target_type,
            variant_ident,
            ..
        } = self;

        if let Some(target_type) = target_type {
            let target_type = ctx.resolve_type_for_module(target_type);

            *other = Some(
                quote! { x => Ok(Self::#variant_ident(#target_type::deserialize_bytes(helper, x)?)), },
            );

            return None;
        }

        Some(quote! {
            #b_name => Ok(Self::#variant_ident),
        })
    }
}

impl SimpleData<'_> {
    pub(crate) fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            occurs,
            type_ident,
            target_type,
            ..
        } = self;

        let target_type = ctx.resolve_type_for_module(target_type);

        let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");
        let u8_ = resolve_build_in!(ctx, "::core::primitive::u8");
        let result = resolve_build_in!(ctx, "::core::result::Result");

        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserialize_bytes =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeBytes");
        let deserialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");

        let validation = self.constrains.render_validation(ctx);

        let body = match (validation.is_some(), occurs) {
            (true, Occurs::Single) => {
                quote! {
                    let inner = #target_type::deserialize_str(helper, s)?;
                }
            }
            (false, Occurs::Single) => {
                quote! {
                    let inner = #target_type::deserialize_bytes(helper, bytes)?;
                }
            }
            (false, Occurs::DynamicList) => {
                quote! {
                    let inner = bytes
                        .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                        .map(|bytes| #target_type::deserialize_bytes(helper, bytes))
                        .collect::<#result<#vec<_>, _>>()?;
                }
            }
            (need_str, occurs) => {
                unreachable!("Invalid (`need_str`, `occurs`) combination: ({need_str}, {occurs:?})")
            }
        };

        let code = quote! {
            impl #deserialize_bytes for #type_ident {
                fn deserialize_bytes(
                    helper: &mut #deserialize_helper,
                    bytes: &[#u8_],
                ) -> #result<Self, #error> {
                    #validation

                    #body

                    Ok(Self::new(inner).map_err(|error| (bytes, error))?)
                }
            }
        };

        ctx.current_module().append(code);
    }
}

/* ConstrainsData */

impl ConstrainsData<'_> {
    fn render_validation(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let whitespace = self.render_whitespace(ctx);
        let validate_str = self.render_validate_str();

        if whitespace.is_some() || validate_str.is_some() {
            let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
            let from_utf8 = resolve_ident!(ctx, "::core::str::from_utf8");

            Some(quote! {
                let s = #from_utf8(bytes).map_err(#error::from)?;

                #whitespace
                #validate_str
            })
        } else {
            None
        }
    }

    fn render_whitespace(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        match &self.meta.whitespace {
            WhiteSpace::Preserve => None,
            WhiteSpace::Replace => {
                let whitespace_replace =
                    resolve_ident!(ctx, "::xsd_parser_types::quick_xml::whitespace_replace");

                Some(quote! {
                    let buffer = #whitespace_replace(s);
                    let s = buffer.as_str();
                })
            }
            WhiteSpace::Collapse => {
                let whitespace_collapse =
                    resolve_ident!(ctx, "::xsd_parser_types::quick_xml::whitespace_collapse");

                Some(quote! {
                    let buffer = #whitespace_collapse(s);
                    let s = buffer.trim();
                })
            }
        }
    }

    fn render_validate_str(&self) -> Option<TokenStream> {
        self.meta.need_string_validation().then(|| {
            quote! {
                Self::validate_str(s).map_err(|error| (bytes, error))?;
            }
        })
    }
}

/* ComplexData */

impl ComplexData<'_> {
    pub(crate) fn render_deserializer(&self, ctx: &mut Context<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_deserializer(ctx);

                if let Some(content_type) = content_type {
                    ctx.set_content(true);
                    content_type.render_deserializer(ctx);
                    ctx.set_content(false);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_deserializer(ctx);

                if let Some(content_type) = content_type {
                    ctx.set_content(true);
                    content_type.render_deserializer(ctx);
                    ctx.set_content(false);
                }
            }
        }
    }
}

impl ComplexBase<'_> {
    fn return_end_event(&self, ctx: &Context<'_, '_>) -> (TokenStream, TokenStream) {
        let deserializer_event =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerEvent");

        if self.represents_element() {
            (quote!(), quote!(#deserializer_event::None))
        } else {
            (
                quote!(event @),
                quote!(#deserializer_event::Continue(event)),
            )
        }
    }

    fn render_with_deserializer(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            deserializer_ident,
            ..
        } = self;

        let config = ctx.get_ref::<DeserializerConfig>();
        let deserializer_type = if config.boxed_deserializer {
            let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

            quote!(#box_<quick_xml_deserialize::#deserializer_ident>)
        } else {
            quote!(quick_xml_deserialize::#deserializer_ident)
        };

        let with_deserializer =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");

        let code = quote! {
            impl #with_deserializer for #type_ident {
                type Deserializer = #deserializer_type;
            }
        };

        ctx.current_module().append(code);
    }

    fn render_deserializer_impl(
        &self,
        ctx: &mut Context<'_, '_>,
        fn_init: &TokenStream,
        fn_next: &TokenStream,
        fn_finish: &TokenStream,
        finish_mut_self: bool,
    ) {
        let type_ident = &self.type_ident;
        let deserializer_ident = &self.deserializer_ident;
        let config = ctx.get_ref::<DeserializerConfig>();
        let deserializer_type = if config.boxed_deserializer {
            let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

            quote!(#box_<#deserializer_ident>)
        } else {
            quote!(#deserializer_ident)
        };
        let mut_ = finish_mut_self.then(|| quote!(mut));

        let result = resolve_build_in!(ctx, "::core::result::Result");

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Deserializer");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");
        let deserializer_result =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerResult");

        let code = quote! {
            impl<'de> #deserializer<'de, super::#type_ident> for #deserializer_type {
                fn init(
                    helper: &mut #deserialize_helper,
                    event: #event<'de>,
                ) -> #deserializer_result<'de, super::#type_ident> {
                    #fn_init
                }

                fn next(
                    mut self,
                    helper: &mut #deserialize_helper,
                    event: #event<'de>,
                ) -> #deserializer_result<'de, super::#type_ident> {
                    #fn_next
                }

                fn finish(#mut_ self, helper: &mut #deserialize_helper) -> #result<super::#type_ident, #error> {
                    #fn_finish
                }
            }
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_fn_init_for_element(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let _ctx = ctx;
        let deserializer_ident = &self.deserializer_ident;
        let config = ctx.get_ref::<DeserializerConfig>();
        let boxed_deserializer_ident =
            boxed_deserializer_ident(config.boxed_deserializer, deserializer_ident);

        quote! {
            helper.init_deserializer_from_start_event(event, #boxed_deserializer_ident::from_bytes_start)
        }
    }
}

impl ComplexDataEnum<'_> {
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

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        let code = quote! {
            #[derive(Debug)]
            pub struct #deserializer_ident {
                state__: #box_<#deserializer_state_ident>,
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
        let config = ctx.get::<DeserializerConfig>();
        let is_defaultable = ctx.is_defaultable_type();

        let represents_element = self.represents_element();
        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let fn_find_suitable = self.render_deserializer_fn_find_suitable(ctx);
        let fn_from_bytes_start =
            represents_element.then(|| self.render_deserializer_fn_from_bytes_start(ctx));
        let fn_finish_state = self.render_deserializer_fn_finish_state(ctx);

        let store_elements = self
            .elements
            .iter()
            .map(|x| x.deserializer_enum_variant_fn_store(ctx))
            .collect::<Vec<_>>();
        let handle_elements = self
            .elements
            .iter()
            .map(|x| {
                x.deserializer_enum_variant_fn_handle(
                    ctx,
                    represents_element,
                    &boxed_deserializer_ident(config.boxed_deserializer, deserializer_ident),
                    deserializer_state_ident,
                )
            })
            .collect::<Vec<_>>();

        let impl_default = is_defaultable.then(|| self.render_deserializer_impl_default(ctx));

        let code = quote! {
            impl #deserializer_ident {
                #fn_find_suitable
                #fn_from_bytes_start
                #fn_finish_state

                #( #store_elements )*
                #( #handle_elements )*
            }

            #impl_default
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_fn_find_suitable(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let allow_any = self.allow_any;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let result = resolve_build_in!(ctx, "::core::result::Result");

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        let text = self
            .elements
            .iter()
            .find_map(|x| x.deserializer_enum_variant_init_text(ctx));
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
        let any = self
            .elements
            .iter()
            .filter_map(|x| x.deserializer_enum_variant_init_any(ctx))
            .collect::<Vec<_>>();

        let x = if elements.is_empty() {
            quote!(_)
        } else {
            quote!(x)
        };

        let event_decl = (!groups.is_empty() || !any.is_empty() || text.is_some())
            .then(|| quote!(let mut event = event;));
        let (allow_any_result, allow_any_decl) = if groups.is_empty() || allow_any {
            (quote!(#allow_any), None)
        } else {
            (
                quote!(allow_any_element),
                Some(quote!(let mut allow_any_element = false;)),
            )
        };

        quote! {
            fn find_suitable<'de>(
                &mut self,
                helper: &mut #deserialize_helper,
                event: #event<'de>,
            ) -> #result<#element_handler_output<'de>, #error> {
                #event_decl
                #allow_any_decl

                if let #event::Start(#x) | #event::Empty(#x) = &event {
                    #( #elements )*
                    #( #groups )*
                    #( #any )*
                }

                #text

                *self.state__ = #deserializer_state_ident::Init__;

                Ok(#element_handler_output::return_to_parent(event, #allow_any_result))
            }
        }
    }

    fn render_deserializer_fn_from_bytes_start(&self, ctx: &mut Context<'_, '_>) -> TokenStream {
        let config = ctx.get::<DeserializerConfig>();
        let is_defaultable = ctx.is_defaultable_type();

        let deserializer_state_ident = &self.deserializer_state_ident;

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");
        let result = resolve_build_in!(ctx, "::core::result::Result");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let bytes_start =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::BytesStart");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");

        let self_type = if config.boxed_deserializer {
            quote!(#box_<Self>)
        } else {
            quote!(Self)
        };

        let self_ctor = if is_defaultable {
            quote!(Self::default())
        } else {
            quote! {
                Self {
                    state__: #box_::new(#deserializer_state_ident::Init__)
                }
            }
        };
        let self_ctor = ctx.do_box(config.boxed_deserializer, self_ctor);

        let attrib_loop = self.allow_any_attribute.not().then(|| {
            quote! {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
        });

        quote! {
            fn from_bytes_start(
                helper: &mut #deserialize_helper,
                bytes_start: &#bytes_start<'_>
            ) -> #result<#self_type, #error> {
                #attrib_loop

                Ok(#self_ctor)
            }
        }
    }

    fn render_deserializer_fn_finish_state(&self, ctx: &mut Context<'_, '_>) -> TokenStream {
        let config = ctx.get::<DeserializerConfig>();
        let is_defaultable = ctx.is_defaultable_type();

        let type_ident = &self.type_ident;
        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let result = resolve_build_in!(ctx, "::core::result::Result");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let error_kind = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ErrorKind");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");

        let finish_elements = self
            .elements
            .iter()
            .map(|x| {
                x.deserializer_enum_variant_finish(
                    ctx,
                    type_ident,
                    &boxed_deserializer_ident(config.boxed_deserializer, deserializer_ident),
                )
            })
            .collect::<Vec<_>>();

        let finish_init = if is_defaultable {
            self.elements[0].deserializer_enum_variant_default(ctx, type_ident)
        } else {
            quote!(Err(#error_kind::MissingContent.into()))
        };

        quote! {
            fn finish_state(helper: &mut #deserialize_helper, state: #deserializer_state_ident) -> #result<super::#type_ident, #error> {
                use #deserializer_state_ident as S;

                match state {
                    S::Init__ => #finish_init,
                    #( #finish_elements )*
                    S::Done__(data) => Ok(data),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn render_deserializer_impl_default(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");
        let default = resolve_build_in!(ctx, "::core::default::Default");

        quote! {
            impl #default for #deserializer_ident {
                fn default() -> Self {
                    Self {
                        state__: #box_::new(#deserializer_state_ident::Init__),
                    }
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

    fn render_deserializer_fn_init(&self, ctx: &mut Context<'_, '_>) -> TokenStream {
        if self.represents_element() {
            self.render_deserializer_fn_init_for_element(ctx)
        } else {
            self.render_deserializer_fn_init_for_group(ctx)
        }
    }

    fn render_deserializer_fn_init_for_group(&self, ctx: &mut Context<'_, '_>) -> TokenStream {
        let config = ctx.get::<DeserializerConfig>();
        let is_defaultable = ctx.is_defaultable_type();

        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");

        let boxed_deserializer_ident =
            boxed_deserializer_ident(config.boxed_deserializer, deserializer_ident);
        let init_deserializer = if is_defaultable {
            quote! { #boxed_deserializer_ident::default() }
        } else {
            quote! {
                #boxed_deserializer_ident {
                    state__: #box_::new(#deserializer_state_ident::Init__),
                }
            }
        };
        let init_deserializer = ctx.do_box(config.boxed_deserializer, init_deserializer);

        quote! {
            let deserializer = #init_deserializer;
            let mut output = deserializer.next(helper, event)?;

            output.artifact = match output.artifact {
                #deserializer_artifact::Deserializer(x) if matches!(&*x.state__, #deserializer_state_ident::Init__) => #deserializer_artifact::None,
                artifact => artifact,
            };

            Ok(output)
        }
    }

    fn render_deserializer_fn_next(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let config = ctx.get_ref::<DeserializerConfig>();

        let deserializer_ident =
            boxed_deserializer_ident(config.boxed_deserializer, &self.deserializer_ident);
        let deserializer_state_ident = &self.deserializer_state_ident;
        let (event_at, return_end_event) = self.return_end_event(ctx);

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let replace = resolve_quick_xml_ident!(ctx, "::core::mem::replace");
        let deserializer_event =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerEvent");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

        let handlers_continue = self
            .elements
            .iter()
            .map(|x| x.deserializer_enum_variant_fn_next_continue(ctx));
        let handlers_create = self
            .elements
            .iter()
            .map(|x| x.deserializer_enum_variant_fn_next_create(ctx));

        let handler_mixed = self.base.is_mixed().then(|| {
            quote! {
                (state, #event::Text(_) | #event::CData(_)) => {
                    *self.state__ = state;
                    break (#deserializer_event::None, false);
                }
            }
        });

        quote! {
            use #deserializer_state_ident as S;

            let mut event = event;

            let (event, allow_any) = loop {
                let state = #replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    #( #handlers_continue )*
                    (state, #event_at #event::End(_)) => {
                        return Ok(#deserializer_output {
                            artifact: #deserializer_artifact::Data(#deserializer_ident::finish_state(helper, state)?),
                            event: #return_end_event,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        #element_handler_output::Break { event, allow_any } => break (event, allow_any),
                        #element_handler_output::Continue { event, .. } => event,
                    },
                    #( #handlers_create )*
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;

                        break (#deserializer_event::Continue(event), false);
                    },
                    #handler_mixed
                    (state, event) => {
                        *self.state__ = state;
                        break (#deserializer_event::Continue(event), false);
                    }
                }
            };

            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                #deserializer_artifact::Data(self.finish(helper)?)
            } else {
                #deserializer_artifact::Deserializer(self)
            };

            Ok(#deserializer_output {
                artifact,
                event,
                allow_any,
            })
        }
    }

    fn render_deserializer_fn_finish(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let config = ctx.get_ref::<DeserializerConfig>();
        let deserializer_ident =
            boxed_deserializer_ident(config.boxed_deserializer, &self.deserializer_ident);

        quote! {
            #deserializer_ident::finish_state(helper, *self.state__)
        }
    }
}

impl ComplexDataStruct<'_> {
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

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        let code = quote! {
            #[derive(Debug)]
            pub struct #deserializer_ident {
                #( #attributes )*
                #( #elements )*
                #content
                state__: #box_<#deserializer_state_ident>,
            }
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_state_type(&self, ctx: &mut Context<'_, '_>) {
        let deserializer_state_ident = &self.deserializer_state_ident;

        let variants = match &self.mode {
            StructMode::Empty { .. } => {
                quote! {
                    Init__,
                    Unknown__,
                }
            }
            StructMode::Content { content } => {
                let target_type = ctx.resolve_type_for_deserialize_module(&content.target_type);

                let with_deserializer = resolve_quick_xml_ident!(
                    ctx,
                    "::xsd_parser_types::quick_xml::WithDeserializer"
                );

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
                    Content__(<#target_type as #with_deserializer>::Deserializer),
                    #done
                    Unknown__,
                }
            }
            StructMode::All { elements, .. } => {
                let variants = elements.iter().map(|element| {
                    let variant_ident = &element.variant_ident;
                    let target_type = ctx.resolve_type_for_deserialize_module(&element.target_type);
                    let with_deserializer = resolve_quick_xml_ident!(
                        ctx,
                        "::xsd_parser_types::quick_xml::WithDeserializer"
                    );

                    quote! {
                        #variant_ident(<#target_type as #with_deserializer>::Deserializer),
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
                    let variant_ident = &element.variant_ident;
                    let target_type = ctx.resolve_type_for_deserialize_module(&element.target_type);

                    let option = resolve_build_in!(ctx, "::core::option::Option");

                    let with_deserializer = resolve_quick_xml_ident!(
                        ctx,
                        "::xsd_parser_types::quick_xml::WithDeserializer"
                    );

                    quote! {
                        #variant_ident(#option<<#target_type as #with_deserializer>::Deserializer>),
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

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_helper(&self, ctx: &mut Context<'_, '_>) {
        let is_defaultable = ctx.is_defaultable_type();

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
            .map(|x| x.deserializer_struct_field_fn_store(ctx))
            .collect::<Vec<_>>();
        let handle_elements = elements
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let next = elements.get(i + 1);

                if let StructMode::All { .. } = &self.mode {
                    x.deserializer_struct_field_fn_handle_all(ctx, deserializer_state_ident)
                } else {
                    x.deserializer_struct_field_fn_handle_sequence(
                        ctx,
                        next,
                        deserializer_state_ident,
                    )
                }
            })
            .collect::<Vec<_>>();

        let impl_default = is_defaultable.then(|| self.render_deserializer_impl_default(ctx));

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

            #impl_default
        };

        ctx.quick_xml_deserialize().append(code);
    }

    fn render_deserializer_fn_find_suitable(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let allow_any = self.allow_any();
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

        let fallback = self
            .elements()
            .iter()
            .find_map(|x| x.deserializer_struct_field_init_text(ctx))
            .unwrap_or_else(|| {
                let element_handler_output = resolve_quick_xml_ident!(
                    ctx,
                    "::xsd_parser_types::quick_xml::ElementHandlerOutput"
                );

                quote! {
                    *self.state__ = fallback.take().unwrap_or(#deserializer_state_ident::Init__);

                    Ok(#element_handler_output::return_to_parent(event, #allow_any_result))
                }
            });

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        quote! {
            fn find_suitable<'de>(
                &mut self,
                helper: &mut #deserialize_helper,
                event: #event<'de>,
                fallback: &mut #option<#deserializer_state_ident>,
            ) -> #result<#element_handler_output<'de>, #error> {
                #allow_any_decl

                if let #event::Start(x) | #event::Empty(x) = &event {
                    #( #elements )*
                    #( #groups )*
                }

                #fallback
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn render_deserializer_fn_from_bytes_start(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let config = ctx.get_ref::<DeserializerConfig>();

        let deserializer_state_ident = &self.deserializer_state_ident;

        let mut index = 0;
        let mut any_attribute = None;

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        let attrib_var = self.attributes.iter().map(|x| x.deserializer_var_decl(ctx));
        let attrib_match = self
            .attributes
            .iter()
            .filter_map(|x| x.deserializer_matcher(ctx, &mut index, &mut any_attribute))
            .collect::<Vec<_>>();
        let attrib_init = self
            .attributes
            .iter()
            .map(|x| x.deserializer_struct_field_init(ctx, &self.type_ident));
        let element_init = self
            .elements()
            .iter()
            .map(|x| x.deserializer_struct_field_init(ctx));
        let content_init = self
            .content()
            .map(|x| x.deserializer_struct_field_init(ctx));

        let has_normal_attributes = index > 0;
        let need_default_handler = !self.allow_any_attribute || any_attribute.is_some();
        let default_attrib_handler = need_default_handler.then(|| {
            let body = any_attribute
                .unwrap_or_else(|| quote! { helper.raise_unexpected_attrib_checked(&attrib)?; });

            if has_normal_attributes {
                quote! {
                    else {
                        #body
                    }
                }
            } else {
                body
            }
        });

        let need_attrib_loop = self.has_attributes() || default_attrib_handler.is_some();
        let attrib_loop = need_attrib_loop.then(|| {
            quote! {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;

                    #( #attrib_match )*

                    #default_attrib_handler
                }
            }
        });

        let self_type = if config.boxed_deserializer {
            quote!(#box_<Self>)
        } else {
            quote!(Self)
        };

        let self_ctor = if self.represents_element() {
            quote! {
                Self {
                    #( #attrib_init )*
                    #( #element_init )*
                    #content_init
                    state__: #box_::new(#deserializer_state_ident::Init__),
                }
            }
        } else {
            quote!(Self::default())
        };
        let self_ctor = ctx.do_box(config.boxed_deserializer, self_ctor);

        let result = resolve_build_in!(ctx, "::core::result::Result");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let bytes_start =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::BytesStart");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");

        quote! {
            fn from_bytes_start(
                helper: &mut #deserialize_helper,
                bytes_start: &#bytes_start<'_>
            ) -> #result<#self_type, #error> {
                #( #attrib_var )*

                #attrib_loop

                Ok(#self_ctor)
            }
        }
    }

    fn render_deserializer_fn_finish_state(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let deserializer_state_ident = &self.deserializer_state_ident;

        let result = resolve_build_in!(ctx, "::core::result::Result");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");

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
                        self.store_content(deserializer.finish(helper)?)?;
                    }

                    Ok(())
                }
            }
            _ => quote! { Ok(()) },
        };

        quote! {
            fn finish_state(
                &mut self,
                helper: &mut #deserialize_helper,
                state: #deserializer_state_ident
            ) -> #result<(), #error> {
                #body
            }
        }
    }

    fn render_deserializer_impl_default(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");
        let default = resolve_build_in!(ctx, "::core::default::Default");

        let deserializer_ident = &self.deserializer_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let element_init = self
            .elements()
            .iter()
            .map(|x| x.deserializer_struct_field_init(ctx));
        let content_init = self
            .content()
            .map(|x| x.deserializer_struct_field_init(ctx));

        quote! {
            impl #default for #deserializer_ident {
                fn default() -> Self {
                    Self {
                        #( #element_init )*
                        #content_init
                        state__: #box_::new(#deserializer_state_ident::Init__),
                    }
                }
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
        if matches!(&self.mode, StructMode::Content { content } if content.is_simple()) {
            self.render_deserializer_fn_init_simple(ctx)
        } else if self.represents_element() {
            self.render_deserializer_fn_init_for_element(ctx)
        } else {
            self.render_deserializer_fn_init_for_group(ctx)
        }
    }

    fn render_deserializer_fn_init_simple(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let deserializer_ident = &self.deserializer_ident;
        let config = ctx.get_ref::<DeserializerConfig>();
        let boxed_deserializer_ident =
            boxed_deserializer_ident(config.boxed_deserializer, deserializer_ident);

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let deserializer_event =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerEvent");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");

        quote! {
            let (#event::Start(x) | #event::Empty(x)) = &event else {
                return Ok(#deserializer_output {
                    artifact: #deserializer_artifact::None,
                    event: #deserializer_event::Break(event),
                    allow_any: false,
                });
            };

            #boxed_deserializer_ident::from_bytes_start(helper, x)?.next(helper, event)
        }
    }

    fn render_deserializer_fn_init_for_group(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let config = ctx.get_ref::<DeserializerConfig>();
        let deserializer_ident = &self.deserializer_ident;
        let boxed_deserializer_ident =
            boxed_deserializer_ident(config.boxed_deserializer, deserializer_ident);
        let deserializer_state_ident = &self.deserializer_state_ident;

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");

        let element_init = self
            .elements()
            .iter()
            .map(|x| x.deserializer_struct_field_init(ctx));
        let content_init = self
            .content()
            .map(|x| x.deserializer_struct_field_init(ctx));
        let init_deserializer = ctx.do_box(
            config.boxed_deserializer,
            quote! {
                #boxed_deserializer_ident {
                    #( #element_init )*
                    #content_init
                    state__: #box_::new(#deserializer_state_ident::Init__),
                }
            },
        );

        quote! {
            let deserializer = #init_deserializer;
            let mut output = deserializer.next(helper, event)?;

            output.artifact = match output.artifact {
                #deserializer_artifact::Deserializer(x)
                    if matches!(&*x.state__, #deserializer_state_ident::Init__)
                        => #deserializer_artifact::None,
                artifact => artifact,
            };

            Ok(output)
        }
    }

    fn render_deserializer_fn_next(&self, ctx: &Context<'_, '_>) -> TokenStream {
        match &self.mode {
            StructMode::Empty { allow_any } => {
                self.render_deserializer_fn_next_empty(ctx, *allow_any)
            }
            StructMode::Content { content } => {
                if content.is_simple() {
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
        let (_, return_end_event) = self.return_end_event(ctx);

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let deserializer_event =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerEvent");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");

        ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

        let mixed_handler = self.base.is_mixed().then(|| {
            quote! {
                else if matches!(&event, #event::Text(_) | #event::CData(_)) {
                    Ok(#deserializer_output {
                        artifact: #deserializer_artifact::Deserializer(self),
                        event: #deserializer_event::None,
                        allow_any: #allow_any,
                    })
                }
            }
        });

        quote! {
            if let #event::End(_) = &event {
                Ok(#deserializer_output {
                    artifact: #deserializer_artifact::Data(self.finish(helper)?),
                    event: #return_end_event,
                    allow_any: false,
                })
            }
            #mixed_handler
            else {
                Ok(#deserializer_output {
                    artifact: #deserializer_artifact::Deserializer(self),
                    event: #deserializer_event::Break(event),
                    allow_any: #allow_any,
                })
            }
        }
    }

    fn render_deserializer_fn_next_content_simple(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let deserializer_state_ident = &self.deserializer_state_ident;

        let replace = resolve_quick_xml_ident!(ctx, "::core::mem::replace");
        let content_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ContentDeserializer");

        ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

        quote! {
            use #deserializer_state_ident as S;

            match #replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = #content_deserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
    }

    fn render_deserializer_fn_next_content_complex(
        &self,
        ctx: &Context<'_, '_>,
        content: &ComplexDataContent<'_>,
    ) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&content.target_type);
        let (event_at, return_end_event) = self.return_end_event(ctx);
        let deserializer_state_ident = &self.deserializer_state_ident;

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let replace = resolve_quick_xml_ident!(ctx, "::core::mem::replace");
        let with_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");
        let deserializer_event =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerEvent");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

        let has_done_state = content.need_done_state(self.represents_element());
        let done_handler = has_done_state.then(|| {
            quote! {
                (S::Done__, event) => {
                    *self.state__ = S::Done__;

                    break (#deserializer_event::Continue(event), false);
                },
            }
        });
        let artifact_handler = if has_done_state {
            quote! {
                let artifact = match &*self.state__ {
                    S::Done__ => #deserializer_artifact::Data(self.finish(helper)?),
                    _ => #deserializer_artifact::Deserializer(self),
                };
            }
        } else {
            quote! {
                let artifact = #deserializer_artifact::Deserializer(self);
            }
        };

        quote! {
            use #deserializer_state_ident as S;

            let mut event = event;
            let mut fallback = None;

            let (event, allow_any) = loop {
                let state = #replace(&mut *self.state__, S::Unknown__);

                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            #element_handler_output::Break { event, allow_any } => break (event, allow_any),
                            #element_handler_output::Continue { event, .. } => event,
                        }
                    }
                    (_, #event_at #event::End(_)) => {
                        return Ok(#deserializer_output {
                            artifact: #deserializer_artifact::Data(self.finish(helper)?),
                            event: #return_end_event,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = <#target_type as #with_deserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            #element_handler_output::Break { event, allow_any } => break (event, allow_any),
                            #element_handler_output::Continue { event, .. } => event,
                        }
                    },
                    #done_handler
                }
            };

            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }

            #artifact_handler

            Ok(#deserializer_output {
                artifact,
                event,
                allow_any,
            })
        }
    }

    fn render_deserializer_fn_next_all(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let (event_at, return_end_event) = self.return_end_event(ctx);
        let deserializer_state_ident = &self.deserializer_state_ident;

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let replace = resolve_quick_xml_ident!(ctx, "::core::mem::replace");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        let handlers = self
            .elements()
            .iter()
            .map(|x| x.deserializer_struct_field_fn_next_all(ctx));

        quote! {
            use #deserializer_state_ident as S;

            let mut event = event;
            let mut fallback = None;

            let (event, allow_any) = loop {
                let state = #replace(&mut *self.state__, S::Unknown__);

                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    #( #handlers )*
                    (_, #event_at #event::End(_)) => {
                        return Ok(#deserializer_output {
                            artifact: #deserializer_artifact::Data(self.finish(helper)?),
                            event: #return_end_event,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
                            #element_handler_output::Continue { event, .. } => event,
                            #element_handler_output::Break { event, allow_any } => break (event, allow_any),
                        }
                    },
                }
            };

            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }

            Ok(#deserializer_output {
                artifact: #deserializer_artifact::Deserializer(self),
                event,
                allow_any,
            })
        }
    }

    #[allow(clippy::too_many_lines)]
    fn render_deserializer_fn_next_sequence(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let elements = self.elements();
        let allow_any = self.allow_any();
        let (event_at, return_end_event) = self.return_end_event(ctx);
        let deserializer_state_ident = &self.deserializer_state_ident;

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let replace = resolve_quick_xml_ident!(ctx, "::core::mem::replace");
        let deserializer_event =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerEvent");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");

        let text_only = elements.iter().all(|el| el.meta().is_text());
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

        let any_retry = self.has_any().then(|| {
            quote! {
                let mut is_any_retry = false;
                let mut any_fallback = None;
            }
        });

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

        let mut handle_done = quote! {
            *self.state__ = S::Done__;
            break (#deserializer_event::Continue(event), #done_allow_any);
        };

        if self.has_any() {
            handle_done = quote! {
                if let Some(state) = any_fallback.take() {
                    is_any_retry = true;

                    *self.state__ = state;

                    event
                } else {
                    #handle_done
                }
            };
        }

        let handler_mixed = (!text_only && self.base.is_mixed()).then(|| {
            quote! {
                (state, #event::Text(_) | #event::CData(_)) => {
                    *self.state__ = state;
                    break (#deserializer_event::None, false);
                }
            }
        });

        let handler_fallback = text_only.not().then(|| {
            quote! {
                (state, event) => {
                    *self.state__ = state;
                    break (#deserializer_event::Break(event), false);
                }
            }
        });

        quote! {
            use #deserializer_state_ident as S;

            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            #any_retry

            let (event, allow_any) = loop {
                let state = #replace(&mut *self.state__, S::Unknown__);

                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    #( #handlers_continue )*
                    (_, #event_at #event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }

                        return Ok(#deserializer_output {
                            artifact: #deserializer_artifact::Data(self.finish(helper)?),
                            event: #return_end_event,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        #init_set_any

                        fallback.get_or_insert(S::Init__);

                        *self.state__ = S::#first_ident(None);

                        event
                    },
                    #( #handlers_create )*
                    (S::Done__, event) => {
                        #handle_done
                    },
                    #handler_mixed
                    #handler_fallback
                }
            };

            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }

            Ok(#deserializer_output {
                artifact: #deserializer_artifact::Deserializer(self),
                event,
                allow_any,
            })
        }
    }

    fn render_deserializer_fn_finish(&self, ctx: &mut Context<'_, '_>) -> TokenStream {
        let type_ident = &self.type_ident;
        let deserializer_state_ident = &self.deserializer_state_ident;

        let attributes = self
            .attributes
            .iter()
            .map(ComplexDataAttribute::deserializer_struct_field_finish);
        let elements = self
            .elements()
            .iter()
            .map(|x| x.deserializer_struct_field_finish(ctx))
            .collect::<Vec<_>>();
        let content = self
            .content()
            .map(|x| x.deserializer_struct_field_finish(ctx));

        let replace = resolve_quick_xml_ident!(ctx, "::core::mem::replace");

        quote! {
            let state = #replace(&mut *self.state__, #deserializer_state_ident::Unknown__);
            self.finish_state(helper, state)?;

            Ok(super::#type_ident {
                #( #attributes )*
                #( #elements )*
                #content
            })
        }
    }
}

impl ComplexDataContent<'_> {
    fn need_next_state(&self) -> bool {
        !self.is_simple()
    }

    fn need_done_state(&self, represents_element: bool) -> bool {
        !self.is_simple() && !represents_element && self.max_occurs.is_bounded()
    }

    fn deserializer_field_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let target_type = match self.occurs {
            Occurs::Single | Occurs::Optional => {
                let option = resolve_build_in!(ctx, "::core::option::Option");

                quote!(#option<#target_type>)
            }
            Occurs::DynamicList | Occurs::StaticList(_) => {
                let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");

                quote!(#vec<#target_type>)
            }
            e => crate::unreachable!("{:?}", e),
        };

        quote! {
            content: #target_type,
        }
    }

    fn deserializer_struct_field_init(&self, ctx: &Context<'_, '_>) -> TokenStream {
        match self.occurs {
            Occurs::None => quote!(),
            Occurs::Single | Occurs::Optional => quote!(content: None,),
            Occurs::DynamicList | Occurs::StaticList(_) => {
                let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");

                quote!(content: #vec::new(),)
            }
        }
    }

    fn deserializer_struct_field_finish(&self, ctx: &mut Context<'_, '_>) -> TokenStream {
        let convert = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => {
                if ctx.has_defaultable_content() {
                    quote!(helper.finish_default(self.content)?)
                } else {
                    quote!(helper.finish_content(self.content)?)
                }
            }
            Occurs::Optional => {
                quote! { self.content }
            }
            Occurs::DynamicList => {
                let min = self.min_occurs;
                let max = self.max_occurs.render_opt();

                if ctx.has_defaultable_content() {
                    quote!(helper.finish_vec_default(#min, self.content)?)
                } else {
                    quote!(helper.finish_vec(#min, #max, self.content)?)
                }
            }
            Occurs::StaticList(sz) => {
                if ctx.has_defaultable_content() {
                    quote!(helper.finish_arr_default::<_, #sz>(self.content)?)
                } else {
                    quote!(helper.finish_arr::<_, #sz>(self.content)?)
                }
            }
        };

        quote! {
            content: #convert,
        }
    }

    fn deserializer_struct_field_fn_store(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let body = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single | Occurs::Optional => {
                let error_kind =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ErrorKind");

                quote! {
                    if self.content.is_some() {
                        Err(#error_kind::DuplicateContent)?;
                    }

                    self.content = Some(value);
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                self.content.push(value);
            },
        };

        let result = resolve_build_in!(ctx, "::core::result::Result");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");

        quote! {
            fn store_content(&mut self, value: #target_type) -> #result<(), #error> {
                #body

                Ok(())
            }
        }
    }

    fn deserializer_struct_field_fn_handle(
        &self,
        ctx: &mut Context<'_, '_>,
        type_ident: &Ident2,
        represents_element: bool,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        if self.is_simple() {
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
        assert!(self.is_simple());
        assert_eq!(self.min_occurs, 1);
        assert_eq!(self.max_occurs, MaxOccurs::Bounded(1));

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);
        let config = ctx.get_ref::<DeserializerConfig>();
        let self_type = config.boxed_deserializer.then(|| quote!(: #box_<Self>));

        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");
        let deserializer_result =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerResult");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");

        quote! {
            fn handle_content<'de>(
                mut self #self_type,
                helper: &mut #deserialize_helper,
                output: #deserializer_output<'de, #target_type>,
            ) -> #deserializer_result<'de, super::#type_ident> {
                use #deserializer_state_ident as S;

                let #deserializer_output { artifact, event, allow_any } = output;

                match artifact {
                    #deserializer_artifact::None => Ok(#deserializer_output {
                        artifact: #deserializer_artifact::None,
                        event,
                        allow_any,
                    }),
                    #deserializer_artifact::Data(data) => {
                        self.store_content(data)?;
                        let data = self.finish(helper)?;

                        Ok(#deserializer_output {
                            artifact: #deserializer_artifact::Data(data),
                            event,
                            allow_any,
                        })
                    }
                    #deserializer_artifact::Deserializer(deserializer) => {
                        *self.state__ = S::Content__(deserializer);

                        Ok(#deserializer_output {
                            artifact: #deserializer_artifact::Deserializer(self),
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
        ctx: &mut Context<'_, '_>,
        represents_element: bool,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        assert!(!self.is_simple());

        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");
        let usize_ = resolve_build_in!(ctx, "::core::primitive::usize");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        // Handler for `DeserializerArtifact::None`: Should only be the case if
        // we try to initialize a new deserializer.
        let min = self.min_occurs;
        let is_defaultable = self.min_occurs == 0 || ctx.has_defaultable_content();
        let none_handler = match (represents_element, is_defaultable, self.occurs) {
            (_, _, Occurs::None) => unreachable!(),
            // For groups with defaultable content: If we were not able to create
            // a new deserializer we return to the parent.
            (false, true, _) => quote! {
                *self.state__ = fallback.take().unwrap_or(S::Next__);

                return Ok(#element_handler_output::break_(event, allow_any));
            },
            // For groups with non-defaultable single content: If we were not able
            // to create a new deserializer, we return to the parent because this
            // can only happen if we are in the `Init__` state.
            (false, false, Occurs::Single | Occurs::Optional) => quote! {
                *self.state__ = S::Init__;

                return Ok(#element_handler_output::break_(event, allow_any));
            },
            // For groups with non-defaultable list content: If we were not able
            // to create a new deserializer, we check the current size of the list
            // and return to the parent if the minimum bound is fulfilled.
            (false, false, Occurs::DynamicList | Occurs::StaticList(_)) => quote! {
                *self.state__ = fallback.take().unwrap_or(S::Next__);

                let len = self.content.len() + #usize_::from(matches!(*self.state__, S::Content__(_)));
                if len < #min {
                    return Ok(#element_handler_output::return_to_root(event, allow_any));
                } else {
                    return Ok(#element_handler_output::break_(event, allow_any));
                }
            },
            // For elements: If we were not able to create a new deserializer we
            // continue only for `End` events, because they may finish the current
            // deserializer.
            (true, _, _) => quote! {
                *self.state__ = fallback.take().unwrap_or(S::Next__);

                return Ok(#element_handler_output::from_event_end(event, allow_any));
            },
        };

        // Handler for `DeserializerArtifact::Data`
        let data_handler = match (represents_element, self.occurs, self.max_occurs) {
            (_, Occurs::None, _) => unreachable!(),
            // For groups: Return instantly if we have received the expected value
            (false, Occurs::Single | Occurs::Optional, _) => quote! {
                *self.state__ = S::Done__;

                Ok(#element_handler_output::break_(event, allow_any))
            },
            // For groups: Finish the deserialization if the expected max value
            // has been reached, continue if not.
            (false, Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => {
                quote! {
                    if self.content.len() < #max {
                        *self.state__ = S::Next__;

                        Ok(#element_handler_output::from_event(event, allow_any))
                    } else {
                        *self.state__ = S::Done__;

                        Ok(#element_handler_output::break_(event, allow_any))
                    }
                }
            }
            // For groups: Continue if unbound.
            // For elements: Continue always, the element is terminated by the
            // end event.
            (_, _, _) => quote! {
                *self.state__ = S::Next__;

                Ok(#element_handler_output::from_event(event, allow_any))
            },
        };

        // Handler for `DeserializerArtifact::Deserializer`
        #[allow(clippy::match_bool)]
        let break_or_end = match represents_element {
            true => quote!(from_event_end),
            false => quote!(break_),
        };
        let deserializer_handler = match (self.occurs, self.max_occurs) {
            (Occurs::None, _) => unreachable!(),
            // If we only expect single content, we return to the parent.
            (Occurs::Single | Occurs::Optional, _) => {
                quote! {
                    *self.state__ = S::Content__(deserializer);

                    Ok(#element_handler_output::#break_or_end(event, allow_any))
                }
            }
            // We continue the deserialization if we have more space left, otherwise
            // we return to the parent. The `max - 1` is space reserved in the result
            // for the yet unfinished deserializer.
            (Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => {
                assert!(max >= 2);
                let max_minus_one = max - 1;

                quote! {
                    if self.content.len() < #max_minus_one {
                        *fallback = Some(S::Content__(deserializer));
                        *self.state__ = S::Next__;

                        Ok(#element_handler_output::from_event(event, allow_any))
                    } else {
                        *self.state__ = S::Content__(deserializer);

                        Ok(#element_handler_output::#break_or_end(event, allow_any))
                    }
                }
            }
            // For unbound groups and elements, we try a new deserializer in case.
            (Occurs::DynamicList | Occurs::StaticList(_), _) => quote! {
                *fallback = Some(S::Content__(deserializer));
                *self.state__ = S::Next__;

                Ok(#element_handler_output::from_event(event, allow_any))
            },
        };

        quote! {
            fn handle_content<'de>(
                &mut self,
                helper: &mut #deserialize_helper,
                output: #deserializer_output<'de, #target_type>,
                fallback: &mut #option<#deserializer_state_ident>,
            ) -> #result<#element_handler_output<'de>, #error> {
                use #deserializer_state_ident as S;

                let #deserializer_output {
                    artifact,
                    event,
                    allow_any,
                } = output;

                if artifact.is_none() {
                    #none_handler
                }

                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }

                match artifact {
                    #deserializer_artifact::None => unreachable!(),
                    #deserializer_artifact::Data(data) => {
                        self.store_content(data)?;

                        #data_handler
                    }
                    #deserializer_artifact::Deserializer(deserializer) => {
                        #deserializer_handler
                    }
                }
            }
        }
    }
}

impl ComplexDataAttribute<'_> {
    fn deserializer_matcher(
        &self,
        ctx: &Context<'_, '_>,
        index: &mut usize,
        any_attribute: &mut Option<TokenStream>,
    ) -> Option<TokenStream> {
        let b_name = &self.b_name;
        let field_ident = &self.ident;

        let else_ = (*index).gt(&0).then(|| quote!(else));

        if self.meta.is_any() {
            *any_attribute = Some(quote! {
                #field_ident.push(attrib)?;
            });

            None
        } else if let Some(path) = ctx
            .types
            .meta
            .types
            .modules
            .get(&self.meta.ident.ns)
            .and_then(|x| x.make_ns_const())
        {
            let ns_name = ctx.resolve_type_for_deserialize_module(&path);

            *index += 1;

            Some(quote! {
                #else_ if matches!(helper.resolve_local_name(attrib.key, &#ns_name), Some(#b_name)) {
                    helper.read_attrib(&mut #field_ident, #b_name, &attrib.value)?;
                }
            })
        } else {
            *index += 1;

            Some(quote! {
                #else_ if attrib.key.local_name().as_ref() == #b_name {
                    helper.read_attrib(&mut #field_ident, #b_name, &attrib.value)?;
                }
            })
        }
    }

    fn deserializer_var_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let field_ident = &self.ident;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        if self.meta.is_any() {
            quote!(let mut #field_ident = #target_type::default();)
        } else {
            let option = resolve_build_in!(ctx, "::core::option::Option");

            quote!(let mut #field_ident: #option<#target_type> = None;)
        }
    }

    fn deserializer_struct_field_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let field_ident = &self.ident;
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let target_type = if self.is_option {
            let option = resolve_build_in!(ctx, "::core::option::Option");

            quote!(#option<#target_type>)
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

        let convert = if self.meta.is_any() {
            None
        } else if self.default_value.is_some() {
            let default_fn_ident = format_ident!("default_{field_ident}");

            Some(quote! { .unwrap_or_else(super::#type_ident::#default_fn_ident) })
        } else if self.meta.use_ == Use::Required {
            let name = &self.s_name;

            let error_kind =
                resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ErrorKind");

            Some(quote! { .ok_or_else(|| #error_kind::MissingAttribute(#name.into()))? })
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

impl ComplexDataElement<'_> {
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

    fn target_type_allows_any(&self, types: &MetaTypes) -> bool {
        fn walk(types: &MetaTypes, visit: &mut HashSet<TypeIdent>, ident: &TypeIdent) -> bool {
            if !visit.insert(ident.clone()) {
                return false;
            }

            match types.get_variant(ident) {
                Some(MetaTypeVariant::All(si) | MetaTypeVariant::Choice(si)) => {
                    for element in &*si.elements {
                        match &element.variant {
                            ElementMetaVariant::Text => return false,
                            ElementMetaVariant::Any { .. } => return true,
                            ElementMetaVariant::Type { type_, .. } => {
                                if walk(types, visit, type_) {
                                    return true;
                                }
                            }
                        }
                    }

                    false
                }
                Some(MetaTypeVariant::Sequence(si)) => match si.elements.first() {
                    None => false,
                    Some(ElementMeta {
                        variant: ElementMetaVariant::Any { .. },
                        ..
                    }) => true,
                    Some(ElementMeta {
                        variant: ElementMetaVariant::Text,
                        ..
                    }) => false,
                    Some(ElementMeta {
                        variant: ElementMetaVariant::Type { type_, .. },
                        ..
                    }) => walk(types, visit, type_),
                },
                Some(MetaTypeVariant::ComplexType(ComplexMeta {
                    content: Some(content),
                    ..
                })) => walk(types, visit, content),
                Some(MetaTypeVariant::Custom(x)) => x.allow_any(),
                _ => false,
            }
        }

        let mut visit = HashSet::new();

        match &self.meta().variant {
            ElementMetaVariant::Any { .. } => true,
            ElementMetaVariant::Type { type_, .. } => walk(types, &mut visit, type_),
            ElementMetaVariant::Text => false,
        }
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
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let with_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");

        ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

        let body = quote! {
            let output = <#target_type as #with_deserializer>::init(helper, event)?;

            return #call_handler;
        };

        if let Some(path) = ctx
            .types
            .meta
            .types
            .modules
            .get(&self.meta().ident.ns)
            .and_then(|x| x.make_ns_const())
        {
            let ns_name = ctx.resolve_type_for_deserialize_module(&path);

            Some(quote! {
                if matches!(helper.resolve_local_name(x.name(), &#ns_name), Some(#b_name)) {
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
    ) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let with_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

        let handle_continue = if handle_any {
            quote! {
                #element_handler_output::Continue { event, allow_any } => {
                    allow_any_element = allow_any_element || allow_any;

                    event
                },
            }
        } else {
            quote! {
                #element_handler_output::Continue { event, .. } => event,
            }
        };

        quote! {
            event = {
                let output = <#target_type as #with_deserializer>::init(helper, event)?;

                match #call_handler? {
                    #handle_continue
                    output => { return Ok(output); }
                }
            };
        }
    }

    fn deserializer_enum_variant_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);
        let variant_ident = &self.variant_ident;

        let with_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");

        match self.occurs {
            Occurs::Single | Occurs::Optional => {
                let option = resolve_build_in!(ctx, "::core::option::Option");

                quote! {
                    #variant_ident(
                        #option<#target_type>,
                        #option<<#target_type as #with_deserializer>::Deserializer>,
                        #option<<#target_type as #with_deserializer>::Deserializer>,
                    ),
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => {
                let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");
                let option = resolve_build_in!(ctx, "::core::option::Option");

                quote! {
                    #variant_ident(
                        #vec<#target_type>,
                        #option<<#target_type as #with_deserializer>::Deserializer>,
                        #option<<#target_type as #with_deserializer>::Deserializer>,
                    ),
                }
            }
            e => crate::unreachable!("{:?}", e),
        }
    }

    fn deserializer_enum_variant_init_element(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let default = resolve_build_in!(ctx, "::core::default::Default");

        let handler_ident = self.handler_ident();
        let call_handler = quote!(self.#handler_ident(helper, #default::default(), None, output));

        self.deserializer_init_element(ctx, &call_handler)
    }

    fn deserializer_enum_variant_init_group(
        &self,
        ctx: &Context<'_, '_>,
        handle_any: bool,
    ) -> Option<TokenStream> {
        if !self.treat_as_group_or_dynamic() {
            return None;
        }

        let default = resolve_build_in!(ctx, "::core::default::Default");

        let handler_ident = self.handler_ident();
        let call_handler = quote!(self.#handler_ident(helper, #default::default(), None, output));

        Some(self.deserializer_init_group(ctx, handle_any, &call_handler))
    }

    fn deserializer_enum_variant_init_any(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        if !self.treat_as_any() {
            return None;
        }

        let default = resolve_build_in!(ctx, "::core::default::Default");

        let handler_ident = self.handler_ident();
        let call_handler = quote!(self.#handler_ident(helper, #default::default(), None, output));

        Some(self.deserializer_init_group(ctx, false, &call_handler))
    }

    fn deserializer_enum_variant_init_text(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        if !self.treat_as_text() {
            return None;
        }

        let default = resolve_build_in!(ctx, "::core::default::Default");

        let handler_ident = self.handler_ident();
        let call_handler = quote!(self.#handler_ident(helper, #default::default(), None, output));

        Some(self.deserializer_init_group(ctx, false, &call_handler))
    }

    fn deserializer_enum_variant_default(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
    ) -> TokenStream {
        let variant_ident = &self.variant_ident;

        match self.occurs {
            Occurs::None => unreachable!(),
            Occurs::Single => {
                let value = quote!(helper.finish_default(None)?);
                let value = ctx.do_box(self.need_indirection, value);

                quote! {
                    Ok(super::#type_ident::#variant_ident(#value))
                }
            }
            Occurs::Optional => quote!(Ok(super::#type_ident::#variant_ident(None))),
            Occurs::DynamicList => {
                let min = self.meta().min_occurs;

                quote! {
                    Ok(super::#type_ident::#variant_ident(helper.finish_vec_default(#min, Vec::new())?))
                }
            }
            Occurs::StaticList(sz) => {
                let value = quote!(helper.finish_arr_default::<_, #sz>(Vec::new())?);
                let value = ctx.do_box(self.need_indirection, value);

                quote! {
                    Ok(super::#type_ident::#variant_ident(#value))
                }
            }
        }
    }

    fn deserializer_enum_variant_finish(
        &self,
        ctx: &mut Context<'_, '_>,
        type_ident: &Ident2,
        deserializer_ident: &Ident2,
    ) -> TokenStream {
        let name = &self.s_name;
        let store_ident = self.store_ident();
        let variant_ident = &self.variant_ident;

        let convert = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => {
                let value = if ctx.is_defaultable_element(self.meta()) {
                    quote!(helper.finish_default(values)?)
                } else {
                    quote!(helper.finish_element(#name, values)?)
                };

                ctx.do_box(self.need_indirection, value)
            }
            Occurs::Optional if self.need_indirection => {
                let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

                quote! { values.map(#box_::new) }
            }
            Occurs::Optional => {
                quote! { values }
            }
            Occurs::DynamicList => {
                let min = self.meta().min_occurs;
                let max = self.meta().max_occurs.render_opt();

                if min == 0 {
                    quote!(values)
                } else if ctx.is_defaultable_element(self.meta()) {
                    quote!(helper.finish_vec_default(#min, values)?)
                } else {
                    quote!(helper.finish_vec(#min, #max, values)?)
                }
            }
            Occurs::StaticList(sz) => {
                let value = if ctx.is_defaultable_element(self.meta()) {
                    quote!(helper.finish_arr_default::<_, #sz>(values)?)
                } else {
                    quote!(helper.finish_arr::<_, #sz>(values)?)
                };

                ctx.do_box(self.need_indirection, value)
            }
        };

        quote! {
            S::#variant_ident(mut values, None, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(helper)?;
                    #deserializer_ident::#store_ident(&mut values, value)?;
                }

                Ok(super::#type_ident::#variant_ident(#convert))
            }
        }
    }

    fn deserializer_enum_variant_fn_store(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let name = &self.b_name;
        let store_ident = self.store_ident();

        match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single | Occurs::Optional => {
                let result = resolve_build_in!(ctx, "::core::result::Result");
                let option = resolve_build_in!(ctx, "::core::option::Option");

                let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
                let error_kind =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ErrorKind");
                let raw_byte_str =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::RawByteStr");

                quote! {
                    fn #store_ident(values: &mut #option<#target_type>, value: #target_type) -> #result<(), #error> {
                        if values.is_some() {
                            Err(#error_kind::DuplicateElement(#raw_byte_str::from_slice(#name)))?;
                        }

                        *values = Some(value);

                        Ok(())
                    }
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => {
                let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");
                let result = resolve_build_in!(ctx, "::core::result::Result");

                let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");

                quote! {
                    fn #store_ident(values: &mut #vec<#target_type>, value: #target_type) -> #result<(), #error> {
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
        ctx: &mut Context<'_, '_>,
        represents_element: bool,
        deserializer_ident: &Ident2,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let store_ident = self.store_ident();
        let handler_ident = self.handler_ident();
        let variant_ident = &self.variant_ident;

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");
        let usize_ = resolve_build_in!(ctx, "::core::primitive::usize");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let with_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        #[allow(clippy::match_bool)]
        let break_or_end = match represents_element {
            true => quote!(from_event_end),
            false => quote!(break_),
        };

        #[allow(clippy::if_same_then_else)]
        let continue_or_return = if self.treat_as_text() {
            quote!(from_event)
        } else if self.treat_as_group_or_dynamic() {
            quote!(from_event)
        } else {
            quote!(return_to_root)
        };

        let values = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single | Occurs::Optional => quote!(#option<#target_type>),
            Occurs::DynamicList | Occurs::StaticList(_) => {
                let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");

                quote!(#vec<#target_type>)
            }
        };

        // Handler for `DeserializerArtifact::None`: Should only be the
        // case if we try to initialize a new deserializer.
        let min = self.meta().min_occurs;
        let is_defaultable = min == 0 || ctx.is_defaultable_element(self.meta());
        let none_handler = match (is_defaultable, self.occurs) {
            (_, Occurs::None) => unreachable!(),
            // Single or optional element: This is only triggered by `find_suitable`,
            // if the deserializer did not produce anything we continue/return.
            (_, Occurs::Single | Occurs::Optional) => quote! {
                return Ok(#element_handler_output::#continue_or_return(event, allow_any));
            },
            // Defaultable list of type (group or element): If the list and the
            // fallback is empty, we called from `find_suitable`, so we continue/
            // return. If we have a fallback or some values we called from
            // `next` and break or end.
            (true, Occurs::DynamicList | Occurs::StaticList(_)) => quote! {
                if fallback.is_none() && values.is_empty() {
                    *self.state__ = S::Init__;
                    return Ok(#element_handler_output::#continue_or_return(event, allow_any));
                } else {
                    *self.state__ = S::#variant_ident(values, None, fallback);
                    return Ok(#element_handler_output::#break_or_end(event, allow_any));
                }
            },
            // Non-defaultable list of type (group or element): If the list and
            // the fallback is empty, we called from `find_suitable`, so we
            // continue/return. If we have a fallback or some values, we called
            // from `next`, so we if the min bound of the list is fulfilled and
            // continue or break/end
            (false, Occurs::DynamicList | Occurs::StaticList(_)) => quote! {
                if fallback.is_none() && values.is_empty() {
                    *self.state__ = S::Init__;
                    return Ok(#element_handler_output::#continue_or_return(event, allow_any));
                } else if values.len() + #usize_::from(fallback.is_some()) < #min {
                    *self.state__ = S::#variant_ident(values, None, fallback);
                    return Ok(#element_handler_output::return_to_root(event, allow_any));
                } else {
                    *self.state__ = S::#variant_ident(values, None, fallback);
                    return Ok(#element_handler_output::#break_or_end(event, allow_any));
                }
            },
        };

        // Handler for `DeserializerArtifact::Data`
        let data_handler = match (represents_element, self.occurs, self.meta().max_occurs) {
            (_, Occurs::None, _) => unreachable!(),
            // For groups: Return instantly if we have received the expected value
            (false, Occurs::Single | Occurs::Optional, _) => quote! {
                let data = #deserializer_ident::finish_state(helper, S::#variant_ident(values, None, None))?;
                *self.state__ = S::Done__(data);

                Ok(#element_handler_output::break_(event, allow_any))
            },
            // For bounded groups: Finish the deserialization if the expected max value
            // has been reached, continue if not.
            (false, Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => {
                quote! {
                    if values.len() < #max {
                        *self.state__ = S::#variant_ident(values, None, None);

                        Ok(#element_handler_output::from_event(event, allow_any))
                    } else {
                        let data = #deserializer_ident::finish_state(helper, S::#variant_ident(values, None, None))?;
                        *self.state__ = S::Done__(data);

                        Ok(#element_handler_output::break_(event, allow_any))
                    }
                }
            }
            // For elements and unbounded groups, continue in any case
            (_, _, _) => quote! {
                *self.state__ = S::#variant_ident(values, None, None);

                Ok(#element_handler_output::from_event(event, allow_any))
            },
        };

        // Handler for `DeserializerArtifact::Deserializer`
        let deserializer_handler = match (self.occurs, self.meta().max_occurs) {
            (Occurs::None, _) => unreachable!(),
            // If we only expect single content, we return to the parent.
            (Occurs::Single | Occurs::Optional, _) => quote! {
                *self.state__ = S::#variant_ident(values, None, Some(deserializer));

                Ok(#element_handler_output::#break_or_end(event, allow_any))
            },
            // For bounded lists we continue the deserialization if we have more
            // space left, otherwise we return to the parent. The `max - 1` is
            // space reserved in the result for the yet unfinished deserializer.
            (Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => {
                assert!(max >= 2);
                let max_minus_one = max - 1;

                quote! {
                    let can_have_more = values.len() < #max_minus_one;
                    let ret = if can_have_more {
                        #element_handler_output::from_event(event, allow_any)
                    } else {
                        #element_handler_output::#break_or_end(event, allow_any)
                    };

                    if can_have_more && ret.is_continue_start_or_empty() {
                        *self.state__ = S::#variant_ident(values, Some(deserializer), None);
                    } else {
                        *self.state__ = S::#variant_ident(values, None, Some(deserializer));
                    }

                    Ok(ret)
                }
            }
            // For unbound lists, we can try a new deserializer in any case.
            (Occurs::DynamicList | Occurs::StaticList(_), _) => quote! {
                let ret = #element_handler_output::from_event(event, allow_any);

                if ret.is_continue_start_or_empty() {
                    *self.state__ = S::#variant_ident(values, Some(deserializer), None);
                } else {
                    *self.state__ = S::#variant_ident(values, None, Some(deserializer));
                }

                Ok(ret)
            },
        };

        quote! {
            fn #handler_ident<'de>(
                &mut self,
                helper: &mut #deserialize_helper,
                mut values: #values,
                fallback: #option<<#target_type as #with_deserializer>::Deserializer>,
                output: #deserializer_output<'de, #target_type>,
            ) -> #result<#element_handler_output<'de>, #error> {
                use #deserializer_state_ident as S;

                let #deserializer_output {
                    artifact,
                    event,
                    allow_any,
                } = output;

                if artifact.is_none() {
                    #none_handler
                }

                if let Some(deserializer) = fallback {
                    let data = deserializer.finish(helper)?;
                    #deserializer_ident::#store_ident(&mut values, data)?;
                }

                match artifact {
                    #deserializer_artifact::None => unreachable!(),
                    #deserializer_artifact::Data(data) => {
                        #deserializer_ident::#store_ident(&mut values, data)?;

                        #data_handler
                    }
                    #deserializer_artifact::Deserializer(deserializer) => {
                        #deserializer_handler
                    }
                }
            }
        }
    }

    fn deserializer_enum_variant_fn_next_continue(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let deserializer_matcher = quote!(Some(deserializer));
        let event_matcher = quote!(event);
        let output = quote!(deserializer.next(helper, event));

        self.deserializer_enum_variant_fn_next(ctx, &deserializer_matcher, &event_matcher, &output)
    }

    fn deserializer_enum_variant_fn_next_create(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let name = &self.b_name;
        let allow_any = self.target_type_allows_any(ctx.types.meta.types);
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let with_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");

        let deserializer_matcher = quote!(None);
        let event_matcher = quote!(event @ (#event::Start(_) | #event::Empty(_)));

        let need_name_matcher = !self.target_is_dynamic
            && matches!(
                &self.meta().variant,
                ElementMetaVariant::Type {
                    mode: ElementMode::Element,
                    ..
                }
            );

        let output = if need_name_matcher {
            let ns_name = ctx
                .types
                .meta
                .types
                .modules
                .get(&self.meta().ident.ns)
                .and_then(|x| x.make_ns_const())
                .map(|path| ctx.resolve_type_for_deserialize_module(&path))
                .map_or_else(|| quote!(None), |ns_name| quote!(Some(&#ns_name)));

            quote! {
                helper.init_start_tag_deserializer(event, #ns_name, #name, #allow_any)
            }
        } else {
            ctx.add_quick_xml_deserialize_usings(
                true,
                ["::xsd_parser_types::quick_xml::Deserializer"],
            );

            quote! {
                <#target_type as #with_deserializer>::init(helper, event)
            }
        };

        self.deserializer_enum_variant_fn_next(ctx, &deserializer_matcher, &event_matcher, &output)
    }

    fn deserializer_enum_variant_fn_next(
        &self,
        ctx: &Context<'_, '_>,
        deserializer_matcher: &TokenStream,
        event_matcher: &TokenStream,
        output: &TokenStream,
    ) -> TokenStream {
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();

        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        quote! {
            (S::#variant_ident(values, fallback, #deserializer_matcher), #event_matcher) => {
                let output = #output?;

                match self.#handler_ident(helper, values, fallback, output)? {
                    #element_handler_output::Break { event, allow_any } => break (event, allow_any),
                    #element_handler_output::Continue { event, .. } => event,
                }
            },
        }
    }

    fn deserializer_struct_field_decl(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let field_ident = &self.field_ident;

        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);
        let target_type = match self.occurs {
            Occurs::Single | Occurs::Optional => {
                let option = resolve_build_in!(ctx, "::core::option::Option");

                quote!(#option<#target_type>)
            }
            Occurs::StaticList(_) if self.need_indirection => {
                let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");
                let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

                quote!(#vec<#box_<#target_type>>)
            }
            Occurs::StaticList(_) | Occurs::DynamicList => {
                let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");

                quote!(#vec<#target_type>)
            }
            e => crate::unreachable!("{:?}", e),
        };

        quote! {
            #field_ident: #target_type,
        }
    }

    fn deserializer_struct_field_init(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let occurs = self.occurs;
        let field_ident = &self.field_ident;

        match occurs {
            Occurs::None => quote!(),
            Occurs::Single | Occurs::Optional => quote!(#field_ident: None,),
            Occurs::DynamicList | Occurs::StaticList(_) => {
                let vec = resolve_build_in!(ctx, "::alloc::vec::Vec");

                quote!(#field_ident: #vec::new(),)
            }
        }
    }

    fn deserializer_struct_field_init_element(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        let handler_ident = self.handler_ident();
        let call_handler = quote!(self.#handler_ident(helper, output, &mut *fallback));

        self.deserializer_init_element(ctx, &call_handler)
    }

    fn deserializer_struct_field_init_group(
        &self,
        ctx: &Context<'_, '_>,
        handle_any: bool,
    ) -> Option<TokenStream> {
        if !self.treat_as_group_or_dynamic() {
            return None;
        }

        let handler_ident = self.handler_ident();
        let call_handler = quote!(self.#handler_ident(helper, output, &mut *fallback));

        Some(self.deserializer_init_group(ctx, handle_any, &call_handler))
    }

    fn deserializer_struct_field_init_text(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        if !self.treat_as_text() {
            return None;
        }

        let handler_ident = self.handler_ident();
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let with_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");

        ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

        Some(quote! {
            let output = <#target_type as #with_deserializer>::init(helper, event)?;

            self.#handler_ident(helper, output, &mut *fallback)
        })
    }

    fn deserializer_struct_field_finish_state_all(&self) -> TokenStream {
        let store_ident = self.store_ident();
        let variant_ident = &self.variant_ident;

        quote! {
            S::#variant_ident(deserializer) => self.#store_ident(deserializer.finish(helper)?)?,
        }
    }

    fn deserializer_struct_field_finish_state_sequence(&self) -> TokenStream {
        let store_ident = self.store_ident();
        let variant_ident = &self.variant_ident;

        quote! {
            S::#variant_ident(Some(deserializer)) => self.#store_ident(deserializer.finish(helper)?)?,
        }
    }

    fn deserializer_struct_field_finish(&self, ctx: &mut Context<'_, '_>) -> TokenStream {
        let name = &self.s_name;
        let field_ident = &self.field_ident;

        let convert = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single => {
                let value = if ctx.is_defaultable_element(self.meta()) {
                    quote!(helper.finish_default(self.#field_ident)?)
                } else {
                    quote!(helper.finish_element(#name, self.#field_ident)?)
                };

                ctx.do_box(self.need_indirection, value)
            }
            Occurs::Optional if self.need_indirection => {
                let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

                quote! { self.#field_ident.map(#box_::new) }
            }
            Occurs::Optional => {
                quote! { self.#field_ident }
            }
            Occurs::DynamicList => {
                let min = self.meta().min_occurs;
                let max = self.meta().max_occurs.render_opt();

                if min == 0 {
                    quote!(self.#field_ident)
                } else if ctx.is_defaultable_element(self.meta()) {
                    quote!(helper.finish_vec_default(#min, self.#field_ident)?)
                } else {
                    quote!(helper.finish_vec(#min, #max, self.#field_ident)?)
                }
            }
            Occurs::StaticList(sz) => {
                let value = if ctx.is_defaultable_element(self.meta()) {
                    quote!(helper.finish_arr_default::<_, #sz>(self.#field_ident)?)
                } else {
                    quote!(helper.finish_arr::<_, #sz>(self.#field_ident)?)
                };

                ctx.do_box(self.need_indirection, value)
            }
        };

        quote! {
            #field_ident: #convert,
        }
    }

    fn deserializer_struct_field_fn_store(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let name = &self.b_name;
        let field_ident = &self.field_ident;
        let store_ident = self.store_ident();

        let body = match self.occurs {
            Occurs::None => crate::unreachable!(),
            Occurs::Single | Occurs::Optional => {
                let error_kind =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ErrorKind");
                let raw_byte_str =
                    resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::RawByteStr");

                quote! {
                    if self.#field_ident.is_some() {
                        Err(#error_kind::DuplicateElement(#raw_byte_str::from_slice(#name)))?;
                    }

                    self.#field_ident = Some(value);
                }
            }
            Occurs::StaticList(_) if self.need_indirection => {
                let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");

                quote! {
                    self.#field_ident.push(#box_::new(value));
                }
            }
            Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                self.#field_ident.push(value);
            },
        };

        let result = resolve_build_in!(ctx, "::core::result::Result");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");

        quote! {
            fn #store_ident(&mut self, value: #target_type) -> #result<(), #error> {
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
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let store_ident = self.store_ident();
        let handler_ident = self.handler_ident();
        let variant_ident = &self.variant_ident;

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        #[allow(clippy::if_same_then_else)]
        let continue_or_return = if self.treat_as_text() {
            quote!(from_event)
        } else if self.treat_as_group() {
            quote!(from_event)
        } else {
            quote!(return_to_root)
        };

        quote! {
            fn #handler_ident<'de>(
                &mut self,
                helper: &mut #deserialize_helper,
                output: #deserializer_output<'de, #target_type>,
                fallback: &mut #option<#deserializer_state_ident>,
            ) -> #result<#element_handler_output<'de>, #error> {
                use #deserializer_state_ident as S;

                let #deserializer_output {
                    artifact,
                    event,
                    allow_any,
                } = output;

                if artifact.is_none() {
                    *self.state__ = S::Next__;

                    return Ok(#element_handler_output::#continue_or_return(event, allow_any));
                }

                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }

                match artifact {
                    #deserializer_artifact::None => unreachable!(),
                    #deserializer_artifact::Data(data) => {
                        self.#store_ident(data)?;

                        *self.state__ = S::Next__;

                        Ok(#element_handler_output::from_event(event, allow_any))
                    }
                    #deserializer_artifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::#variant_ident(deserializer));

                        *self.state__ = S::Next__;

                        Ok(#element_handler_output::from_event(event, allow_any))
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn deserializer_struct_field_fn_handle_sequence(
        &self,
        ctx: &mut Context<'_, '_>,
        next: Option<&ComplexDataElement<'_>>,
        deserializer_state_ident: &Ident2,
    ) -> TokenStream {
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let store_ident = self.store_ident();
        let field_ident = &self.field_ident;
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let deserialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializeHelper");
        let deserializer_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerOutput");
        let deserializer_artifact =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::DeserializerArtifact");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        let next_state = if let Some(next) = next {
            let variant_ident = &next.variant_ident;

            quote!(S::#variant_ident(None))
        } else {
            quote!(S::Done__)
        };

        // Handler for `DeserializerArtifact::None`: Should only be the
        // case if we try to initialize a new deserializer.
        let is_defaultable = ctx.is_defaultable_element(self.meta());
        let handler_none = match (is_defaultable, self.occurs, self.meta().min_occurs) {
            (_, Occurs::None, _) => unreachable!(),
            // If we do not expect any data we continue with the next state
            (_, _, 0) | (_, Occurs::Optional, _) => quote! {
                fallback.get_or_insert(S::#variant_ident(None));

                *self.state__ = #next_state;

                return Ok(#element_handler_output::from_event(event, allow_any));
            },
            // If this represents an mandatory element with non-defaultable content,
            // we break, because we did not received the data yet.
            (false, Occurs::Single, _) => quote! {
                fallback.get_or_insert(S::#variant_ident(None));

                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(#element_handler_output::break_(event, allow_any));
                } else {
                    return Ok(#element_handler_output::return_to_root(event, allow_any));
                }
            },
            // If we did not reach the expected amount of elements, we stay in the
            // current state and break, otherwise we continue with the next state.
            (false, Occurs::DynamicList | Occurs::StaticList(_), min) => quote! {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(#element_handler_output::break_(event, allow_any));
                } else if self.#field_ident.len() < #min {
                    fallback.get_or_insert(S::#variant_ident(None));

                    return Ok(#element_handler_output::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::#variant_ident(None));

                    *self.state__ = #next_state;

                    return Ok(#element_handler_output::from_event(event, allow_any));
                }
            },
            // If this is a group, we continue, because groups may be default constructed.
            (true, _, _) => quote! {
                fallback.get_or_insert(S::#variant_ident(None));

                *self.state__ = #next_state;

                return Ok(#element_handler_output::from_event(event, allow_any));
            },
        };

        // Handler for `DeserializerArtifact::Data`:
        let data_handler = match (self.occurs, self.meta().max_occurs) {
            (Occurs::None, _) => unreachable!(),
            // If we got some data we simple move one to the next element
            (Occurs::Single | Occurs::Optional, _) => quote! {
                *self.state__ = #next_state;
            },
            // If we got some data and the maximum amount of elements of this
            // type is reached we move on, otherwise we stay in the current state.
            (Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => quote! {
                if self.#field_ident.len() < #max {
                    *self.state__ = S::#variant_ident(None);
                } else {
                    *self.state__ = #next_state;
                }
            },
            // Unbounded amount. Stay in the current state in any case.
            (_, _) => quote! {
                *self.state__ = S::#variant_ident(None);
            },
        };

        // Handler for `DeserializerArtifact::Deserializer:
        let deserializer_handler = match (self.occurs, self.meta().max_occurs) {
            // If we expect only one element we continue to the next state,
            // because the old yet unfinished deserializer already contains
            // this data.
            (Occurs::Single | Occurs::Optional, _) => quote! {
                *self.state__ = #next_state;
            },
            // If we expect multiple elements we only try to initialize a new
            // deserializer if the maximum has not been reached yet.
            // The `max - 1` is for the data that is contained in the yet unfinished
            // deserializer.
            (Occurs::DynamicList | Occurs::StaticList(_), MaxOccurs::Bounded(max)) => {
                assert!(max >= 2);
                let max_minus_one = max - 1;
                quote! {
                    if self.#field_ident.len() < #max_minus_one {
                        *self.state__ = S::#variant_ident(None);
                    } else {
                        *self.state__ = #next_state;
                    }
                }
            }
            // Infinit amount of data: Stay in the current state.
            (_, _) => quote! {
                *self.state__ = S::#variant_ident(None);
            },
        };

        quote! {
            fn #handler_ident<'de>(
                &mut self,
                helper: &mut #deserialize_helper,
                output: #deserializer_output<'de, #target_type>,
                fallback: &mut #option<#deserializer_state_ident>,
            ) -> #result<#element_handler_output<'de>, #error> {
                use #deserializer_state_ident as S;

                let #deserializer_output {
                    artifact,
                    event,
                    allow_any,
                } = output;

                if artifact.is_none() {
                    #handler_none
                }

                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }

                match artifact {
                    #deserializer_artifact::None => unreachable!(),
                    #deserializer_artifact::Data(data) => {
                        self.#store_ident(data)?;

                        #data_handler

                        Ok(#element_handler_output::from_event(event, allow_any))
                    }
                    #deserializer_artifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::#variant_ident(Some(deserializer)));

                        #deserializer_handler

                        Ok(#element_handler_output::from_event(event, allow_any))
                    }
                }
            }
        }
    }

    fn deserializer_struct_field_fn_next_all(&self, ctx: &Context<'_, '_>) -> TokenStream {
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();

        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        quote! {
            (
                S::#variant_ident(deserializer),
                event
            ) => {
                let output = deserializer.next(helper, event)?;
                match self.#handler_ident(helper, output, &mut fallback)? {
                    #element_handler_output::Continue { event, .. } => event,
                    #element_handler_output::Break { event, allow_any } => break (event, allow_any),
                }
            }
        }
    }

    fn deserializer_struct_field_fn_next_sequence_continue(
        &self,
        ctx: &Context<'_, '_>,
    ) -> TokenStream {
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();

        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        quote! {
            (
                S::#variant_ident(Some(deserializer)),
                event
            ) => {
                let output = deserializer.next(helper, event)?;
                match self.#handler_ident(helper, output, &mut fallback)? {
                    #element_handler_output::Continue { event, allow_any } => {
                        allow_any_element = allow_any_element || allow_any;

                        event
                    },
                    #element_handler_output::Break { event, allow_any } => break (event, allow_any),
                }
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn deserializer_struct_field_fn_next_sequence_create(
        &self,
        ctx: &Context<'_, '_>,
        next: Option<&ComplexDataElement<'_>>,
        allow_any: bool,
    ) -> TokenStream {
        let name = &self.b_name;
        let variant_ident = &self.variant_ident;
        let handler_ident = self.handler_ident();
        let target_type = ctx.resolve_type_for_deserialize_module(&self.target_type);

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let with_deserializer =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::WithDeserializer");
        let element_handler_output =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::ElementHandlerOutput");

        ctx.add_quick_xml_deserialize_usings(true, ["::xsd_parser_types::quick_xml::Deserializer"]);

        let next_state = if let Some(next) = next {
            let variant_ident = &next.variant_ident;

            quote!(S::#variant_ident(None))
        } else {
            quote!(S::Done__)
        };

        let allow_any = allow_any || self.target_type_allows_any(ctx.types.meta.types);

        let need_name_matcher = !self.target_is_dynamic
            && matches!(
                &self.meta().variant,
                ElementMetaVariant::Any { .. }
                    | ElementMetaVariant::Type {
                        mode: ElementMode::Element,
                        ..
                    }
            );

        let mut body = quote! {
            let output = <#target_type as #with_deserializer>::init(helper, event)?;
            match self.#handler_ident(helper, output, &mut fallback)? {
                #element_handler_output::Continue { event, allow_any } => {
                    allow_any_element = allow_any_element || allow_any;

                    event
                },
                #element_handler_output::Break { event, allow_any } => break (event, allow_any),
            }
        };

        if self.treat_as_any() {
            body = quote! {
                if is_any_retry {
                    #body
                } else {
                    any_fallback.get_or_insert(S::#variant_ident(None));

                    *self.state__ = #next_state;

                    event
                }
            };
        } else if need_name_matcher {
            let ns_name = ctx
                .types
                .meta
                .types
                .modules
                .get(&self.meta().ident.ns)
                .and_then(|x| x.make_ns_const())
                .map(|path| ctx.resolve_type_for_deserialize_module(&path))
                .map_or_else(|| quote!(None), |ns_name| quote!(Some(&#ns_name)));

            body = quote! {
                let output = helper.init_start_tag_deserializer(event, #ns_name, #name, #allow_any)?;
                match self.#handler_ident(helper, output, &mut fallback)? {
                    #element_handler_output::Continue { event, allow_any } => {
                        allow_any_element = allow_any_element || allow_any;

                        event
                    },
                    #element_handler_output::Break { event, allow_any } => break (event, allow_any),
                }
            }
        }

        let filter = self
            .treat_as_text()
            .not()
            .then(|| quote!(@ (#event::Start(_) | #event::Empty(_))));

        quote! {
            (S::#variant_ident(None), event #filter) => {
                #body
            }
        }
    }
}

impl Context<'_, '_> {
    fn do_box(&self, is_boxed: bool, tokens: TokenStream) -> TokenStream {
        if is_boxed {
            let box_ = self.resolve_build_in("::alloc::boxed::Box");

            quote!(#box_::new(#tokens))
        } else {
            tokens
        }
    }

    fn set_content(&mut self, value: bool) {
        self.values.get_or_create::<DefaultableCache>().content = value;
    }

    fn is_defaultable_type(&mut self) -> bool {
        let cache = self.values.get_or_create::<DefaultableCache>();
        let content = cache.content;
        let defaultable = cache.get_defaultable(self.meta.types.meta.types, self.ident);

        matches!(
            (content, defaultable),
            (true, Defaultable::Complete | Defaultable::Content) | (false, Defaultable::Complete)
        )
    }

    fn has_defaultable_content(&mut self) -> bool {
        self.values
            .get_or_create::<DefaultableCache>()
            .type_has_defaultable_content(self.meta.types.meta.types, self.ident)
    }

    fn is_defaultable_element(&mut self, el: &ElementMeta) -> bool {
        self.values
            .get_or_create::<DefaultableCache>()
            .is_defaultable_element(self.meta.types.meta.types, el)
    }
}

impl MaxOccurs {
    fn render_opt(&self) -> TokenStream {
        match self {
            Self::Unbounded => quote!(None),
            Self::Bounded(sz) => quote!(Some(#sz)),
        }
    }
}

fn boxed_deserializer_ident(is_boxed: bool, deserializer_ident: &Ident2) -> Ident2 {
    if is_boxed {
        deserializer_ident.clone()
    } else {
        format_ident!("Self")
    }
}

#[derive(Default, Debug)]
struct DefaultableCache {
    cache: HashMap<TypeIdent, Option<Defaultable>>,
    content: bool,
}

#[derive(Default, Debug, Clone, Copy)]
enum Defaultable {
    #[default]
    None,
    Content,
    Complete,
}

impl ValueKey for DefaultableCache {
    type Type = Self;
}

impl DefaultableCache {
    fn is_defaultable_type(&mut self, types: &MetaTypes, ident: &TypeIdent) -> bool {
        matches!(
            self.get_defaultable_impl(types, ident),
            Some(Defaultable::Complete)
        )
    }

    fn type_has_defaultable_content(&mut self, types: &MetaTypes, ident: &TypeIdent) -> bool {
        matches!(
            self.get_defaultable_impl(types, ident),
            Some(Defaultable::Complete | Defaultable::Content)
        )
    }

    fn is_defaultable_element(&mut self, types: &MetaTypes, el: &ElementMeta) -> bool {
        if let ElementMetaVariant::Type {
            type_,
            mode: ElementMode::Group,
        } = &el.variant
        {
            self.is_defaultable_type(types, type_)
        } else {
            false
        }
    }

    fn get_defaultable(&mut self, types: &MetaTypes, ident: &TypeIdent) -> Defaultable {
        self.get_defaultable_impl(types, ident).unwrap_or_default()
    }

    fn get_defaultable_impl(
        &mut self,
        types: &MetaTypes,
        ident: &TypeIdent,
    ) -> Option<Defaultable> {
        let create = match self.cache.entry(ident.clone()) {
            Entry::Vacant(e) => {
                e.insert(None);

                true
            }
            Entry::Occupied(_) => false,
        };

        let new_value = create.then(|| self.make_new_entry(types, ident));

        match self.cache.entry(ident.clone()) {
            Entry::Occupied(mut e) => {
                if let Some(value) = new_value {
                    e.insert(Some(value));
                }

                *e.into_mut()
            }
            Entry::Vacant(_) => unreachable!(),
        }
    }

    fn make_new_entry(&mut self, types: &MetaTypes, ident: &TypeIdent) -> Defaultable {
        match types.get_variant(ident) {
            None
            | Some(
                MetaTypeVariant::Union(_)
                | MetaTypeVariant::BuildIn(_)
                | MetaTypeVariant::Custom(_)
                | MetaTypeVariant::Reference(_)
                | MetaTypeVariant::Enumeration(_)
                | MetaTypeVariant::Dynamic(_)
                | MetaTypeVariant::SimpleType(_),
            ) => Defaultable::None,
            Some(MetaTypeVariant::All(x) | MetaTypeVariant::Sequence(x)) => {
                let mut defaultable = Defaultable::Complete;

                for el in &*x.elements {
                    if el.min_occurs > 0 && !self.is_defaultable_element(types, el) {
                        defaultable = Defaultable::None;
                        break;
                    }
                }

                defaultable
            }
            Some(MetaTypeVariant::Choice(x)) => {
                let defaultable = x.elements.len() == 1
                    && x.elements[0].min_occurs >= 1
                    && matches!(
                        &x.elements[0].variant,
                        ElementMetaVariant::Type {
                            mode: ElementMode::Group,
                            ..
                        }
                    );

                if defaultable {
                    Defaultable::Complete
                } else {
                    Defaultable::None
                }
            }
            Some(MetaTypeVariant::ComplexType(x)) => {
                let is_defaultable = x
                    .content
                    .as_ref()
                    .is_some_and(|ident| self.is_defaultable_type(types, ident));

                if is_defaultable {
                    Defaultable::Content
                } else {
                    Defaultable::None
                }
            }
        }
    }
}
