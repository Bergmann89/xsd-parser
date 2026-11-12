use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote};

use crate::config::{RendererFlags, TypedefMode};
use crate::models::{
    data::{
        ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement, ComplexDataEnum,
        ComplexDataStruct, DynamicData, EnumerationData, EnumerationDataVariant, Occurs,
        ReferenceData, SimpleData, UnionData, UnionTypeVariant,
    },
    meta::{MetaType, MetaTypeVariant},
    schema::xs::Use,
    TypeIdent,
};
use crate::traits::Naming;

use super::super::super::{Context, DataTypeVariant, RenderStep, RenderStepType};
use super::super::{format_traits, render_trait_impls};
use super::{get_derive, get_dyn_type_traits, misc::SerdeAttribute, SerdeOptions};

/// Implements a [`RenderStep`] that renders rust types of the types defined in
/// the schema with `serde-xml-rs >= 0.8` support.
#[derive(Debug, Default, Clone, Copy)]
pub struct SerdeXmlRsV8TypesRenderStep {
    /// Adapt arrays longer than 32 elements. Disabled by default.
    /// Requires `serde_with` with `std` or `alloc` in the generated-code crate.
    pub large_arrays: bool,

    /// Skip serializing named fields containing `None`. Disabled by default.
    pub skip_none: bool,

    /// Skip serializing named fields containing empty vectors. Disabled by default.
    pub skip_empty_vecs: bool,
}

impl RenderStep for SerdeXmlRsV8TypesRenderStep {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::Types
    }

    fn render_type(&mut self, ctx: &mut Context<'_, '_>) {
        let options = SerdeOptions {
            large_arrays: self.large_arrays,
            skip_none: self.skip_none,
            skip_empty_vecs: self.skip_empty_vecs,
            arrays_as_vec: true,
        };
        match &ctx.data.variant {
            DataTypeVariant::BuildIn(_) | DataTypeVariant::Custom(_) => (),
            DataTypeVariant::Union(x) => x.render_type_serde_xml_rs_v8(ctx, options),
            DataTypeVariant::Dynamic(x) => x.render_type_serde_xml_rs_v8(ctx),
            DataTypeVariant::Reference(x) => x.render_type_serde_xml_rs_v8(ctx, options),
            DataTypeVariant::Enumeration(x) => x.render_type_serde_xml_rs_v8(ctx, options),
            DataTypeVariant::Simple(x) => x.render_type_serde_xml_rs_v8(ctx, options),
            DataTypeVariant::Complex(x) => x.render_type_serde_xml_rs_v8(ctx, options),
        }
    }
}

/* Context */

impl<'a> Context<'a, '_> {
    fn get_resolved_complex_content(&self) -> Option<(&'a TypeIdent, &'a MetaType)> {
        let MetaTypeVariant::ComplexType(cm) = &self.data.meta.variant else {
            return None;
        };

        let content_ident = cm.content.as_ref()?;

        self.meta.types.meta.types.get_resolved(content_ident)
    }
}

/* UnionData */

impl UnionData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>, options: SerdeOptions) {
        let Self {
            type_ident,
            trait_impls,
            variants,
            ..
        } = self;
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let trait_impls = render_trait_impls(type_ident, trait_impls);
        let variants = variants
            .iter()
            .map(|x| x.render_variant_serde_xml_rs_v8(ctx, options));
        let extra_attributes = &ctx.data.extra_attributes;

        let code = quote! {
            #docs
            #derive
            #( #[#extra_attributes] )*
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        };

        ctx.current_module().append(code).export(type_ident);

        self.render_common_impls(ctx);
    }
}

/* UnionTypeVariant */

impl UnionTypeVariant<'_> {
    fn render_variant_serde_xml_rs_v8(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> TokenStream {
        let serde_attribute = SerdeAttribute::new(&self.extra_attributes).with_field(
            ctx,
            options,
            self.serde_field(),
        );
        let Self {
            target_type,
            variant_ident,
            extra_attributes,
            ..
        } = self;

        let target_type = ctx.resolve_type_for_module(target_type);

        quote! {
            #( #[#extra_attributes] )*
            #variant_ident ( #serde_attribute #target_type ),
        }
    }
}

/* DynamicData */

