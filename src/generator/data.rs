use std::ops::{Deref, DerefMut};

use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::format_ident;
use tracing::instrument;

use crate::schema::NamespaceId;
use crate::schema::{xs::Use, MaxOccurs, MinOccurs};
use crate::types::{
    AnyAttributeInfo, AnyInfo, AttributeInfo, ComplexInfo, DynamicInfo, ElementInfo,
    EnumerationInfo, GroupInfo, Ident, Name, ReferenceInfo, Type, Types, UnionInfo, UnionTypeInfo,
    VariantInfo,
};

use super::misc::{
    format_field_ident, format_type_ident, format_type_ref, format_type_ref_ex,
    format_variant_ident, Occurs, TypeMode, TypeRef,
};
use super::renderer::{ImplRenderer, QuickXmlRenderer, TypeRenderer};
use super::{Error, GenerateFlags, Generator, TypedefMode};

/* TypeData */

#[derive(Debug)]
pub(super) struct TypeData<'a, 'types> {
    pub ty: &'types Type,
    pub ident: Ident,
    pub generator: &'a mut Generator<'types>,
}

macro_rules! render {
    (
        $data:expr,
        $render:ident,
        $render_quick_xml_serialize:ident,
        $render_quick_xml_deserialize:ident
    ) => {
        TypeRenderer.$render(&mut $data);
        ImplRenderer.$render(&mut $data);

        if $data.check_generate_flags(GenerateFlags::QUICK_XML_SERIALIZE) {
            QuickXmlRenderer.$render_quick_xml_serialize(&mut $data);
        }

        if $data.check_generate_flags(GenerateFlags::QUICK_XML_DESERIALIZE) {
            QuickXmlRenderer.$render_quick_xml_deserialize(&mut $data);
        }
    };
}

