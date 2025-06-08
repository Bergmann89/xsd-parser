use std::mem::take;

use proc_macro2::{Ident as Ident2, Literal};
use quote::format_ident;

use crate::config::{BoxFlags, GeneratorFlags};
use crate::models::meta::MetaTypes;
use crate::models::{
    code::{format_field_ident, format_variant_ident, IdentPath},
    data::{
        ComplexBase, ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement,
        ComplexDataEnum, ComplexDataStruct, Occurs, StructMode,
    },
    meta::{
        AttributeMeta, AttributeMetaVariant, ComplexMeta, ElementMeta, ElementMetaVariant,
        GroupMeta, MetaTypeVariant,
    },
    schema::{xs::Use, MaxOccurs, MinOccurs},
    Ident,
};

use super::super::{Context, Error};

#[derive(Debug, Clone)]
enum TypeMode {
    All,
    Choice,
    Sequence,
    Simple { target_type: IdentPath },
}

impl<'types> ComplexData<'types> {
    pub(super) fn new_all(
        meta: &'types GroupMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        Self::new(
            ctx,
            TypeMode::All,
            1,
            MaxOccurs::Bounded(1),
            &[],
            &meta.elements,
        )
    }

    pub(super) fn new_choice(
        meta: &'types GroupMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        Self::new(
            ctx,
            TypeMode::Choice,
            1,
            MaxOccurs::Bounded(1),
            &[],
            &meta.elements,
        )
    }

    pub(super) fn new_sequence(
        meta: &'types GroupMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        Self::new(
            ctx,
            TypeMode::Sequence,
            1,
            MaxOccurs::Bounded(1),
            &[],
            &meta.elements,
        )
    }

    pub(super) fn new_complex(
        meta: &'types ComplexMeta,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        let (type_mode, elements) = match meta.content.as_ref().and_then(|ident| {
            ctx.types
                .get_resolved_type(ident)
                .map(|ty| (&ty.variant, ident))
        }) {
            None => (TypeMode::Sequence, &[][..]),
            Some((MetaTypeVariant::All(si), _)) => (TypeMode::All, &si.elements[..]),
            Some((MetaTypeVariant::Choice(si), _)) => (TypeMode::Choice, &si.elements[..]),
            Some((MetaTypeVariant::Sequence(si), _)) => (TypeMode::Sequence, &si.elements[..]),
            Some((
                MetaTypeVariant::BuildIn(_)
                | MetaTypeVariant::Union(_)
                | MetaTypeVariant::Enumeration(_)
                | MetaTypeVariant::Reference(_),
                ident,
            )) => {
                let content_ref = ctx.get_or_create_type_ref(ident.clone())?;
                let target_type = content_ref.to_ident_path();

                (TypeMode::Simple { target_type }, &[][..])
            }
            Some((x, _)) => {
                let ident = &ctx.current_type_ref().type_ident;

                tracing::warn!("Complex type has unexpected content: ident={ident}, meta={meta:#?}, content={x:#?}!");

                (TypeMode::Sequence, &[][..])
            }
        };

        Self::new(
            ctx,
            type_mode,
            meta.min_occurs,
            meta.max_occurs,
            &meta.attributes,
            elements,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        ctx: &mut Context<'_, 'types>,
        type_mode: TypeMode,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeMeta],
        elements: &'types [ElementMeta],
    ) -> Result<Self, Error> {
        match type_mode {
            TypeMode::Simple { target_type } => {
                Self::new_simple(ctx, target_type, min_occurs, max_occurs, attributes)
            }
            TypeMode::Choice => Self::new_enum(ctx, min_occurs, max_occurs, attributes, elements),
            TypeMode::All | TypeMode::Sequence => Self::new_struct(
                ctx, &type_mode, min_occurs, max_occurs, attributes, elements,
            ),
        }
    }

    fn new_simple(
        ctx: &mut Context<'_, 'types>,
        target_type: IdentPath,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeMeta],
    ) -> Result<Self, Error> {
        let base = ComplexBase::new(ctx, false)?;
        let occurs = Occurs::from_occurs(min_occurs, max_occurs);

        let mut allow_any_attribute = false;
        let attributes = attributes
            .iter()
            .filter_map(|meta| {
                ComplexDataAttribute::new_field(meta, ctx, &mut allow_any_attribute).transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let content = ComplexDataContent {
            occurs,
            is_simple: true,
            min_occurs,
            max_occurs,
            target_type,
        };

        let type_ = ComplexDataStruct {
            base,
            mode: StructMode::Content { content },
            attributes,
            allow_any_attribute,
        };

        Ok(Self::Struct {
            type_,
            content_type: None,
        })
    }

    fn new_enum(
        ctx: &mut Context<'_, 'types>,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeMeta],
        elements: &'types [ElementMeta],
    ) -> Result<Self, Error> {
        let has_any = ctx.any_type.is_some() && elements.iter().any(ElementMeta::is_any);
        let mut base = ComplexBase::new(ctx, has_any)?;
        let occurs = Occurs::from_occurs(min_occurs, max_occurs);

        let mut allow_any_attribute = false;
        let attributes = attributes
            .iter()
            .filter_map(|meta| {
                ComplexDataAttribute::new_field(meta, ctx, &mut allow_any_attribute).transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut allow_any = false;
        let mut elements = elements
            .iter()
            .filter_map(|meta| {
                ComplexDataElement::new_variant(meta, ctx, &mut allow_any, occurs.is_direct())
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let flatten = occurs == Occurs::Single
            && attributes.is_empty()
            && ctx.check_generator_flags(GeneratorFlags::FLATTEN_ENUM_CONTENT);

        if flatten {
            let type_ = ComplexDataEnum {
                base,
                elements,
                allow_any,
                allow_any_attribute,
            };

            return Ok(ComplexData::Enum {
                type_,
                content_type: None,
            });
        }

        let type_ident = &base.type_ident;
        let content_ident = format_ident!("{type_ident}Content");
        let has_content = occurs.is_some() && !elements.is_empty();

        let content_type = has_content.then(|| {
            let type_ = ComplexDataEnum {
                base: ComplexBase::new_empty(content_ident.clone(), take(&mut base.has_any)),
                elements: take(&mut elements),
                allow_any,
                allow_any_attribute,
            };

            Box::new(ComplexData::Enum {
                type_,
                content_type: None,
            })
        });

        let mode = if has_content {
            let type_ref = ctx.current_type_ref();
            let target_type = type_ref.to_ident_path().with_ident(content_ident.clone());
            let content = ComplexDataContent {
                occurs,
                is_simple: false,
                min_occurs,
                max_occurs,
                target_type,
            };

            StructMode::Content { content }
        } else {
            StructMode::Empty { allow_any }
        };

        let type_ = ComplexDataStruct {
            base,
            mode,

            attributes,
            allow_any_attribute,
        };

        Ok(ComplexData::Struct {
            type_,
            content_type,
        })
    }

    #[allow(clippy::too_many_lines, clippy::too_many_arguments)]
    fn new_struct(
        ctx: &mut Context<'_, 'types>,
        type_mode: &TypeMode,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeMeta],
        elements: &'types [ElementMeta],
    ) -> Result<Self, Error> {
        let has_any = ctx.any_type.is_some() && elements.iter().any(ElementMeta::is_any);
        let mut base = ComplexBase::new(ctx, has_any)?;
        let occurs = Occurs::from_occurs(min_occurs, max_occurs);
        let flatten = occurs == Occurs::Single
            && ctx.check_generator_flags(GeneratorFlags::FLATTEN_STRUCT_CONTENT);

        let mut allow_any_attribute = false;
        let attributes = attributes
            .iter()
            .filter_map(|meta| {
                ComplexDataAttribute::new_field(meta, ctx, &mut allow_any_attribute).transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut allow_any = false;
        let elements = elements
            .iter()
            .filter_map(|meta| {
                ComplexDataElement::new_field(meta, ctx, &mut allow_any, occurs.is_direct())
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if flatten {
            let mode = match type_mode {
                _ if elements.is_empty() => StructMode::Empty { allow_any },
                TypeMode::All => StructMode::All {
                    elements,
                    allow_any,
                },
                TypeMode::Sequence => StructMode::Sequence {
                    elements,
                    allow_any,
                },
                _ => crate::unreachable!(),
            };

            let type_ = ComplexDataStruct {
                base,
                mode,

                attributes,
                allow_any_attribute,
            };

            return Ok(ComplexData::Struct {
                type_,
                content_type: None,
            });
        }

        let type_ident = &base.type_ident;
        let content_ident = format_ident!("{type_ident}Content");
        let has_content = occurs.is_some() && !elements.is_empty();

        let content_type = has_content.then(|| {
            let mode = match type_mode {
                TypeMode::All => StructMode::All {
                    elements,
                    allow_any,
                },
                TypeMode::Sequence => StructMode::Sequence {
                    elements,
                    allow_any,
                },
                _ => crate::unreachable!(),
            };

            let type_ = ComplexDataStruct {
                base: ComplexBase::new_empty(content_ident.clone(), take(&mut base.has_any)),
                mode,

                attributes: Vec::new(),
                allow_any_attribute: false,
            };

            Box::new(ComplexData::Struct {
                type_,
                content_type: None,
            })
        });

        let mode = if has_content {
            let type_ref = ctx.current_type_ref();
            let target_type = type_ref.to_ident_path().with_ident(content_ident.clone());
            let content = ComplexDataContent {
                occurs,
                is_simple: false,
                min_occurs,
                max_occurs,
                target_type,
            };

            StructMode::Content { content }
        } else {
            StructMode::Empty { allow_any }
        };

        let type_ = ComplexDataStruct {
            base,
            mode,

            attributes,
            allow_any_attribute,
        };

        Ok(ComplexData::Struct {
            type_,
            content_type,
        })
    }
}

impl ComplexBase {
    fn new(ctx: &mut Context<'_, '_>, has_any: bool) -> Result<Self, Error> {
        let type_ref = ctx.current_type_ref();
        let type_ident = type_ref.type_ident.clone();

        let mut ret = Self::new_empty(type_ident, has_any);
        ret.tag_name = Some(make_tag_name(ctx.types, ctx.ident));
        ret.trait_impls = ctx.make_trait_impls()?;

        if let Some(MetaTypeVariant::ComplexType(ci)) = ctx.types.get_variant(ctx.ident) {
            ret.is_complex = true;
            ret.is_dynamic = ci.is_dynamic;
        }

        Ok(ret)
    }

    fn new_empty(type_ident: Ident2, has_any: bool) -> Self {
        let serializer_ident = format_ident!("{type_ident}Serializer");
        let serializer_state_ident = format_ident!("{type_ident}SerializerState");

        let deserializer_ident = format_ident!("{type_ident}Deserializer");
        let deserializer_state_ident = format_ident!("{type_ident}DeserializerState");

        Self {
            type_ident,
            trait_impls: Vec::new(),

            tag_name: None,
            has_any,
            is_complex: false,
            is_dynamic: false,

            serializer_ident,
            serializer_state_ident,

            deserializer_ident,
            deserializer_state_ident,
        }
    }
}

impl<'types> ComplexDataElement<'types> {
    fn new_variant(
        meta: &'types ElementMeta,
        ctx: &mut Context<'_, 'types>,
        allow_any: &mut bool,
        direct_usage: bool,
    ) -> Result<Option<Self>, Error> {
        let force_box = ctx.box_flags.intersects(BoxFlags::ENUM_ELEMENTS);

        Self::new(meta, ctx, allow_any, direct_usage, force_box)
    }

    fn new_field(
        info: &'types ElementMeta,
        ctx: &mut Context<'_, 'types>,
        allow_any: &mut bool,
        direct_usage: bool,
    ) -> Result<Option<Self>, Error> {
        let force_box = ctx.box_flags.intersects(BoxFlags::STRUCT_ELEMENTS);

        Self::new(info, ctx, allow_any, direct_usage, force_box)
    }

    fn new(
        info: &'types ElementMeta,
        ctx: &mut Context<'_, 'types>,
        allow_any: &mut bool,
        direct_usage: bool,
        force_box: bool,
    ) -> Result<Option<Self>, Error> {
        let occurs = Occurs::from_occurs(info.min_occurs, info.max_occurs);
        if occurs == Occurs::None {
            return Ok(None);
        }

        let tag_name = make_tag_name(ctx.types, &info.ident);
        let s_name = info.ident.name.to_string();
        let b_name = Literal::byte_string(s_name.as_bytes());
        let field_ident = format_field_ident(&info.ident.name, info.display_name.as_deref());
        let variant_ident = format_variant_ident(&info.ident.name, info.display_name.as_deref());

        let (target_type, target_is_dynamic) = match &info.variant {
            ElementMetaVariant::Any(_) => {
                let Some(type_) = ctx.any_type.as_ref() else {
                    *allow_any = true;

                    return Ok(None);
                };

                let target_type = IdentPath::from_ident(type_.ident().clone()).with_path([]);
                let target_is_dynamic = false;

                (target_type, target_is_dynamic)
            }
            ElementMetaVariant::Type(type_) => {
                let target_ref = ctx.get_or_create_type_ref(type_.clone())?;
                let target_type = target_ref.to_ident_path();
                let target_is_dynamic = is_dynamic(type_, ctx.types);

                (target_type, target_is_dynamic)
            }
        };

        let need_box = ctx.current_type_ref().boxed_elements.contains(&info.ident);
        let need_indirection = (direct_usage && need_box) || force_box;

        Ok(Some(Self {
            info,
            occurs,
            s_name,
            b_name,
            tag_name,
            field_ident,
            variant_ident,
            target_type,
            need_indirection,
            target_is_dynamic,
        }))
    }
}

impl<'types> ComplexDataAttribute<'types> {
    fn new_field(
        info: &'types AttributeMeta,
        ctx: &mut Context<'_, 'types>,
        allow_any_attribute: &mut bool,
    ) -> Result<Option<Self>, Error> {
        if info.use_ == Use::Prohibited {
            return Ok(None);
        }

        let current_module = ctx.current_module();
        let ident = format_field_ident(&info.ident.name, info.display_name.as_deref());

        let (target_type, default_value) = match &info.variant {
            AttributeMetaVariant::Any(_) => {
                let Some(type_) = ctx.any_attribute_type.as_ref() else {
                    *allow_any_attribute = true;

                    return Ok(None);
                };

                let target_type = IdentPath::from_ident(type_.ident().clone()).with_path([]);

                (target_type, None)
            }
            AttributeMetaVariant::Type(type_) => {
                let target_ref = ctx.get_or_create_type_ref(type_.clone())?;
                let target_type = target_ref.to_ident_path();

                let default_value = info
                    .default
                    .as_ref()
                    .map(|default| ctx.get_default(current_module, default, type_))
                    .transpose()?;

                (target_type, default_value)
            }
        };

        let s_name = info.ident.name.to_string();
        let b_name = Literal::byte_string(s_name.as_bytes());
        let tag_name = make_tag_name(ctx.types, &info.ident);
        let is_option = matches!((&info.use_, &default_value), (Use::Optional, None));

        Ok(Some(Self {
            info,
            ident,
            s_name,
            b_name,
            tag_name,
            is_option,
            target_type,
            default_value,
        }))
    }
}

fn is_dynamic(ident: &Ident, types: &MetaTypes) -> bool {
    let Some(ty) = types.items.get(ident) else {
        return false;
    };

    match &ty.variant {
        MetaTypeVariant::Dynamic(_) => true,
        MetaTypeVariant::ComplexType(ci) => ci.is_dynamic,
        MetaTypeVariant::Reference(x) if x.is_single() => is_dynamic(&x.type_, types),
        _ => false,
    }
}

fn make_tag_name(types: &MetaTypes, ident: &Ident) -> String {
    let name = ident.name.to_string();

    if let Some(module) = ident
        .ns
        .as_ref()
        .and_then(|ns| types.modules.get(ns))
        .and_then(|module| module.name.as_ref())
    {
        format!("{module}:{name}")
    } else {
        name
    }
}