impl DynamicData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            trait_ident,
            sub_traits,
            ..
        } = self;

        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let trait_impls = render_trait_impls(type_ident, &[]);
        let dyn_traits = sub_traits.as_ref().map_or_else(
            || get_dyn_type_traits(ctx),
            |traits| format_traits(traits.iter().map(|x| ctx.resolve_type_for_module(x))),
        );
        let extra_attributes = &ctx.data.extra_attributes;

        let box_ = ctx.resolve_build_in("::alloc::boxed::Box");

        let code = quote! {
            #docs
            #derive
            #( #[#extra_attributes] )*
            pub struct #type_ident(pub #box_<dyn #trait_ident>);

            pub trait #trait_ident: #dyn_traits { }

            #( #trait_impls )*
        };

        ctx.current_module()
            .append(code)
            .export(type_ident)
            .export(trait_ident);
    }
}

/* ReferenceData */

impl ReferenceData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>, options: SerdeOptions) {
        let serde_attribute =
            SerdeAttribute::new(&[]).with_field(ctx, options, self.serde_field(options));
        let Self {
            mode,
            occurs,
            type_ident,
            target_type,
            trait_impls,
            ..
        } = self;

        let docs = ctx.render_type_docs();
        let occurs = occurs.array_to_vec();
        let target_type = ctx.resolve_type_for_module(target_type);

        let code = match mode {
            TypedefMode::Auto => crate::unreachable!(),
            TypedefMode::Typedef => {
                let target_type = occurs.make_type(ctx, &target_type, false);
                let trait_impls = render_trait_impls(type_ident, trait_impls);

                quote! {
                    #docs
                    pub type #type_ident = #target_type;

                    #( #trait_impls )*
                }
            }
            TypedefMode::NewType => {
                let target_type = occurs.make_type(ctx, &target_type, false);
                let extra_derive =
                    matches!(occurs, Occurs::Optional | Occurs::DynamicList).then_some("Default");
                let derive = get_derive(ctx, extra_derive);
                let trait_impls = render_trait_impls(type_ident, trait_impls);
                let extra_attributes = &ctx.data.extra_attributes;

                quote! {
                    #docs
                    #derive
                    #( #[#extra_attributes] )*
                    pub struct #type_ident(#serde_attribute pub #target_type);

                    #( #trait_impls )*
                }
            }
        };

        ctx.current_module().append(code).export(type_ident);
    }
}

/* EnumerationData */

impl EnumerationData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>, options: SerdeOptions) {
        let Self {
            type_ident,
            variants,
            trait_impls,
            ..
        } = self;

        let values_ident = format_enum_values_ident(&*ctx.meta.types.naming, ctx.ident);

        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let trait_impls = render_trait_impls(type_ident, trait_impls);
        let extra_attributes = &ctx.data.extra_attributes;

        let variants = variants
            .iter()
            .map(|d| d.render_variant_serde_xml_rs_v8(ctx, options))
            .collect::<Vec<_>>();

        let from = ctx.resolve_build_in("::core::convert::From");
        let deref = ctx.resolve_build_in("::core::ops::Deref");
        let deref_mut = ctx.resolve_build_in("::core::ops::DerefMut");

        ctx.add_usings(["::core::ops::Deref", "::core::ops::DerefMut"]);

        let code = quote! {
            #docs
            #derive
            pub struct #type_ident {
                #[serde(rename = "#text")]
                pub value: #values_ident,
            }

            impl #from<#values_ident> for #type_ident {
                fn from(value: #values_ident) -> Self {
                    Self { value }
                }
            }

            impl #from<#type_ident> for #values_ident {
                fn from(value: #type_ident) -> Self {
                    value.value
                }
            }

            impl #deref for #type_ident {
                type Target = #values_ident;

                fn deref(&self) -> &Self::Target {
                    &self.value
                }
            }

            impl #deref_mut for #type_ident {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.value
                }
            }

            #derive
            #( #[#extra_attributes] )*
            pub enum #values_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        };

        ctx.current_module()
            .append(code)
            .export(type_ident)
            .export(&values_ident);

        self.render_common_impls(ctx);
    }
}

