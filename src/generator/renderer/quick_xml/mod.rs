mod deserialize;
mod serialize;

use std::ops::Deref;

use proc_macro2::Ident as Ident2;
use quote::format_ident;

use crate::generator::misc::Occurs;
use crate::types::{Ident, Types};

use super::super::data::{AttributeData, ComplexTypeData, DynamicData, ElementData, TypeInfoData};
use super::super::misc::TypeRef;

pub(crate) struct QuickXmlRenderer;

/* DynamicTypeImpl */

#[derive(Debug)]
#[allow(dead_code)]
struct DynamicTypeImpl<'a, 'b, 'types> {
    inner: &'a DynamicData<'b, 'types>,

    type_ref: &'a TypeRef,
    type_ident: &'a Ident2,

    serializer_ident: Ident2,
    deserializer_ident: Ident2,
}

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
enum ComplexTypeImpl<'a, 'b, 'types> {
    Simple {
        simple: ComplexTypeImplSimple<'a, 'b, 'types>,
    },
    Complex {
        complex: ComplexTypeImplComplex<'a, 'b, 'types>,
    },
    ComplexWithContent {
        complex: ComplexTypeImplComplex<'a, 'b, 'types>,
        content: ComplexTypeImplSimple<'a, 'b, 'types>,
    },
}

#[derive(Debug)]
struct ComplexTypeImplBase<'a, 'b, 'types> {
    inner: &'a ComplexTypeData<'b, 'types>,
    type_ident: &'a Ident2,

    serializer_ident: Ident2,
    serializer_state_ident: Ident2,
}

#[derive(Debug)]
struct ComplexTypeImplSimple<'a, 'b, 'types> {
    base: ComplexTypeImplBase<'a, 'b, 'types>,

    elements: Vec<ElementImpl<'a, 'types>>,
}

#[derive(Debug)]
struct ComplexTypeImplComplex<'a, 'b, 'types> {
    base: ComplexTypeImplBase<'a, 'b, 'types>,

    tag_name: String,
    has_content: bool,
    content_occurs: Occurs,

    elements: Vec<ElementImpl<'a, 'types>>,
    attributes: Vec<AttributeImpl<'a, 'types>>,
}

#[derive(Debug)]
struct AttributeImpl<'a, 'types> {
    inner: &'a AttributeData<'types>,

    tag_name: String,
}

#[derive(Debug)]
struct ElementImpl<'a, 'types> {
    inner: &'a ElementData<'types>,

    tag_name: String,
}

/* DynamicTypeImpl */

impl<'a, 'b, 'types> DynamicTypeImpl<'a, 'b, 'types> {
    fn new(inner: &'a DynamicData<'b, 'types>) -> Self {
        let type_ref = inner.current_type_ref();
        let type_ident = &type_ref.type_ident;

        let serializer_ident = format_ident!("{type_ident}Serializer");
        let deserializer_ident = format_ident!("{type_ident}Deserializer");

        Self {
            inner,

            type_ref,
            type_ident,

            serializer_ident,
            deserializer_ident,
        }
    }
}

impl<'b, 'types> Deref for DynamicTypeImpl<'_, 'b, 'types> {
    type Target = DynamicData<'b, 'types>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

/* ComplexTypeImpl */

impl<'a, 'b, 'types> ComplexTypeImpl<'a, 'b, 'types> {
    fn new(inner: &'a ComplexTypeData<'b, 'types>) -> Self {
        let type_ref = inner.current_type_ref();
        let tag_name = make_tag_name(inner.types, &inner.ident);
        let type_ident = &type_ref.type_ident;

        let serializer_ident = format_ident!("{type_ident}Serializer");
        let serializer_state_ident = format_ident!("{type_ident}SerializerState");

        let base = ComplexTypeImplBase {
            inner,
            type_ident,

            serializer_ident,
            serializer_state_ident,
        };

        let elements = inner
            .elements
            .iter()
            .map(|element| ElementImpl::new(inner.types, element))
            .collect();

        match &inner.ty {
            TypeInfoData::Group(_) => ComplexTypeImpl::Simple {
                simple: ComplexTypeImplSimple { base, elements },
            },
            TypeInfoData::Complex(_) => {
                let has_content = !elements.is_empty() || inner.simple_content.is_some();
                let content_occurs = if inner.flatten_content {
                    Occurs::None
                } else {
                    inner.occurs
                };

                let attributes = inner
                    .attributes
                    .iter()
                    .map(|attribute| AttributeImpl::new(inner.types, attribute))
                    .collect();

                if inner.flatten_content || inner.simple_content.is_some() {
                    ComplexTypeImpl::Complex {
                        complex: ComplexTypeImplComplex {
                            base,

                            tag_name,
                            has_content,
                            content_occurs,

                            elements,
                            attributes,
                        },
                    }
                } else {
                    ComplexTypeImpl::ComplexWithContent {
                        complex: ComplexTypeImplComplex {
                            base,

                            tag_name,
                            has_content,
                            content_occurs,

                            elements: vec![],
                            attributes,
                        },
                        content: ComplexTypeImplSimple {
                            base: ComplexTypeImplBase {
                                inner,
                                type_ident: &inner.content_ident,

                                serializer_ident: format_ident!("{type_ident}ContentSerializer"),
                                serializer_state_ident: format_ident!(
                                    "{type_ident}ContentSerializerState"
                                ),
                            },

                            elements,
                        },
                    }
                }
            }
        }
    }
}

impl<'b, 'types> Deref for ComplexTypeImplBase<'_, 'b, 'types> {
    type Target = ComplexTypeData<'b, 'types>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, 'b, 'types> Deref for ComplexTypeImplSimple<'a, 'b, 'types> {
    type Target = ComplexTypeImplBase<'a, 'b, 'types>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a, 'b, 'types> Deref for ComplexTypeImplComplex<'a, 'b, 'types> {
    type Target = ComplexTypeImplBase<'a, 'b, 'types>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/* AttributeImpl */

impl<'a, 'types> AttributeImpl<'a, 'types> {
    fn new(types: &'types Types, inner: &'a AttributeData<'types>) -> Self {
        let tag_name = make_tag_name(types, &inner.ident);

        Self { inner, tag_name }
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
        let tag_name = make_tag_name(types, &inner.ident);

        Self { inner, tag_name }
    }
}

impl<'types> Deref for ElementImpl<'_, 'types> {
    type Target = ElementData<'types>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

fn make_tag_name(types: &Types, ident: &Ident) -> String {
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
