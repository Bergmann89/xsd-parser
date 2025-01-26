mod deserialize;
mod serialize;

use std::collections::HashSet;
use std::ops::Deref;

use proc_macro2::{Ident as Ident2, Literal};
use quote::format_ident;

use crate::types::{ComplexInfo, Ident, Type, Types};
use crate::Generator;

use super::super::data::{AttributeData, ComplexTypeData, ElementData, TypeInfoData};
use super::super::misc::{GenerateFlags, TypeRef};

pub(crate) struct QuickXmlRenderer;

/* ComplexTypeImpl */

#[derive(Debug)]
#[allow(dead_code)]
struct ComplexTypeImpl<'a, 'b, 'types> {
    inner: &'a ComplexTypeData<'b, 'types>,

    type_ref: &'a TypeRef,
    type_ident: &'a Ident2,
    content_ident: Ident2,

    serializer_ident: Ident2,
    serializer_state_ident: Ident2,

    deserializer_ident: Ident2,
    deserializer_state_ident: Ident2,

    flatten_content: bool,
    is_static_complex: bool,

    attributes: Vec<AttributeImpl<'a, 'types>>,
    elements: Vec<ElementImpl<'a, 'types>>,
}

#[derive(Debug)]
struct AttributeImpl<'a, 'types> {
    inner: &'a AttributeData<'types>,

    s_name: String,
    b_name: Literal,
    tag_name: String,
}

#[derive(Debug)]
struct ElementImpl<'a, 'types> {
    inner: &'a ElementData<'types>,

    s_name: String,
    b_name: Literal,
    tag_name: String,
}

impl<'a, 'b, 'types> ComplexTypeImpl<'a, 'b, 'types> {
    fn new(inner: &'a ComplexTypeData<'b, 'types>) -> Self {
        let type_ref = inner.current_type_ref();

        let has_attributes = !inner.attributes.is_empty();
        let is_static_complex = matches!(&inner.ty, TypeInfoData::Complex(ci) if !ci.is_dynamic);
        let flatten_content =
            !has_attributes && inner.check_generate_flags(GenerateFlags::FLATTEN_CONTENT);

        let type_ident = &type_ref.type_ident;
        let content_ident = if flatten_content {
            type_ident.clone()
        } else {
            format_ident!("{type_ident}Content")
        };

        let serializer_ident = format_ident!("{type_ident}Serializer");
        let serializer_state_ident = format_ident!("{type_ident}SerializerState");

        let deserializer_ident = format_ident!("{type_ident}Deserializer");
        let deserializer_state_ident = format_ident!("{type_ident}DeserializerState");

        let attributes = inner
            .attributes
            .iter()
            .map(|attribute| AttributeImpl::new(inner.types, attribute))
            .collect();
        let elements = inner
            .elements
            .iter()
            .map(|element| ElementImpl::new(inner.types, element))
            .collect();

        Self {
            inner,

            type_ref,
            type_ident,
            content_ident,

            serializer_ident,
            serializer_state_ident,

            deserializer_ident,
            deserializer_state_ident,

            flatten_content,
            is_static_complex,

            attributes,
            elements,
        }
    }
}

impl<'b, 'types> Deref for ComplexTypeImpl<'_, 'b, 'types> {
    type Target = ComplexTypeData<'b, 'types>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

/* AttributeImpl */

impl<'a, 'types> AttributeImpl<'a, 'types> {
    fn new(types: &'types Types, inner: &'a AttributeData<'types>) -> Self {
        let s_name = inner.ident.name.to_string();
        let b_name = Literal::byte_string(s_name.as_bytes());

        let tag_name = if let Some(m_name) = inner
            .ident
            .ns
            .as_ref()
            .and_then(|ns| types.modules.get(ns))
            .and_then(|module| module.name.as_ref())
        {
            format!("{m_name}:{s_name}")
        } else {
            s_name.clone()
        };

        Self {
            inner,
            s_name,
            b_name,
            tag_name,
        }
    }
}

impl<'types> Deref for AttributeImpl<'_, 'types> {
    type Target = AttributeData<'types>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

/* FieldImpl */

impl<'a, 'types> ElementImpl<'a, 'types> {
    fn new(types: &'types Types, inner: &'a ElementData<'types>) -> Self {
        let s_name = inner.ident.name.to_string();
        let b_name = Literal::byte_string(s_name.as_bytes());

        let tag_name = if let Some(m_name) = inner
            .ident
            .ns
            .as_ref()
            .and_then(|ns| types.modules.get(ns))
            .and_then(|module| module.name.as_ref())
        {
            format!("{m_name}:{s_name}")
        } else {
            s_name.clone()
        };

        Self {
            inner,
            s_name,
            b_name,
            tag_name,
        }
    }

    fn new_target_type_allows_any(&self, generator: &Generator<'_>) -> bool {
        fn walk(generator: &Generator<'_>, visit: &mut HashSet<Ident>, ident: &Ident) -> bool {
            if !visit.insert(ident.clone()) {
                return false;
            }

            match generator.types.get(ident) {
                None => false,
                Some(Type::All(si) | Type::Choice(si)) => {
                    if si.any.is_some() {
                        return true;
                    }

                    si.elements.iter().any(|f| walk(generator, visit, &f.type_))
                }
                Some(Type::Sequence(si)) => {
                    if si.any.is_some() {
                        return true;
                    }

                    if let Some(first) = si.elements.first() {
                        return walk(generator, visit, &first.type_);
                    }

                    false
                }
                Some(Type::ComplexType(ComplexInfo {
                    content: Some(content),
                    ..
                })) => walk(generator, visit, content),
                _ => false,
            }
        }

        let mut visit = HashSet::new();

        walk(generator, &mut visit, &self.type_)
    }
}

impl<'types> Deref for ElementImpl<'_, 'types> {
    type Target = ElementData<'types>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
