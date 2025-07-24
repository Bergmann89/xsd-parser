use std::mem::{swap, take};

use proc_macro2::{Ident as Ident2, Literal};
use quote::format_ident;

use crate::config::{BoxFlags, GeneratorFlags};
use crate::models::{
    code::{format_field_ident, format_variant_ident, IdentPath},
    data::{
        ComplexBase, ComplexData, ComplexDataAttribute, ComplexDataContent, ComplexDataElement,
        ComplexDataElementOrigin, ComplexDataEnum, ComplexDataStruct, Occurs, PathData, StructMode,
    },
    meta::{
        AttributeMeta, AttributeMetaVariant, ComplexMeta, ElementMeta, ElementMetaVariant,
        ElementMode, GroupMeta, MetaTypeVariant, MetaTypes,
    },
    schema::{
        xs::{FormChoiceType, Use},
        MaxOccurs, MinOccurs,
    },
    Ident,
};

use super::super::{Context, Error};

#[derive(Debug, Clone)]
enum TypeMode<'types> {
    All,
    Choice,
    Sequence,
    Simple { simple_type: &'types Ident },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MixedMode {
    None,
    Group,
    Complex,
}

impl<'types> ComplexData<'types> {
    pub(super) fn new_all(
        meta: &'types GroupMeta,
        ctx: &mut Context<'_, 'types>,
        form: FormChoiceType,
    ) -> Result<Self, Error> {
        Self::new(
            ctx,
            form,
            &TypeMode::All,
            ctx.mixed_mode(meta.is_mixed, MixedMode::Group),
            1,
            MaxOccurs::Bounded(1),
            &[],
            &meta.elements,
        )
    }

    pub(super) fn new_choice(
        meta: &'types GroupMeta,
        ctx: &mut Context<'_, 'types>,
        form: FormChoiceType,
    ) -> Result<Self, Error> {
        Self::new(
            ctx,
            form,
            &TypeMode::Choice,
            ctx.mixed_mode(meta.is_mixed, MixedMode::Group),
            1,
            MaxOccurs::Bounded(1),
            &[],
            &meta.elements,
        )
    }

    pub(super) fn new_sequence(
        meta: &'types GroupMeta,
        ctx: &mut Context<'_, 'types>,
        form: FormChoiceType,
    ) -> Result<Self, Error> {
        Self::new(
            ctx,
            form,
            &TypeMode::Sequence,
            ctx.mixed_mode(meta.is_mixed, MixedMode::Group),
            1,
            MaxOccurs::Bounded(1),
            &[],
            &meta.elements,
        )
    }

