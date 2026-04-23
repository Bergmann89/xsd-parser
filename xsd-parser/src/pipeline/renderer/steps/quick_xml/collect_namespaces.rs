use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote};

use crate::config::TypedefMode;
use crate::models::data::TagName;
use crate::models::{
    code::IdentPath,
    data::{
        ComplexBase, ComplexData, ComplexDataAttribute, ComplexDataElement, ComplexDataEnum,
        ComplexDataStruct, DynamicData, EnumerationData, Occurs, ReferenceData, SimpleData,
        StructMode, UnionData,
    },
    schema::xs::FormChoiceType,
};

use super::super::super::{
    context::Context, DataTypeVariant, MetaData, RenderStep, RenderStepType,
};

macro_rules! resolve_ident {
    ($ctx:ident, $path:expr) => {
        $ctx.resolve_ident_path($path)
    };
}

/// Implements a [`RenderStep`] that renders the [`CollectNamespaces`](xsd_parser_types::quick_xml::CollectNamespaces)
/// trait for every generated type.
///
/// This is required when using [`NamespaceSerialization::Dynamic`](super::NamespaceSerialization::Dynamic)
/// so that the serializer can traverse the value tree at runtime to discover which
/// XML namespaces are actually needed before emitting the root start element.
#[derive(Debug, Clone, Copy)]
pub struct QuickXmlCollectNamespacesRenderStep;

impl RenderStep for QuickXmlCollectNamespacesRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::ExtraImpls
    }

    fn initialize(&mut self, meta: &mut MetaData<'_>) {
        let ident = IdentPath::from_parts(
            [meta.xsd_parser_types.clone(), format_ident!("quick_xml")],
            format_ident!("CollectNamespaces"),
        );

        if !meta.dyn_type_traits.contains(&ident) {
            meta.dyn_type_traits.push(ident);
        }
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) | DataTypeVariant::Custom(_) => (),
            DataTypeVariant::Union(x) => x.render_collect_namespaces(ctx),
            DataTypeVariant::Dynamic(x) => x.render_collect_namespaces(ctx),
            DataTypeVariant::Reference(x) => x.render_collect_namespaces(ctx),
            DataTypeVariant::Enumeration(x) => x.render_collect_namespaces(ctx),
            DataTypeVariant::Simple(x) => x.render_collect_namespaces(ctx),
            DataTypeVariant::Complex(x) => x.render_collect_namespaces(ctx),
        }
    }
}

/* UnionData */

impl UnionData<'_> {
    pub(crate) fn render_collect_namespaces(&self, ctx: &mut Context<'_, '_>) {
        ctx.render_collect_namespaces_noop(&self.type_ident);
    }
}

/* DynamicData */

impl DynamicData<'_> {
    pub(crate) fn render_collect_namespaces(&self, ctx: &mut Context<'_, '_>) {
        let Self { type_ident, .. } = self;

        let bytes_start = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::BytesStart");
        let collect_namespaces =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::CollectNamespaces");
        let serialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

        let code = quote! {
            impl #collect_namespaces for #type_ident {
                fn collect_namespaces(&self, helper: &mut #serialize_helper, bytes: &mut #bytes_start<'_>) {
                    self.0.collect_namespaces(helper, bytes);
                }
            }
        };

        ctx.current_module().append(code);
    }
}

/* ReferenceData */

impl ReferenceData<'_> {
    pub(crate) fn render_collect_namespaces(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            mode, type_ident, ..
        } = self;

        if matches!(mode, TypedefMode::Auto | TypedefMode::Typedef) {
            return;
        }

        let bytes_start = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::BytesStart");
        let collect_namespaces =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::CollectNamespaces");
        let serialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

        let code = quote! {
            impl #collect_namespaces for #type_ident {
                fn collect_namespaces(&self, helper: &mut #serialize_helper, bytes: &mut #bytes_start<'_>) {
                    self.0.collect_namespaces(helper, bytes);
                }
            }
        };

        ctx.current_module().append(code);
    }
}

/* EnumerationData */

impl EnumerationData<'_> {
    pub(crate) fn render_collect_namespaces(&self, ctx: &mut Context<'_, '_>) {
        ctx.render_collect_namespaces_noop(&self.type_ident);
    }
}

/* SimpleData */

impl SimpleData<'_> {
    pub(crate) fn render_collect_namespaces(&self, ctx: &mut Context<'_, '_>) {
        ctx.render_collect_namespaces_noop(&self.type_ident);
    }
}

/* ComplexData */

impl ComplexData<'_> {
    pub(crate) fn render_collect_namespaces(&self, ctx: &mut Context<'_, '_>) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_collect_namespaces(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_collect_namespaces(ctx);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_collect_namespaces(ctx);

                if let Some(content_type) = content_type {
                    content_type.render_collect_namespaces(ctx);
                }
            }
        }
    }
}