impl EnumerationDataVariant<'_> {
    fn render_variant_serde_xml_rs_v8(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> TokenStream {
        let serde_attribute = SerdeAttribute::new(&self.extra_attributes).with_field(
            ctx,
            options,
            self.serde_field(),
        );
        let Self {
            meta,
            s_name,
            variant_ident,
            target_type,
            extra_attributes,
            ..
        } = self;

        let docs = ctx.render_docs(RendererFlags::RENDER_VARIANT_DOCS, &meta.documentation[..]);

        let target_type = target_type.as_ref().map(|target_type| {
            let target_type = ctx.resolve_type_for_module(target_type);

            quote!((#serde_attribute #target_type))
        });

        quote! {
            #docs
            #( #[#extra_attributes] )*
            #[serde(rename = #s_name)]
            #variant_ident #target_type,
        }
    }
}

/* SimpleData */

impl SimpleData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>, options: SerdeOptions) {
        let serde_attribute = SerdeAttribute::new(&[]).with_field(ctx, options, self.serde_field());
        let Self {
            occurs,
            type_ident,
            target_type,
            trait_impls,
            ..
        } = self;

        let docs = ctx.render_type_docs();
        let target_type = ctx.resolve_type_for_module(target_type);
        let target_type = occurs.make_type(ctx, &target_type, false);
        let extra_attributes = &ctx.data.extra_attributes;

        let derive = get_derive(ctx, []);
        let trait_impls = render_trait_impls(type_ident, trait_impls);

        let code = quote! {
            #docs
            #derive
            #( #[#extra_attributes] )*
            pub struct #type_ident(#serde_attribute pub #target_type);

            #( #trait_impls )*
        };

        ctx.current_module().append(code).export(type_ident);

        self.render_common_impls(ctx);
    }
}

/* ComplexData */

impl ComplexData<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>, options: SerdeOptions) {
        match self {
            Self::Enum {
                type_,
                content_type,
            } => {
                type_.render_type_serde_xml_rs_v8(ctx, options);

                if let Some(content_type) = content_type {
                    content_type.render_type_serde_xml_rs_v8(ctx, options);
                }
            }
            Self::Struct {
                type_,
                content_type,
            } => {
                type_.render_type_serde_xml_rs_v8(ctx, options);

                if let Some(content_type) = content_type {
                    content_type.render_type_serde_xml_rs_v8(ctx, options);
                }
            }
        }
    }
}

impl ComplexDataEnum<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>, options: SerdeOptions) {
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(type_ident, &self.trait_impls);
        let extra_attributes = &ctx.data.extra_attributes;

        let variants = self
            .elements
            .iter()
            .map(|x| x.render_variant_serde_xml_rs_v8(ctx, options));

        let code = quote! {
            #docs
            #derive
            #( #[#extra_attributes] )*
            pub enum #type_ident {
                #( #variants )*
            }

            #( #trait_impls )*
        };

        ctx.current_module().append(code).export(type_ident);
    }
}

impl ComplexDataStruct<'_> {
    fn render_type_serde_xml_rs_v8(&self, ctx: &mut Context<'_, '_>, options: SerdeOptions) {
        let docs = ctx.render_type_docs();
        let derive = get_derive(ctx, []);
        let type_ident = &self.type_ident;
        let trait_impls = render_trait_impls(type_ident, &self.trait_impls);
        let extra_attributes = &ctx.data.extra_attributes;

        let attributes = self
            .attributes
            .iter()
            .map(|x| x.render_field_serde_xml_rs_v8(ctx, type_ident, options));
        let fields = self
            .elements()
            .iter()
            .map(|x| x.render_field_serde_xml_rs_v8(ctx, options));
        let content = self
            .content()
            .as_ref()
            .and_then(|x| x.render_field_serde_xml_rs_v8(ctx, options));

        let struct_data = if self.is_unit_struct() {
            quote!(;)
        } else {
            quote! {
                {
                    #( #attributes )*
                    #( #fields )*
                    #content
                }
            }
        };

        let code = quote! {
            #docs
            #derive
            #( #[#extra_attributes] )*
            pub struct #type_ident
                #struct_data

            #( #trait_impls )*
        };

        ctx.current_module().append(code).export(type_ident);
    }
}

