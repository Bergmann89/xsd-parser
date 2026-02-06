use std::cell::RefCell;
use std::collections::{
    btree_map::Entry, hash_map::Entry as HashMapEntry, BTreeMap, HashMap, HashSet,
};
use std::mem::replace;
use std::ops::Not;
use std::rc::Rc;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote};

use xsd_parser_types::misc::Namespace;

use crate::config::{GeneratorFlags, TypedefMode};
use crate::models::{
    code::IdentPath,
    data::{
        ComplexBase, ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement,
        ComplexDataEnum, ComplexDataStruct, DataTypeVariant, DynamicData, EnumerationData,
        EnumerationTypeVariant, Occurs, PathData, ReferenceData, SimpleData, UnionData,
        UnionTypeVariant,
    },
    meta::{CustomMetaNamespace, ElementMetaVariant, MetaTypeVariant, MetaTypes},
    schema::{xs::FormChoiceType, NamespaceId},
    TypeIdent,
};

use super::super::super::{
    context::{Context, ValueKey},
    MetaData, RenderStep, RenderStepType,
};

/// Implements a [`RenderStep`] that renders the code for the `quick_xml` serialization.
#[derive(Debug, Clone)]
pub struct QuickXmlSerializeRenderStep {
    /// How to serialize namespace definitions.
    pub namespaces: NamespaceSerialization,

    /// Default namespace to use for the serialization.
    pub default_namespace: Option<Namespace>,
}

/// Defines how the [`QuickXmlSerializeRenderStep`] will serialize the different
/// namespaces of the elements and attributes.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NamespaceSerialization {
    /// Do not serialize any namespace definitions.
    None,

    /// All namespace definitions are added to the local element if they are
    /// not already in scope.
    Local,

    /// All namespace definitions are serialized to the root element.
    Global,
}

impl NamespaceSerialization {
    /// Returns `true` if this is [`NamespaceSerialization::None`], `false` otherwise.
    #[inline]
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if this is [`NamespaceSerialization::Local`] or
    /// [`NamespaceSerialization::Global`], `false` otherwise.
    #[inline]
    #[must_use]
    pub fn is_some(&self) -> bool {
        matches!(self, Self::Local | Self::Global)
    }
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

#[derive(Default, Debug)]
struct NamespaceCollector {
    cache: HashMap<TypeIdent, Option<SharedGlobalNamespaces>>,
    xsi_namespace: Option<NamespaceId>,
    nillable_type_support: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum NamespaceKey {
    Normal(NamespaceId),
    Default(NamespaceId),
}

type GlobalNamespaces = BTreeMap<NamespaceKey, (Option<PathData>, PathData)>;
type SharedGlobalNamespaces = Rc<GlobalNamespaces>;

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

        let str_ = resolve_build_in!(ctx, "::core::primitive::str");
        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");
        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serialize_bytes = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeBytes");
        let serialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

        let code = quote! {
            impl #serialize_bytes for #type_ident {
                fn serialize_bytes(&self, helper: &mut #serialize_helper) -> #result<#option<#cow<'_, #str_>>, #error> {
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
            Self::#variant_ident(x) => x.serialize_bytes(helper),
        }
    }
}

/* DynamicData */

impl DynamicData<'_> {
    pub(crate) fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        let Self { type_ident, .. } = self;