impl ComplexDataEnum<'_> {
    fn render_collect_namespaces(&self, ctx: &mut Context<'_, '_>) {
        let type_ident = &self.type_ident;

        let own_ns = self.render_collect_own_namespace(ctx);

        let variants = self
            .elements
            .iter()
            .map(ComplexDataElement::render_collect_namespaces_match_arm);

        let bytes_start = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::BytesStart");
        let collect_namespaces =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::CollectNamespaces");
        let serialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

        let code = quote! {
            impl #collect_namespaces for #type_ident {
                fn collect_namespaces(&self, helper: &mut #serialize_helper, bytes: &mut #bytes_start<'_>) {
                    #own_ns
                    match self {
                        #( #variants )*
                    }
                }
            }
        };

        ctx.current_module().append(code);
    }
}

impl ComplexDataStruct<'_> {
    fn render_collect_namespaces(&self, ctx: &mut Context<'_, '_>) {
        let type_ident = &self.type_ident;

        let own_ns = self.render_collect_own_namespace(ctx);
        let attrib_ns = self.render_collect_attribute_namespaces(ctx);

        let field_calls: Vec<_> = match &self.mode {
            StructMode::Empty { .. } => Vec::new(),
            StructMode::Content { content } => {
                let field_ident = &content.field_ident;

                vec![quote! {
                    self.#field_ident.collect_namespaces(helper, bytes);
                }]
            }
            StructMode::All { elements, .. } | StructMode::Sequence { elements, .. } => elements
                .iter()
                .map(ComplexDataElement::render_collect_namespaces_field_call)
                .collect(),
        };

        let bytes_start = resolve_ident!(ctx, "::xsd_parser_types::quick_xml::BytesStart");
        let collect_namespaces =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::CollectNamespaces");
        let serialize_helper =
            resolve_ident!(ctx, "::xsd_parser_types::quick_xml::SerializeHelper");

        let code = quote! {
            impl #collect_namespaces for #type_ident {
                fn collect_namespaces(&self, helper: &mut #serialize_helper, bytes: &mut #bytes_start<'_>) {
                    #own_ns
                    #( #attrib_ns )*
                    #( #field_calls )*
                }
            }
        };

        ctx.current_module().append(code);
    }
}

impl ComplexBase<'_> {
    fn render_collect_own_namespace(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        self.tag_name.as_ref()?.render_collect_namespace(ctx)
    }
}

impl ComplexDataStruct<'_> {
    fn render_collect_attribute_namespaces(&self, ctx: &Context<'_, '_>) -> Vec<TokenStream> {
        self.attributes
            .iter()
            .filter_map(|attrib| attrib.render_collect_attribute_namespace(ctx))
            .collect()
    }
}

impl ComplexDataAttribute<'_> {
    fn render_collect_attribute_namespace(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        self.tag_name.render_collect_namespace(ctx)
    }
}

impl ComplexDataElement<'_> {
    fn render_collect_namespaces_field_call(&self) -> TokenStream {
        let field_ident = &self.field_ident;

        match self.occurs {
            Occurs::None => unreachable!(),
            Occurs::Single => quote! {
                self.#field_ident.collect_namespaces(helper, bytes);
            },
            Occurs::Optional => quote! {
                if let Some(inner) = &self.#field_ident {
                    inner.collect_namespaces(helper, bytes);
                }
            },
            Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                for item in &self.#field_ident {
                    item.collect_namespaces(helper, bytes);
                }
            },
        }
    }

    fn render_collect_namespaces_match_arm(&self) -> TokenStream {
        let variant_ident = &self.variant_ident;

        match self.occurs {
            Occurs::None => unreachable!(),
            Occurs::Single => quote! {
                Self::#variant_ident(x) => x.collect_namespaces(helper, bytes),
            },
            Occurs::Optional => quote! {
                Self::#variant_ident(x) => {
                    if let Some(inner) = x {
                        inner.collect_namespaces(helper, bytes);
                    }
                }
            },
            Occurs::DynamicList | Occurs::StaticList(_) => quote! {
                Self::#variant_ident(x) => {
                    for item in x.iter() {
                        item.collect_namespaces(helper, bytes);
                    }
                }
            },
        }
    }
}

/* Misc */

impl Context<'_, '_> {
    fn render_collect_namespaces_noop(&mut self, type_ident: &Ident2) {
        let bytes_start = resolve_ident!(self, "::xsd_parser_types::quick_xml::BytesStart");
        let collect_namespaces =
            resolve_ident!(self, "::xsd_parser_types::quick_xml::CollectNamespaces");
        let serialize_helper =
            resolve_ident!(self, "::xsd_parser_types::quick_xml::SerializeHelper");

        let code = quote! {
            impl #collect_namespaces for #type_ident {
                fn collect_namespaces(&self, _helper: &mut #serialize_helper, _bytes: &mut #bytes_start<'_>) { }
            }
        };

        self.current_module().append(code);
    }
}

impl TagName<'_> {
    fn render_collect_namespace(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        if self.form != FormChoiceType::Qualified {
            return None;
        }

        let module = self.module?;
        let ns = module.make_ns_const()?;
        let ns_const = ctx.resolve_type_for_module(&ns);

        let prefix = module.make_prefix_const()?;
        let prefix_const = ctx.resolve_type_for_module(&prefix);

        Some(quote! {
            helper.write_xmlns(bytes, Some(&#prefix_const), &#ns_const);
        })
    }
}