impl ComplexDataContent<'_> {
    fn render_field_serde_xml_rs_v8(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> Option<TokenStream> {
        let field_ident = &self.field_ident;
        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self
            .occurs
            .array_to_vec()
            .make_type(ctx, &target_type, false)?;
        let extra_attributes = &self.extra_attributes;

        let mut field = self.serde_field(options);
        let (name, target_type) = match ctx
            .get_resolved_complex_content()
            .map(|(ident, meta)| (ident, &meta.variant))
        {
            Some((ident, MetaTypeVariant::Enumeration(_))) => {
                let enum_values_type = format_enum_values_ident(&*ctx.meta.types.naming, ident);
                // The rendered enum values field is scalar regardless of content occurrence.
                field.occurs = Occurs::Single;

                ("#text", quote!(#enum_values_type))
            }
            Some((_, MetaTypeVariant::BuildIn(_) | MetaTypeVariant::Reference(_))) => {
                ("#text", target_type)
            }
            None | Some((_, _)) => ("#content", target_type),
        };
        let serde_attribute = SerdeAttribute::new(&self.extra_attributes)
            .with_default(self.is_empty_string_content(ctx) || self.min_occurs == 0)
            .with_rename(name)
            .with_field(ctx, options, field);

        Some(quote! {
            #( #[#extra_attributes] )*
            #serde_attribute
            pub #field_ident: #target_type,
        })
    }
}

impl ComplexDataAttribute<'_> {
    fn render_field_serde_xml_rs_v8(
        &self,
        ctx: &Context<'_, '_>,
        type_ident: &Ident2,
        options: SerdeOptions,
    ) -> TokenStream {
        let Self {
            tag_name,
            ident: field_ident,
            extra_attributes,
            ..
        } = self;

        let tag_name = tag_name.get(true);

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = if self.is_option {
            let option = ctx.resolve_build_in("::core::option::Option");

            quote!(#option<#target_type>)
        } else {
            target_type
        };

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ATTRIBUTE_DOCS,
            &self.meta.documentation[..],
        );

        let name = format!("@{tag_name}");
        let serde_attribute = SerdeAttribute::new(&self.extra_attributes)
            .with_default_function(self.serde_default_path(type_ident))
            .with_default(self.meta.use_ == Use::Optional)
            .with_rename(name)
            .with_field(ctx, options, self.serde_field());

        quote! {
            #docs
            #( #[#extra_attributes] )*
            #serde_attribute
            pub #field_ident: #target_type,
        }
    }
}

impl ComplexDataElement<'_> {
    fn render_field_serde_xml_rs_v8(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> TokenStream {
        let Self {
            tag_name,
            field_ident,
            extra_attributes,
            ..
        } = self;

        let name = if self.meta().is_text() {
            "#text".to_owned()
        } else {
            tag_name.get(true)
        };

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type = self
            .occurs
            .array_to_vec()
            .make_type(ctx, &target_type, self.need_indirection)
            .unwrap();

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ELEMENT_DOCS,
            &self.meta().documentation[..],
        );

        let serde_attribute = SerdeAttribute::new(&self.extra_attributes)
            .with_default(matches!(
                self.occurs.array_to_vec(),
                Occurs::Optional | Occurs::DynamicList
            ))
            .with_rename(name)
            .with_field(ctx, options, self.serde_field(options));

        quote! {
            #docs
            #( #[#extra_attributes] )*
            #serde_attribute
            pub #field_ident: #target_type,
        }
    }

    fn render_variant_serde_xml_rs_v8(
        &self,
        ctx: &Context<'_, '_>,
        options: SerdeOptions,
    ) -> TokenStream {
        let serde_attribute = SerdeAttribute::new(&self.extra_attributes).with_field(
            ctx,
            options,
            self.serde_field(options).as_payload(),
        );
        let Self {
            tag_name,
            variant_ident,
            extra_attributes,
            ..
        } = self;

        let name = if self.meta().is_text() {
            "#text".to_owned()
        } else {
            tag_name.get(true)
        };

        let target_type = ctx.resolve_type_for_module(&self.target_type);
        let target_type =
            self.occurs
                .array_to_vec()
                .make_type(ctx, &target_type, self.need_indirection);

        let docs = ctx.render_docs(
            RendererFlags::RENDER_ELEMENT_DOCS,
            &self.meta().documentation[..],
        );

        quote! {
            #docs
            #( #[#extra_attributes] )*
            #[serde(rename = #name)]
            #variant_ident(#serde_attribute #target_type),
        }
    }
}

fn format_enum_values_ident(naming: &dyn Naming, ident: &TypeIdent) -> Ident2 {
    let values_ident = naming.format_type_name(ident.name.as_str());

    format_ident!("{values_ident}Value")
}