        let str_ = resolve_build_in!(ctx, "::core::primitive::str");
        let bool_ = resolve_build_in!(ctx, "::core::primitive::bool");
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
                    name: #option<&'ser #str_>,
                    is_root: #bool_
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
                    self.0.serialize_bytes(helper)
                }
            }
            Occurs::Optional => {
                quote! {
                    if let Some(inner) = &self.0 {
                        Ok(Some(inner.serialize_bytes(helper)?))
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
                        if let Some(bytes) = item.serialize_bytes(helper)? {
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

        let str_ = resolve_build_in!(ctx, "::core::primitive::str");
        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");
        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serialize_bytes = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeBytes");
        let serialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

        let code = quote! {
            impl #serialize_bytes for #type_ident {
                fn serialize_bytes(&self, helper: &mut #serialize_helper) -> #result<#option<#cow<'_, #str_>>, #error> {
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

        let str_ = resolve_build_in!(ctx, "::core::primitive::str");
        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");
        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serialize_bytes = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeBytes");
        let serialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

        let code = quote! {
            impl #serialize_bytes for #type_ident {
                fn serialize_bytes(&self, helper: &mut #serialize_helper) -> #result<#option<#cow<'_, #str_>>, #error> {
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
                Self::#variant_ident(x) => x.serialize_bytes(helper),
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

        let str_ = resolve_build_in!(ctx, "::core::primitive::str");
        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let cow = resolve_ident!(ctx, "::alloc::borrow::Cow");
        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serialize_bytes = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeBytes");
        let serialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

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
                    if let Some(bytes) = item.serialize_bytes(helper)? {
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
                self.0.serialize_bytes(helper)
            }
        };

        let code = quote! {
            impl #serialize_bytes for #type_ident {
                fn serialize_bytes(&self, helper: &mut #serialize_helper) -> #result<#option<#cow<'_, #str_>>, #error> {
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

        let str_ = resolve_build_in!(ctx, "::core::primitive::str");
        let bool_ = resolve_build_in!(ctx, "::core::primitive::bool");
        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let error = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let with_serializer = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::WithSerializer");

        let code = quote! {
            impl #with_serializer for #type_ident {
                type Serializer<'x> = quick_xml_serialize::#serializer_ident<'x>;

                fn serializer<'ser>(
                    &'ser self,
                    name: #option<&'ser #str_>,
                    is_root: #bool_,
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

        let box_ = resolve_build_in!(ctx, "::alloc::boxed::Box");
        let str_ = resolve_build_in!(ctx, "::core::primitive::str");
        let bool_ = resolve_build_in!(ctx, "::core::primitive::bool");

        let name = self.represents_element().then(|| {
            quote! {
                pub(super) name: &'ser #str_,
            }
        });
        let is_root = forward_root.then(|| {
            quote! {
                pub(super) is_root: #bool_,
            }
        });

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

    fn render_serializer_handle_state_end(
        &self,
        ctx: &Context<'_, '_>,
        need_ns_scope: bool,
    ) -> TokenStream {
        let serializer_state_ident = &self.serializer_state_ident;

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let bytes_end = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::BytesEnd");

        let end_ns_scope = need_ns_scope.then(|| quote!(helper.end_ns_scope();));

        quote! {
            #serializer_state_ident::End__ => {
                *self.state = #serializer_state_ident::Done__;

                #end_ns_scope

                return Ok(Some(
                    #event::End(
                        #bytes_end::new(self.name))
                    )
                );
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn render_serializer_bytes_ctor(
        &self,
        ctx: &mut Context<'_, '_>,
        attributes: &[ComplexDataAttribute<'_>],
    ) -> (TokenStream, bool) {
        let attributes_ctor = attributes.iter().map(|attrib| {
            let attrib_name = attrib.tag_name.get(true);
            let field_ident = &attrib.ident;

            if attrib.meta.is_any() {
                quote! {
                    bytes.extend_attributes(self.value.#field_ident.attributes());
                }
            } else if attrib.is_option {
                quote! {
                    helper.write_attrib_opt(&mut bytes, #attrib_name, &self.value.#field_ident)?;
                }
            } else {
                quote! {
                    helper.write_attrib(&mut bytes, #attrib_name, &self.value.#field_ident)?;
                }
            }
        });

        let config = ctx.get_ref::<SerializerConfig>();
        let namespaces = config.namespaces;
        let default_namespace = config.default_namespace.clone();
        let xmlns = match namespaces {
            NamespaceSerialization::None => None,
            NamespaceSerialization::Local => {
                let mut xmlns = Vec::new();
                let mut cache = HashSet::new();

                let element_module = self
                    .tag_name
                    .as_ref()
                    .and_then(|tag| {
                        let module = tag.module?;
                        let form = if matches!((&default_namespace, &module.namespace), (Some(a), Some(b)) if a == b) {
                            FormChoiceType::Unqualified
                        } else {
                            tag.form
                        };

                        Some((form, module))
                    })
                    .into_iter();
                let attribute_modules = attributes
                    .iter()
                    .filter_map(|attrib| {
                        (attrib.tag_name.form == FormChoiceType::Qualified)
                            .then_some(attrib.tag_name.module?)
                    })
                    .map(|module| (FormChoiceType::Qualified, module));

                for (form, module) in element_module.chain(attribute_modules) {
                    if !cache.insert(module.namespace_id) {
                        continue;
                    }

                    let Some(path) = module.make_ns_const() else {
                        continue;
                    };

                    let ns_const = ctx.resolve_type_for_serialize_module(&path);
                    let prefix_const = (form == FormChoiceType::Qualified)
                        .then(|| module.make_prefix_const())
                        .flatten()
                        .map(|path| ctx.resolve_type_for_serialize_module(&path))
                        .map_or_else(|| quote!(None), |x| quote!(Some(&#x)));

                    xmlns.push(quote! {
                        helper.write_xmlns(&mut bytes, #prefix_const, &#ns_const);
                    });
                }

                xmlns.is_empty().not().then(|| {
                    quote! {
                        #( #xmlns )*
                    }
                })
            }
            NamespaceSerialization::Global => {
                let nillable_type_support =
                    ctx.check_generator_flags(GeneratorFlags::NILLABLE_TYPE_SUPPORT);
                let types: &MetaTypes = ctx.types;
                let collector = ctx
                    .get_or_create_with::<NamespaceCollector, _>(|| {
                        NamespaceCollector::new(types, nillable_type_support)
                    })
                    .clone();
                let mut collector = collector.borrow_mut();

                let xmlns = self.tag_name.as_ref().and_then(|tag| {
                    let module = tag.module?;
                    if tag.form != FormChoiceType::Qualified || module.prefix().is_some() {
                        return None;
                    }

                    let ns = module.make_ns_const()?;
                    let ns_const = ctx.resolve_type_for_serialize_module(&ns);

                    Some(quote!(helper.write_xmlns(&mut bytes, None, &#ns_const);))
                });

                let need_xsi_namespace =
                    nillable_type_support && !collector.provides_xsi_namespace();
                let xsi = need_xsi_namespace.then(|| {
                    let namespace = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::misc::Namespace");
                    let namespace_prefix = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::misc::NamespacePrefix");

                    quote!(helper.write_xmlns(&mut bytes, Some(&#namespace_prefix::XSI), &#namespace::XSI);)
                });

                let global_xmlns = collector
                    .get_namespaces(ctx.types, ctx.ident, default_namespace.as_ref())
                    .values()
                    .map(|(prefix, namespace)| {
                        let ns_const = ctx.resolve_type_for_serialize_module(namespace);
                        let prefix_const = prefix.as_ref().map_or_else(
                            || quote!(None),
                            |prefix| {
                                let x = ctx.resolve_type_for_serialize_module(prefix);

                                quote!(Some(&#x))
                            },
                        );

                        quote! {
                            helper.write_xmlns(&mut bytes, #prefix_const, &#ns_const);
                        }
                    });
                let global_xmlns = xsi.into_iter().chain(global_xmlns).collect::<Vec<_>>();
                let global_xmlns = global_xmlns.is_empty().not().then(|| {
                    quote! {
                        if self.is_root {
                            #( #global_xmlns )*
                        }
                    }
                });

                (xmlns.is_some() || global_xmlns.is_some()).then(|| {
                    quote! {
                        #xmlns
                        #global_xmlns
                    }
                })
            }
        };

        let mut_ = xmlns.is_some() || !attributes.is_empty();
        let mut_ = mut_.then(|| quote!(mut));

        let bytes_start =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::BytesStart");

        let need_ns_scope = xmlns.is_some();
        let begin_ns_scope = need_ns_scope.then(|| quote!(helper.begin_ns_scope();));
        let code = quote! {
            let #mut_ bytes = #bytes_start::new(self.name);
            #begin_ns_scope
            #xmlns
            #( #attributes_ctor )*
        };

        (code, need_ns_scope)
    }
}

impl ComplexDataEnum<'_> {
    fn serializer_need_end_state(&self) -> bool {
        self.represents_element()
    }

    fn render_serializer(&self, ctx: &mut Context<'_, '_>) {
        self.render_with_serializer(ctx, !self.is_content());
        self.render_serializer_type(ctx, !self.is_content());
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

        let (emit_start_event, handle_state_end) = self
            .serializer_need_end_state()
            .then(|| {
                let (emit_start_event, need_ns_scope) =
                    self.render_serializer_impl_start_event(ctx);
                let handle_state_end = self.render_serializer_handle_state_end(ctx, need_ns_scope);

                (emit_start_event, handle_state_end)
            })
            .unzip();

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
                !self.is_content(),
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
                    match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = #final_state,
                    }
                }
            }
        });

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serializer = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Serializer");
        let serialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

        ctx.add_quick_xml_serialize_usings(true, ["::xsd_parser_types::quick_xml::Serializer"]);

        let code = quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self, helper: &mut #serialize_helper) -> #result<#option<#event<'ser>>, #error> {
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

            impl<'ser> #serializer<'ser> for #serializer_ident<'ser> {
                fn next(&mut self, helper: &mut #serialize_helper) -> #option<#result<#event<'ser>, #error>> {
                    match self.next_event(helper) {
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

    fn render_serializer_impl_start_event(&self, ctx: &mut Context<'_, '_>) -> (TokenStream, bool) {
        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");

        let (bytes_ctor, need_ns_scope) = self.render_serializer_bytes_ctor(ctx, &[]);

        let code = quote! {
            #bytes_ctor
            return Ok(Some(#event::Start(bytes)))
        };

        (code, need_ns_scope)
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

        let (emit_start_event, handle_state_end) = self
            .represents_element()
            .then(|| {
                let (emit_start_event, need_ns_scope) =
                    self.render_serializer_impl_start_event(ctx);
                let handle_state_end = self
                    .serializer_need_end_state()
                    .then(|| self.render_serializer_handle_state_end(ctx, need_ns_scope));

                (emit_start_event, handle_state_end)
            })
            .unzip();
        let handle_state_end = handle_state_end.flatten();

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
                #serializer_state_ident::#variant_ident(x) => match x.next(helper).transpose()? {
                    Some(event) => return Ok(Some(event)),
                    None => #next
                }
            }
        });

        let handle_state_content = self.content().map(|_| {
            quote! {
                #serializer_state_ident::Content__(x) => match x.next(helper).transpose()? {
                    Some(event) => return Ok(Some(event)),
                    None => *self.state = #final_state,
                }
            }
        });

        let result = resolve_build_in!(ctx, "::core::result::Result");
        let option = resolve_build_in!(ctx, "::core::option::Option");

        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");
        let error = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Error");
        let serializer = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Serializer");
        let serialize_helper =
            resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

        ctx.add_quick_xml_serialize_usings(true, ["::xsd_parser_types::quick_xml::Serializer"]);

        let code = quote! {
            impl<'ser> #serializer_ident<'ser> {
                fn next_event(&mut self, helper: &mut #serialize_helper) -> #result<#option<#event<'ser>>, #error> {
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

            impl<'ser> #serializer<'ser> for #serializer_ident<'ser> {
                fn next(&mut self, helper: &mut #serialize_helper) -> #option<#result<#event<'ser>, #error>> {
                    match self.next_event(helper) {
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

    fn render_serializer_impl_start_event(&self, ctx: &mut Context<'_, '_>) -> (TokenStream, bool) {
        let event = resolve_quick_xml_ident!(ctx, "::xsd_parser_types::quick_xml::Event");

        let (bytes_ctor, need_ns_scope) = self.render_serializer_bytes_ctor(ctx, &self.attributes);
        let end_ns_scope =
            (need_ns_scope && !self.has_content()).then(|| quote!(helper.end_ns_scope();));

        let variant = if self.has_content() {
            format_ident!("Start")
        } else {
            format_ident!("Empty")
        };

        let code = quote! {
            #bytes_ctor
            #end_ns_scope

            return Ok(Some(#event::#variant(bytes)))
        };

        (code, need_ns_scope)
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

impl ValueKey for NamespaceCollector {
    type Type = Rc<RefCell<Self>>;
}

#[derive(Debug)]
enum GetNamespaceState {
    Empty,
    Weak(GlobalNamespaces),
    Strong(GlobalNamespaces),
    Shared(SharedGlobalNamespaces),
}

impl NamespaceCollector {
    fn new(types: &MetaTypes, nillable_type_support: bool) -> Rc<RefCell<Self>> {
        let xsi_namespace = types.modules.iter().find_map(|(id, module)| {
            if matches!(module.namespace.as_ref(), Some(ns) if *ns == Namespace::XSI) {
                Some(*id)
            } else {
                None
            }
        });

        Rc::new(RefCell::new(Self {
            cache: HashMap::new(),
            xsi_namespace,
            nillable_type_support,
        }))
    }

    fn provides_xsi_namespace(&self) -> bool {
        self.xsi_namespace.is_some()
    }

    fn get_namespaces(
        &mut self,
        types: &MetaTypes,
        ident: &TypeIdent,
        default_ns: Option<&Namespace>,
    ) -> &SharedGlobalNamespaces {
        self.get_namespaces_impl(types, ident, default_ns).unwrap()
    }

    #[allow(clippy::too_many_lines)]
    fn get_namespaces_impl(
        &mut self,
        types: &MetaTypes,
        ident: &TypeIdent,
        default_ns: Option<&Namespace>,
    ) -> Option<&SharedGlobalNamespaces> {
        let create = match self.cache.entry(ident.clone()) {
            HashMapEntry::Vacant(e) => {
                e.insert(None);

                true
            }
            HashMapEntry::Occupied(_) => false,
        };

        let new_value = if create {
            let ty = types.items.get(ident).unwrap();
            let mut state = GetNamespaceState::Empty;

            if self.nillable_type_support {
                if let Some(id) = &self.xsi_namespace {
                    Self::add_ns(&mut state, types, NamespaceKey::Normal(*id));
                }
            }

            match &ty.variant {
                MetaTypeVariant::Union(x) => {
                    for ty in &*x.types {
                        self.merge(&mut state, types, &ty.type_, default_ns);
                    }
                }
                MetaTypeVariant::Reference(x) => {
                    self.merge(&mut state, types, &x.type_, default_ns);
                }
                MetaTypeVariant::Enumeration(x) => {
                    for var in &*x.variants {
                        if let Some(ident) = &var.type_ {
                            self.merge(&mut state, types, ident, default_ns);
                        }
                    }
                }
                MetaTypeVariant::Dynamic(x) => {
                    for ident in &x.derived_types {
                        self.merge(&mut state, types, ident, default_ns);
                    }
                }
                MetaTypeVariant::All(x)
                | MetaTypeVariant::Choice(x)
                | MetaTypeVariant::Sequence(x) => {
                    for el in &*x.elements {
                        if let ElementMetaVariant::Type { type_, .. } = &el.variant {
                            self.merge(&mut state, types, type_, default_ns);
                        }
                    }
                }
                MetaTypeVariant::ComplexType(x) => {
                    if let Some(ident) = &x.content {
                        self.merge(&mut state, types, ident, default_ns);
                    }

                    for attrib in &*x.attributes {
                        if attrib.form == FormChoiceType::Qualified {
                            Self::add_ns(&mut state, types, NamespaceKey::Normal(attrib.ident.ns));
                        }
                    }
                }
                MetaTypeVariant::Custom(x) => {
                    for ns in x.namespaces() {
                        let key = match ns {
                            CustomMetaNamespace::Id(id) => NamespaceKey::Default(*id),
                            CustomMetaNamespace::Namespace(ns) => {
                                let id = types.modules.iter().find_map(|(id, module)| {
                                    matches!(module.namespace.as_ref(), Some(x) if x == ns)
                                        .then_some(*id)
                                });
                                let Some(id) = id else {
                                    continue;
                                };

                                NamespaceKey::Normal(id)
                            }
                        };

                        Self::add_ns(&mut state, types, key);
                    }
                }
                MetaTypeVariant::BuildIn(_) | MetaTypeVariant::SimpleType(_) => (),
            }

            if ty.form() == FormChoiceType::Qualified {
                let ns = types
                    .modules
                    .get(&ident.ns)
                    .and_then(|module| module.namespace.as_ref());

                let key = if matches!((ns, default_ns), (Some(a), Some(b)) if a == b) {
                    NamespaceKey::Default(ident.ns)
                } else {
                    NamespaceKey::Normal(ident.ns)
                };

                Self::add_ns(&mut state, types, key);
            }

            let value = match state {
                GetNamespaceState::Empty => Default::default(),
                GetNamespaceState::Shared(value) => value,
                GetNamespaceState::Weak(value) | GetNamespaceState::Strong(value) => Rc::new(value),
            };

            Some(value)
        } else {
            None
        };

        match self.cache.entry(ident.clone()) {
            HashMapEntry::Occupied(mut e) => {
                if let Some(value) = new_value {
                    e.insert(Some(value));
                }

                e.into_mut().as_ref()
            }
            HashMapEntry::Vacant(_) => unreachable!(),
        }
    }

    fn merge(
        &mut self,
        state: &mut GetNamespaceState,
        types: &MetaTypes,
        ident: &TypeIdent,
        default_ns: Option<&Namespace>,
    ) {
        let Some(src) = self.get_namespaces_impl(types, ident, default_ns) else {
            return;
        };

        match replace(state, GetNamespaceState::Empty) {
            GetNamespaceState::Empty => *state = GetNamespaceState::Shared(src.clone()),
            GetNamespaceState::Weak(dst) => {
                *state = GetNamespaceState::Shared(src.clone());

                for ns in dst.into_keys() {
                    Self::add_ns(state, types, ns);
                }
            }
            GetNamespaceState::Shared(dst) if Rc::ptr_eq(&dst, src) => {
                *state = GetNamespaceState::Shared(dst);
            }
            s => {
                *state = s;

                for ns in src.keys() {
                    Self::add_ns(state, types, *ns);
                }
            }
        }
    }

    fn add_ns(state: &mut GetNamespaceState, types: &MetaTypes, key: NamespaceKey) {
        match state {
            GetNamespaceState::Empty => {
                if let Some(value) = Self::make_value(types, key) {
                    let mut map = GlobalNamespaces::new();
                    map.insert(key, value);

                    *state = GetNamespaceState::Weak(map);
                }
            }
            GetNamespaceState::Weak(map) | GetNamespaceState::Strong(map) => {
                if let Entry::Vacant(e) = map.entry(key) {
                    if let Some(value) = Self::make_value(types, key) {
                        e.insert(value);
                    }
                }
            }
            GetNamespaceState::Shared(map) => {
                if !map.contains_key(&key) {
                    if let Some(value) = Self::make_value(types, key) {
                        let mut map = (**map).clone();
                        map.insert(key, value);

                        *state = GetNamespaceState::Strong(map);
                    }
                }
            }
        }
    }

    fn make_value(types: &MetaTypes, key: NamespaceKey) -> Option<(Option<PathData>, PathData)> {
        match key {
            NamespaceKey::Normal(id) => {
                let module = types.modules.get(&id)?;

                let prefix = Some(module.make_prefix_const()?);
                let namespace = module.make_ns_const()?;

                Some((prefix, namespace))
            }
            NamespaceKey::Default(id) => {
                let module = types.modules.get(&id)?;
                let namespace = module.make_ns_const()?;

                Some((None, namespace))
            }
        }
    }
}