impl<'types> TypeData<'_, 'types> {
    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn generate_union(self, ty: &'types UnionInfo) -> Result<(), Error> {
        let mut data = UnionData::new(ty, self)?;

        render!(
            data,
            render_union,
            render_union_serialize,
            render_union_deserialize
        );

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn generate_dynamic(self, ty: &'types DynamicInfo) -> Result<(), Error> {
        let mut data = DynamicData::new(ty, self)?;

        render!(
            data,
            render_dynamic,
            render_dynamic_serialize,
            render_dynamic_deserialize
        );

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn generate_reference(self, ty: &'types ReferenceInfo) -> Result<(), Error> {
        let mut data = ReferenceData::new(ty, self)?;

        render!(
            data,
            render_reference,
            render_reference_serialize,
            render_reference_deserialize
        );

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn generate_enumeration(self, ty: &'types EnumerationInfo) -> Result<(), Error> {
        let mut data = EnumerationData::new(ty, self)?;

        render!(
            data,
            render_enumeration,
            render_enumeration_serialize,
            render_enumeration_deserialize
        );

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn generate_complex_type(self, ty: &'types ComplexInfo) -> Result<(), Error> {
        let mut data = ComplexTypeData::new_complex_type(ty, self)?;

        render!(
            data,
            render_complex_type,
            render_complex_type_serialize,
            render_complex_type_deserialize
        );

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn generate_all(self, ty: &'types GroupInfo) -> Result<(), Error> {
        let mut data = ComplexTypeData::new_all(ty, self)?;

        render!(
            data,
            render_complex_type,
            render_complex_type_serialize,
            render_complex_type_deserialize
        );

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn generate_choice(self, ty: &'types GroupInfo) -> Result<(), Error> {
        let mut data = ComplexTypeData::new_choice(ty, self)?;

        render!(
            data,
            render_complex_type,
            render_complex_type_serialize,
            render_complex_type_deserialize
        );

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn generate_sequence(self, ty: &'types GroupInfo) -> Result<(), Error> {
        let mut data = ComplexTypeData::new_sequence(ty, self)?;

        render!(
            data,
            render_complex_type,
            render_complex_type_serialize,
            render_complex_type_deserialize
        );

        Ok(())
    }

    pub(super) fn add_code(&mut self, code: TokenStream) {
        let ns = self
            .generate_flags
            .intersects(GenerateFlags::USE_MODULES)
            .then_some(self.ident.ns)
            .flatten();
        self.modules.add_code(ns, code);
    }

    pub(super) fn add_quick_xml_serialize_code(&mut self, code: TokenStream) {
        let ns = self
            .generate_flags
            .intersects(GenerateFlags::USE_MODULES)
            .then_some(self.ident.ns)
            .flatten();
        self.modules
            .get_mut(ns)
            .quick_xml_serialize
            .get_or_insert_default()
            .extend(code);
    }

    pub(super) fn add_quick_xml_deserialize_code(&mut self, code: TokenStream) {
        let ns = self
            .generate_flags
            .intersects(GenerateFlags::USE_MODULES)
            .then_some(self.ident.ns)
            .flatten();
        self.modules
            .get_mut(ns)
            .quick_xml_deserialize
            .get_or_insert_default()
            .extend(code);
    }

    pub(super) fn current_module(&self) -> Option<NamespaceId> {
        self.generate_flags
            .intersects(GenerateFlags::USE_MODULES)
            .then_some(self.ident.ns)
            .flatten()
    }

    pub(super) fn current_type_ref(&self) -> &TypeRef {
        self.cache.get(&self.ident).unwrap()
    }

    pub(super) fn current_type_ref_mut(&mut self) -> &mut TypeRef {
        let TypeData {
            ident, generator, ..
        } = self;

        generator.cache.get_mut(ident).unwrap()
    }
}

impl<'types> Deref for TypeData<'_, 'types> {
    type Target = Generator<'types>;

    fn deref(&self) -> &Self::Target {
        self.generator
    }
}

impl DerefMut for TypeData<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.generator
    }
}

/* TraitData */

#[derive(Debug)]
pub(super) struct TraitData {
    pub trait_ident: TokenStream,
}

/* UnionData */

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct UnionData<'a, 'types> {
    pub ty: &'types UnionInfo,
    pub inner: TypeData<'a, 'types>,
    pub variants: Vec<UnionVariantData<'types>>,
    pub trait_impls: Vec<TraitData>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct UnionVariantData<'types> {
    pub var: &'types UnionTypeInfo,
    pub variant_ident: Ident2,
    pub target_type: TokenStream,
}

impl<'a, 'types> UnionData<'a, 'types> {
    fn new(ty: &'types UnionInfo, mut inner: TypeData<'a, 'types>) -> Result<Self, Error> {
        let current_module = inner.current_module();
        let trait_impls = make_trait_impls(&mut inner)?;
        let variants = ty
            .types
            .iter()
            .map(|var| {
                let type_ref = inner.get_or_create_type_ref(var.type_.clone())?;
                let variant_ident =
                    format_variant_ident(&var.type_.name, var.display_name.as_deref());
                let target_type = format_type_ref(current_module, type_ref);

                Ok(UnionVariantData {
                    var,
                    variant_ident,
                    target_type,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            ty,
            inner,
            variants,
            trait_impls,
        })
    }
}

impl<'a, 'types> Deref for UnionData<'a, 'types> {
    type Target = TypeData<'a, 'types>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for UnionData<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/* DynamicData */

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct DynamicData<'a, 'types> {
    pub ty: &'types DynamicInfo,
    pub inner: TypeData<'a, 'types>,
    pub sub_traits: Option<Vec<TokenStream>>,
    pub trait_ident: Ident2,
    pub derived_types: Vec<DerivedTypeData>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct DerivedTypeData {
    pub ident: Ident,
    pub s_name: String,
    pub b_name: Literal,
    pub target_ident: TokenStream,
    pub variant_ident: Ident2,
}

impl<'a, 'types> DynamicData<'a, 'types> {
    fn new(ty: &'types DynamicInfo, mut inner: TypeData<'a, 'types>) -> Result<Self, Error> {
        let type_ident = &inner.current_type_ref().type_ident;
        let trait_ident = format_ident!("{type_ident}Trait");
        let ident = inner.ident.clone();
        let current_module = inner.current_module();
        let sub_traits = inner
            .get_traits()
            .get(&ident)
            .map(|info| info.traits_direct.clone())
            .map(|traits_direct| {
                traits_direct
                    .iter()
                    .map(|ident| {
                        inner.get_or_create_type_ref(ident.clone()).map(|x| {
                            let ident = format_ident!("{}Trait", x.type_ident);

                            format_type_ref_ex(current_module, None, &ident, x)
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let derived_types = ty
            .derived_types
            .iter()
            .map(|ident| make_derived_type_data(&mut inner, ident))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            ty,
            inner,
            sub_traits,
            trait_ident,
            derived_types,
        })
    }
}

impl<'a, 'types> Deref for DynamicData<'a, 'types> {
    type Target = TypeData<'a, 'types>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DynamicData<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn make_derived_type_data<'types>(
    inner: &mut TypeData<'_, 'types>,
    ident: &'types Ident,
) -> Result<DerivedTypeData, Error> {
    let s_name = ident.name.to_string();
    let b_name = Literal::byte_string(s_name.as_bytes());

    let ty = inner
        .types
        .get(ident)
        .ok_or_else(|| Error::UnknownType(ident.clone()))?;
    let ident = (if let Type::Dynamic(di) = ty {
        di.type_.clone()
    } else {
        None
    })
    .unwrap_or(ident.clone());

    let current_module = inner.current_module();
    let target_ref = inner.get_or_create_type_ref(ident.clone())?;
    let target_ident = format_type_ref(current_module, target_ref);
    let variant_ident = format_variant_ident(&ident.name, None);

    Ok(DerivedTypeData {
        ident,
        s_name,
        b_name,
        target_ident,
        variant_ident,
    })
}

/* ReferenceData */

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct ReferenceData<'a, 'types> {
    pub ty: &'types ReferenceInfo,
    pub mode: TypedefMode,
    pub inner: TypeData<'a, 'types>,
    pub occurs: Occurs,
    pub type_ident: Ident2,
    pub trait_impls: Vec<TraitData>,
    pub target_ident: TokenStream,
}

impl<'a, 'types> ReferenceData<'a, 'types> {
    fn new(ty: &'types ReferenceInfo, mut inner: TypeData<'a, 'types>) -> Result<Self, Error> {
        let occurs = Occurs::from_occurs(ty.min_occurs, ty.max_occurs);
        let current_module = inner.current_module();
        let type_ident = inner.current_type_ref().type_ident.clone();
        let target_ref = inner.get_or_create_type_ref(ty.type_.clone())?;
        let target_ident = format_type_ref(current_module, target_ref);
        let trait_impls = make_trait_impls(&mut inner)?;

        let mode = match (inner.typedef_mode, occurs) {
            (TypedefMode::Auto, Occurs::None | Occurs::Single) => TypedefMode::Typedef,
            (TypedefMode::Auto, _) => TypedefMode::NewType,
            (mode, _) => mode,
        };

        Ok(Self {
            ty,
            mode,
            inner,
            occurs,
            type_ident,
            trait_impls,
            target_ident,
        })
    }
}

impl<'a, 'types> Deref for ReferenceData<'a, 'types> {
    type Target = TypeData<'a, 'types>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ReferenceData<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/* EnumerationData */

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct EnumerationData<'a, 'types> {
    pub ty: &'types EnumerationInfo,
    pub inner: TypeData<'a, 'types>,
    pub type_ident: Ident2,
    pub variants: Vec<EnumVariantData<'types>>,
    pub trait_impls: Vec<TraitData>,
}

#[derive(Debug)]
pub(super) struct EnumVariantData<'types> {
    pub var: &'types VariantInfo,
    pub variant_ident: Ident2,
    pub target_type: Option<TokenStream>,
}

impl<'a, 'types> EnumerationData<'a, 'types> {
    fn new(ty: &'types EnumerationInfo, mut inner: TypeData<'a, 'types>) -> Result<Self, Error> {
        let mut unknown = 0usize;
        let current_module = inner.current_module();
        let type_ident = inner.current_type_ref().type_ident.clone();
        let trait_impls = make_trait_impls(&mut inner)?;

        let variants = ty
            .variants
            .iter()
            .filter_map(|var| match var.use_ {
                Use::Prohibited => None,
                Use::Required | Use::Optional => {
                    let type_ref = if let Some(t) = &var.type_ {
                        match inner.get_or_create_type_ref(t.clone()) {
                            Ok(target_ref) => Some(target_ref),
                            Err(error) => return Some(Err(error)),
                        }
                    } else {
                        None
                    };

                    let variant_ident = if let Some(display_name) = var.display_name.as_deref() {
                        format_ident!("{display_name}")
                    } else if let (Some(type_ref), true) = (type_ref, var.ident.name.is_unnamed()) {
                        type_ref.type_ident.clone()
                    } else if matches!(var.ident.name.as_str(), Some("")) {
                        unknown += 1;

                        format_ident!("Unknown{unknown}")
                    } else {
                        format_variant_ident(&var.ident.name, var.display_name.as_deref())
                    };

                    let target_type = type_ref.map(|x| format_type_ref(current_module, x));

                    Some(Ok(EnumVariantData {
                        var,
                        variant_ident,
                        target_type,
                    }))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(EnumerationData {
            ty,
            inner,
            type_ident,
            variants,
            trait_impls,
        })
    }
}

impl<'a, 'types> Deref for EnumerationData<'a, 'types> {
    type Target = TypeData<'a, 'types>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for EnumerationData<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/* ComplexTypeData */

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct ComplexTypeData<'a, 'types> {
    pub ty: TypeInfoData<'types>,
    pub inner: TypeData<'a, 'types>,

    pub occurs: Occurs,
    pub source_mode: TypeMode,
    pub target_mode: TypeMode,
    pub content_ident: Ident2,

    pub trait_impls: Vec<TraitData>,
    pub simple_content: Option<SimpleContentData<'types>>,

    pub attributes: Vec<AttributeData<'types>>,
    pub any_attribute: Option<&'types AnyAttributeInfo>,

    pub elements: Vec<ElementData<'types>>,
    pub any_element: Option<&'types AnyInfo>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(super) enum TypeInfoData<'types> {
    Group(&'types GroupInfo),
    Complex(&'types ComplexInfo),
}

#[derive(Debug)]
#[allow(dead_code)]
pub(super) struct SimpleContentData<'types> {
    pub ident: &'types Ident,
    pub target_type: TokenStream,
}

#[derive(Debug)]
pub(super) struct ElementData<'types> {
    pub element: &'types ElementInfo,
    pub occurs: Occurs,
    pub need_box: bool,
    pub is_dynamic: bool,
    pub field_ident: Ident2,
    pub variant_ident: Ident2,
    pub target_type: TokenStream,
    pub target_is_complex: bool,
}

#[derive(Debug)]
pub(super) struct AttributeData<'types> {
    pub attrib: &'types AttributeInfo,
    pub is_option: bool,
    pub field_ident: Ident2,
    pub target_type: TokenStream,
    pub default_value: Option<TokenStream>,
}

impl<'a, 'types> ComplexTypeData<'a, 'types> {
    #[instrument(err, level = "trace", skip(inner))]
    fn new_all(ty: &'types GroupInfo, inner: TypeData<'a, 'types>) -> Result<Self, Error> {
        let target_mode = inner.target_mode(TypeMode::All);

        Self::new(
            TypeInfoData::Group(ty),
            inner,
            TypeMode::All,
            target_mode,
            1,
            MaxOccurs::Bounded(1),
            &[],
            None,
            &ty.elements,
            ty.any.as_ref(),
        )
    }

    #[instrument(err, level = "trace", skip(inner))]
    fn new_choice(ty: &'types GroupInfo, inner: TypeData<'a, 'types>) -> Result<Self, Error> {
        let target_mode = inner.target_mode(TypeMode::Choice);

        Self::new(
            TypeInfoData::Group(ty),
            inner,
            TypeMode::Choice,
            target_mode,
            1,
            MaxOccurs::Bounded(1),
            &[],
            None,
            &ty.elements,
            ty.any.as_ref(),
        )
    }

    #[instrument(err, level = "trace", skip(inner))]
    fn new_sequence(ty: &'types GroupInfo, inner: TypeData<'a, 'types>) -> Result<Self, Error> {
        let target_mode = inner.target_mode(TypeMode::Sequence);

        Self::new(
            TypeInfoData::Group(ty),
            inner,
            TypeMode::Sequence,
            target_mode,
            1,
            MaxOccurs::Bounded(1),
            &[],
            None,
            &ty.elements,
            ty.any.as_ref(),
        )
    }

    #[instrument(err, level = "trace", skip(inner))]
    fn new_complex_type(
        ty: &'types ComplexInfo,
        inner: TypeData<'a, 'types>,
    ) -> Result<Self, Error> {
        let (source_mode, target_mode, elements, any_element) = match ty
            .content
            .as_ref()
            .and_then(|ident| inner.types.get_resolved(ident))
        {
            None => (TypeMode::Sequence, TypeMode::Sequence, &[][..], None),
            Some(Type::All(si)) => (
                TypeMode::All,
                inner.target_mode(TypeMode::All),
                &si.elements[..],
                si.any.as_ref(),
            ),
            Some(Type::Choice(si)) => (
                TypeMode::Choice,
                inner.target_mode(TypeMode::Choice),
                &si.elements[..],
                si.any.as_ref(),
            ),
            Some(Type::Sequence(si)) => (
                TypeMode::Sequence,
                inner.target_mode(TypeMode::Sequence),
                &si.elements[..],
                si.any.as_ref(),
            ),
            Some(Type::BuildIn(_) | Type::Union(_) | Type::Enumeration(_) | Type::Reference(_)) => {
                (TypeMode::Simple, TypeMode::Simple, &[][..], None)
            }
            x => {
                let ident = &inner.current_type_ref().type_ident;

                tracing::warn!("Complex type has unexpected content: ident={ident}, ty={ty:#?}, content={x:#?}!");

                (TypeMode::Sequence, TypeMode::Sequence, &[][..], None)
            }
        };

        Self::new(
            TypeInfoData::Complex(ty),
            inner,
            source_mode,
            target_mode,
            ty.min_occurs,
            ty.max_occurs,
            &ty.attributes,
            ty.any_attribute.as_ref(),
            elements,
            any_element,
        )
    }

    #[instrument(err, level = "trace", skip(inner))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        ty: TypeInfoData<'types>,
        mut inner: TypeData<'a, 'types>,
        source_mode: TypeMode,
        target_mode: TypeMode,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeInfo],
        any_attribute: Option<&'types AnyAttributeInfo>,
        elements: &'types [ElementInfo],
        any_element: Option<&'types AnyInfo>,
    ) -> Result<Self, Error> {
        let (occurs, elements) = make_element_data(
            &mut inner,
            elements,
            min_occurs,
            max_occurs,
            source_mode,
            target_mode,
        )?;
        let attributes = make_attribute_data(&mut inner, attributes)?;

        let trait_impls = make_trait_impls(&mut inner)?;
        let type_ref = inner.current_type_ref();
        let type_ident = &type_ref.type_ident;
        let content_ident = format_type_ident(&Name::new(format!("{type_ident}Content")), None);

        let simple_content = make_simple_content_data(&ty, &mut inner, target_mode)?;

        Ok(Self {
            ty,
            inner,

            occurs,
            source_mode,
            target_mode,
            content_ident,

            trait_impls,
            simple_content,

            attributes,
            any_attribute,

            elements,
            any_element,
        })
    }
}

impl<'a, 'types> Deref for ComplexTypeData<'a, 'types> {
    type Target = TypeData<'a, 'types>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ComplexTypeData<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Deref for ElementData<'_> {
    type Target = ElementInfo;

    fn deref(&self) -> &Self::Target {
        self.element
    }
}

impl Deref for AttributeData<'_> {
    type Target = AttributeInfo;

    fn deref(&self) -> &Self::Target {
        self.attrib
    }
}

#[instrument(err, level = "trace", skip(inner))]
fn make_simple_content_data<'types>(
    ty: &TypeInfoData<'types>,
    inner: &mut TypeData<'_, 'types>,
    target_mode: TypeMode,
) -> Result<Option<SimpleContentData<'types>>, Error> {
    if target_mode != TypeMode::Simple {
        return Ok(None);
    }

    let TypeInfoData::Complex(ci) = &ty else {
        return Ok(None);
    };

    let Some(ident) = ci.content.as_ref() else {
        return Ok(None);
    };

    let current_module = inner.current_module();
    let content_ref = inner.get_or_create_type_ref(ident.clone())?;

    let target_type = format_type_ref(current_module, content_ref);

    Ok(Some(SimpleContentData { ident, target_type }))
}

#[instrument(err, level = "trace", skip(inner))]
fn make_element_data<'types>(
    inner: &mut TypeData<'_, 'types>,
    elements: &'types [ElementInfo],
    min_occurs: MinOccurs,
    max_occurs: MaxOccurs,
    source_mode: TypeMode,
    target_mode: TypeMode,
) -> Result<(Occurs, Vec<ElementData<'types>>), Error> {
    let mut min = None;
    let mut max = None;

    let elements = elements
        .iter()
        .filter_map(|element| {
            let mut occurs = Occurs::from_occurs(element.min_occurs, element.max_occurs);
            if occurs == Occurs::None {
                return None;
            }

            match source_mode {
                TypeMode::Choice => {
                    let min = min.get_or_insert(element.min_occurs);
                    let max = max.get_or_insert(element.max_occurs);

                    *min = (*min).min(element.min_occurs);
                    *max = (*max).max(element.max_occurs);

                    if matches!(target_mode, TypeMode::All | TypeMode::Sequence)
                        && occurs == Occurs::Single
                    {
                        occurs = Occurs::Optional;
                    }
                }
                TypeMode::All | TypeMode::Sequence => {
                    *min.get_or_insert(0) += element.min_occurs;
                    *max.get_or_insert(MaxOccurs::Bounded(0)) += element.max_occurs;
                }
                TypeMode::Simple => crate::unreachable!(),
            }

            let current_module = inner.current_module();
            let target_ref = match inner.get_or_create_type_ref(element.type_.clone()) {
                Ok(target_ref) => target_ref,
                Err(error) => return Some(Err(error)),
            };
            let target_type = format_type_ref(current_module, target_ref);
            let need_box = inner
                .current_type_ref()
                .boxed_elements
                .contains(&element.ident);
            let is_dynamic = is_dynamic(&element.type_, inner.types);
            let field_ident =
                format_field_ident(&element.ident.name, element.display_name.as_deref());
            let variant_ident =
                format_variant_ident(&element.ident.name, element.display_name.as_deref());
            let target_is_complex =
                target_is_complex(&element.type_, inner.types, inner.typedef_mode);

            Some(Ok(ElementData {
                element,
                occurs,
                need_box,
                is_dynamic,
                field_ident,
                variant_ident,
                target_type,
                target_is_complex,
            }))
        })
        .collect::<Result<_, _>>()?;

    let occurs = match (min, max) {
        (None, None) => Occurs::None,
        (Some(min), Some(max)) => Occurs::from_occurs(min_occurs * min, max_occurs * max),
        (_, _) => crate::unreachable!(),
    };

    Ok((occurs, elements))
}

fn is_dynamic(ident: &Ident, types: &Types) -> bool {
    let Some(ty) = types.get(ident) else {
        return false;
    };

    match ty {
        Type::Dynamic(_) => true,
        Type::ComplexType(ci) => ci.is_dynamic,
        Type::Reference(x) if x.is_single() => is_dynamic(&x.type_, types),
        _ => false,
    }
}

fn target_is_complex(ident: &Ident, types: &Types, mode: TypedefMode) -> bool {
    let Some(ty) = types.get(ident) else {
        return false;
    };

    match ty {
        Type::All(_) | Type::Choice(_) | Type::Sequence(_) | Type::ComplexType(_) => true,
        Type::Reference(x) if x.is_single() && mode != TypedefMode::NewType => {
            target_is_complex(&x.type_, types, mode)
        }
        _ => false,
    }
}

#[instrument(err, level = "trace", skip(inner))]
fn make_attribute_data<'types>(
    inner: &mut TypeData<'_, 'types>,
    attributes: &'types [AttributeInfo],
) -> Result<Vec<AttributeData<'types>>, Error> {
    attributes
        .iter()
        .filter_map(move |attrib| {
            if attrib.use_ == Use::Prohibited {
                return None;
            }

            let current_module = inner.current_module();
            let target_ref = match inner.get_or_create_type_ref(attrib.type_.clone()) {
                Ok(target_ref) => target_ref,
                Err(error) => return Some(Err(error)),
            };
            let field_ident =
                format_field_ident(&attrib.ident.name, attrib.display_name.as_deref());
            let target_type = format_type_ref(current_module, target_ref);

            let default_value = attrib
                .default
                .as_ref()
                .map(|default| inner.get_default(current_module, default, &attrib.type_));
            let default_value = match default_value {
                None => None,
                Some(Ok(default)) => Some(default),
                Some(Err(error)) => return Some(Err(error)),
            };
            let is_option = matches!((&attrib.use_, &default_value), (Use::Optional, None));

            Some(Ok(AttributeData {
                attrib,
                is_option,
                field_ident,
                target_type,
                default_value,
            }))
        })
        .collect()
}

fn make_trait_impls(inner: &mut TypeData<'_, '_>) -> Result<Vec<TraitData>, Error> {
    let ident = inner.ident.clone();
    let current_module = inner.current_module();

    inner
        .get_traits()
        .get(&ident)
        .into_iter()
        .flat_map(|info| &info.traits_all)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|ident| {
            let type_ref = inner.get_or_create_type_ref(ident.clone())?;
            let ident = format_ident!("{}Trait", type_ref.type_ident);
            let trait_ident = format_type_ref_ex(current_module, None, &ident, type_ref);

            Ok(TraitData { trait_ident })
        })
        .collect::<Result<Vec<_>, _>>()
}