    pub(super) fn new_complex(
        meta: &'types ComplexMeta,
        ctx: &mut Context<'_, 'types>,
        form: FormChoiceType,
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
            )) => (TypeMode::Simple { simple_type: ident }, &[][..]),
            Some((x, _)) => {
                let ident = ctx.current_type_ref().path.ident();

                tracing::warn!("Complex type has unexpected content: ident={ident}, meta={meta:#?}, content={x:#?}!");

                (TypeMode::Sequence, &[][..])
            }
        };

        Self::new(
            ctx,
            form,
            &type_mode,
            ctx.mixed_mode(meta.is_mixed, MixedMode::Complex),
            meta.min_occurs,
            meta.max_occurs,
            &meta.attributes,
            elements,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        ctx: &mut Context<'_, 'types>,
        form: FormChoiceType,
        type_mode: &TypeMode<'types>,
        mixed_mode: MixedMode,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeMeta],
        elements: &'types [ElementMeta],
    ) -> Result<Self, Error> {
        match type_mode {
            TypeMode::Simple { simple_type } => {
                Self::new_simple(ctx, form, simple_type, min_occurs, max_occurs, attributes)
            }
            TypeMode::Choice => Self::new_enum(
                ctx, form, mixed_mode, min_occurs, max_occurs, attributes, elements,
            ),
            TypeMode::All | TypeMode::Sequence => Self::new_struct(
                ctx, form, type_mode, mixed_mode, min_occurs, max_occurs, attributes, elements,
            ),
        }
    }

    fn new_simple(
        ctx: &mut Context<'_, 'types>,
        form: FormChoiceType,
        simple_type: &'types Ident,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeMeta],
    ) -> Result<Self, Error> {
        let base = ComplexBase::new(ctx, false, form)?;
        let occurs = Occurs::from_occurs(min_occurs, max_occurs);

        let content_ref = ctx.get_or_create_type_ref(simple_type)?;
        let target_type = content_ref.path.clone();

        let mut allow_any_attribute = false;
        let attributes = attributes
            .iter()
            .filter_map(|meta| {
                ComplexDataAttribute::new_field(meta, ctx, &mut allow_any_attribute).transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let content = ComplexDataContent {
            occurs,
            simple_type: Some(simple_type),
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
        form: FormChoiceType,
        mixed_mode: MixedMode,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeMeta],
        elements: &'types [ElementMeta],
    ) -> Result<Self, Error> {
        let has_any = ctx.any_type.is_some() && elements.iter().any(ElementMeta::is_any);
        let mut base = ComplexBase::new(ctx, has_any, form)?;
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
                ComplexDataElement::new_variant(
                    ComplexDataElementOrigin::Meta(meta),
                    ctx,
                    &mut allow_any,
                    occurs.is_direct(),
                    mixed_mode != MixedMode::None,
                )
                .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let flatten = occurs == Occurs::Single
            && mixed_mode != MixedMode::Complex
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

        let mode = if !has_content {
            StructMode::Empty { allow_any }
        } else if mixed_mode == MixedMode::Complex {
            let elements = vec![
                ComplexDataElement::new_text_before(),
                ComplexDataElement::new_content(ctx, min_occurs, max_occurs, content_ident),
            ];

            StructMode::Sequence {
                elements,
                allow_any: false,
            }
        } else {
            let type_ref = ctx.current_type_ref();

            let target_type = (*type_ref.path).clone().with_ident(content_ident.clone());
            let target_type = PathData::from_path(target_type);

            let content = ComplexDataContent {
                occurs,
                simple_type: None,
                min_occurs,
                max_occurs,
                target_type,
            };

            StructMode::Content { content }
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
        form: FormChoiceType,
        type_mode: &TypeMode<'types>,
        mixed_mode: MixedMode,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeMeta],
        elements: &'types [ElementMeta],
    ) -> Result<Self, Error> {
        let has_any = ctx.any_type.is_some() && elements.iter().any(ElementMeta::is_any);
        let mut base = ComplexBase::new(ctx, has_any, form)?;
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
        let text_before =
            (mixed_mode == MixedMode::Complex).then(ComplexDataElement::new_text_before);
        let normal_fields = elements.iter().filter_map(|meta| {
            ComplexDataElement::new_field(
                ComplexDataElementOrigin::Meta(meta),
                ctx,
                &mut allow_any,
                occurs.is_direct(),
                mixed_mode != MixedMode::None,
            )
            .transpose()
        });

        if flatten {
            let elements = text_before
                .into_iter()
                .map(Ok)
                .chain(normal_fields)
                .collect::<Result<Vec<_>, _>>()?;

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

        let elements = normal_fields.collect::<Result<Vec<_>, _>>()?;
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

        let mode = if !has_content {
            StructMode::Empty { allow_any }
        } else if let Some(text_before) = text_before {
            let elements = vec![
                text_before,
                ComplexDataElement::new_content(ctx, min_occurs, max_occurs, content_ident),
            ];

            StructMode::Sequence {
                elements,
                allow_any: false,
            }
        } else {
            let type_ref = ctx.current_type_ref();

            let target_type = (*type_ref.path).clone().with_ident(content_ident.clone());
            let target_type = PathData::from_path(target_type);

            let content = ComplexDataContent {
                occurs,
                simple_type: None,
                min_occurs,
                max_occurs,
                target_type,
            };

            StructMode::Content { content }
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
    fn new(ctx: &mut Context<'_, '_>, has_any: bool, form: FormChoiceType) -> Result<Self, Error> {
        let type_ref = ctx.current_type_ref();
        let type_ident = type_ref.path.ident().clone();

        let mut ret = Self::new_empty(type_ident, has_any);
        ret.tag_name = Some(make_tag_name(ctx.types, ctx.ident, form));
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
        origin: ComplexDataElementOrigin<'types>,
        ctx: &mut Context<'_, 'types>,
        allow_any: &mut bool,
        direct_usage: bool,
        mixed: bool,
    ) -> Result<Option<Self>, Error> {
        let force_box = ctx.box_flags.intersects(BoxFlags::ENUM_ELEMENTS);

        Self::new(origin, ctx, allow_any, direct_usage, force_box, mixed)
    }

    fn new_field(
        origin: ComplexDataElementOrigin<'types>,
        ctx: &mut Context<'_, 'types>,
        allow_any: &mut bool,
        direct_usage: bool,
        mixed: bool,
    ) -> Result<Option<Self>, Error> {
        let force_box = ctx.box_flags.intersects(BoxFlags::STRUCT_ELEMENTS);

        Self::new(origin, ctx, allow_any, direct_usage, force_box, mixed)
    }

    fn new(
        origin: ComplexDataElementOrigin<'types>,
        ctx: &mut Context<'_, 'types>,
        allow_any: &mut bool,
        direct_usage: bool,
        force_box: bool,
        mixed: bool,
    ) -> Result<Option<Self>, Error> {
        let meta = match &origin {
            ComplexDataElementOrigin::Meta(meta) => meta,
            ComplexDataElementOrigin::Generated(meta) => &**meta,
        };

        let occurs = Occurs::from_occurs(meta.min_occurs, meta.max_occurs);
        if occurs == Occurs::None {
            return Ok(None);
        }

        let tag_name = make_tag_name(ctx.types, &meta.ident, meta.form);
        let s_name = meta.ident.name.to_string();
        let b_name = Literal::byte_string(s_name.as_bytes());
        let field_ident = format_field_ident(&meta.ident.name, meta.display_name.as_deref());
        let variant_ident = format_variant_ident(&meta.ident.name, meta.display_name.as_deref());

        let (target_type, target_is_dynamic) = match &meta.variant {
            ElementMetaVariant::Any { .. } => {
                let Some(type_) = ctx.any_type.as_ref() else {
                    *allow_any = true;

                    return Ok(None);
                };

                let target_type = PathData::from_path(type_.clone()).into_included();
                let target_type = PathData::from_path_data_mixed(mixed, target_type)
                    .with_using(format!("{type_}"));

                let target_is_dynamic = false;

                (target_type, target_is_dynamic)
            }
            ElementMetaVariant::Type { type_, mode } => {
                let target_ref = ctx.get_or_create_type_ref(type_)?;

                let target_type = target_ref.path.clone();
                let target_type = PathData::from_path_data_mixed(
                    mixed && *mode == ElementMode::Element,
                    target_type,
                );

                let target_is_dynamic = is_dynamic(type_, ctx.types);

                (target_type, target_is_dynamic)
            }
            ElementMetaVariant::Text => {
                let target_type = PathData::text();

                let target_is_dynamic = false;

                (target_type, target_is_dynamic)
            }
        };

        let need_box = ctx.current_type_ref().boxed_elements.contains(&meta.ident);
        let need_indirection = (direct_usage && need_box) || force_box;

        Ok(Some(Self {
            origin,
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

    fn new_text_before() -> Self {
        let meta = ElementMeta {
            ident: Ident::element("text_before"),
            variant: ElementMetaVariant::Text,
            form: FormChoiceType::Unqualified,
            min_occurs: 0,
            max_occurs: MaxOccurs::Bounded(1),
            display_name: None,
            documentation: Vec::new(),
        };
        let origin = ComplexDataElementOrigin::Generated(Box::new(meta));

        Self {
            origin,
            occurs: Occurs::Optional,
            s_name: "text_before".into(),
            b_name: Literal::byte_string(b"text_before"),
            tag_name: String::new(),
            field_ident: format_ident!("text_before"),
            variant_ident: format_ident!("TextBefore"),
            target_type: PathData::text(),
            need_indirection: false,
            target_is_dynamic: false,
        }
    }

    fn new_content(
        ctx: &mut Context<'_, 'types>,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        content_ident: Ident2,
    ) -> Self {
        let meta = ElementMeta {
            ident: Ident::element("content"),
            variant: ElementMetaVariant::Type {
                type_: Ident::UNKNOWN,
                mode: ElementMode::Group,
            },
            form: FormChoiceType::Unqualified,
            min_occurs,
            max_occurs,
            display_name: None,
            documentation: Vec::new(),
        };
        let origin = ComplexDataElementOrigin::Generated(Box::new(meta));

        let occurs = Occurs::from_occurs(min_occurs, max_occurs);
        let type_ref = ctx.current_type_ref();

        let target_type = (*type_ref.path).clone().with_ident(content_ident);
        let target_type = PathData::from_path(target_type);

        Self {
            origin,
            occurs,
            s_name: "content".into(),
            b_name: Literal::byte_string(b"content"),
            tag_name: String::new(),
            field_ident: format_ident!("content"),
            variant_ident: format_ident!("Content"),
            target_type,
            need_indirection: false,
            target_is_dynamic: false,
        }
    }
}

impl<'types> ComplexDataAttribute<'types> {
    fn new_field(
        meta: &'types AttributeMeta,
        ctx: &mut Context<'_, 'types>,
        allow_any_attribute: &mut bool,
    ) -> Result<Option<Self>, Error> {
        if meta.use_ == Use::Prohibited {
            return Ok(None);
        }

        let current_module = ctx.current_module();
        let ident = format_field_ident(&meta.ident.name, meta.display_name.as_deref());

        let (target_type, default_value) = match &meta.variant {
            AttributeMetaVariant::Any(_) => {
                let Some(type_) = ctx.any_attribute_type.as_ref() else {
                    *allow_any_attribute = true;

                    return Ok(None);
                };

                let target_type = IdentPath::from_ident(type_.ident().clone());
                let target_type = PathData::from_path(target_type).with_using(format!("{type_}"));

                (target_type, None)
            }
            AttributeMetaVariant::Type(type_) => {
                let target_ref = ctx.get_or_create_type_ref(type_)?;
                let target_type = target_ref.path.clone();

                let default_value = meta
                    .default
                    .as_ref()
                    .map(|default| ctx.get_default(current_module, default, type_))
                    .transpose()?;

                (target_type, default_value)
            }
        };

        let s_name = meta.ident.name.to_string();
        let b_name = Literal::byte_string(s_name.as_bytes());
        let tag_name = make_tag_name(ctx.types, &meta.ident, meta.form);
        let is_option = matches!((&meta.use_, &default_value), (Use::Optional, None));

        Ok(Some(Self {
            meta,
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

fn make_tag_name(types: &MetaTypes, ident: &Ident, form: FormChoiceType) -> String {
    let name = ident.name.to_string();

    if form == FormChoiceType::Qualified {
        if let Some(module) = ident
            .ns
            .as_ref()
            .and_then(|ns| types.modules.get(ns))
            .and_then(|module| module.prefix.as_ref())
        {
            return format!("{module}:{name}");
        }
    }

    name
}

impl PathData {
    fn from_path_data_mixed(is_mixed: bool, mut path: PathData) -> PathData {
        if is_mixed {
            let mut tmp = IdentPath::from_ident(format_ident!("Mixed"));

            swap(&mut path.path, &mut tmp);

            path.with_generic(tmp).with_using("xsd_parser::xml::Mixed")
        } else {
            path
        }
    }

    fn text() -> Self {
        let target_type = format_ident!("Text");
        let target_type = IdentPath::from_ident(target_type);

        Self::from_path(target_type).with_using("xsd_parser::xml::Text")
    }
}

impl Context<'_, '_> {
    fn mixed_mode(&self, is_mixed: bool, mode: MixedMode) -> MixedMode {
        if is_mixed && self.check_generator_flags(GeneratorFlags::MIXED_TYPE_SUPPORT) {
            mode
        } else {
            MixedMode::None
        }
    }
}
