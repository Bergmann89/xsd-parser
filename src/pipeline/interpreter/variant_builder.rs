use std::borrow::Cow;
use std::collections::btree_map::Entry;
use std::ops::{Deref, DerefMut};
use std::str::from_utf8;

use tracing::instrument;

use crate::models::{
    meta::{
        AnyAttributeMeta, AnyMeta, AttributeMeta, Base, DynamicMeta, ElementMeta,
        ElementMetaVariant, ElementMode, ElementsMeta, EnumerationMetaVariant, MetaType,
        MetaTypeVariant, ReferenceMeta, UnionMetaType,
    },
    schema::{
        xs::{
            Annotation, Any, AnyAttribute, AttributeGroupType, AttributeType, ComplexBaseType,
            ComplexContent, ElementType, ExtensionType, Facet, FacetType, GroupType, List,
            QNameList, Restriction, RestrictionType, SimpleBaseType, SimpleContent, Union, Use,
        },
        MaxOccurs, MinOccurs, Namespace,
    },
    Ident, IdentType, Name,
};
use crate::traits::VecHelper;

use super::{Error, SchemaInterpreter, StackEntry};

#[derive(Debug)]
pub(super) struct VariantBuilder<'a, 'schema, 'state> {
    /// Type variant that is constructed by the builder
    variant: Option<MetaTypeVariant>,

    /// `true` if `type_` is fixed and can not be changed anymore
    fixed: bool,

    /// Mode of the constructed type
    type_mode: TypeMode,

    /// Mode of the content of the constructed type
    content_mode: ContentMode,

    /// Documentation of the currently build type extracted from `xs:annotation`
    /// and `xs:documentation` nodes.
    documentation: Vec<String>,

    /// `true` if the simple content type of a complex type was already duplicated
    /// and is now unique for this type.
    is_simple_content_unique: bool,

    owner: &'a mut SchemaInterpreter<'schema, 'state>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TypeMode {
    Unknown,
    Simple,
    Complex,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ContentMode {
    Unknown,
    Simple,
    Complex,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum UpdateMode {
    Extension,
    Restriction,
}

#[derive(Debug, Clone, Copy)]
enum ComplexContentType {
    All,
    Choice,
    Sequence,
}

/* any type */

/// Initialize the type of a `$builder` to any type `$variant`.
macro_rules! init_any {
    ($builder:expr, $variant:ident) => {
        init_any!($builder, $variant, Default::default(), true)
    };
    ($builder:expr, $variant:ident, $value:expr, $fixed:expr) => {{
        $builder.variant = Some(MetaTypeVariant::$variant($value));
        $builder.fixed = $fixed;

        let MetaTypeVariant::$variant(ret) = $builder.variant.as_mut().unwrap() else {
            crate::unreachable!();
        };

        ret
    }};
}

/// Get the type `$variant` of a `$builder` or set the type variant if unset.
macro_rules! get_or_init_any {
    ($builder:expr, $variant:ident) => {
        get_or_init_any!($builder, $variant, Default::default())
    };
    ($builder:expr, $variant:ident, $default:expr) => {
        match &mut $builder.variant {
            None => init_any!($builder, $variant, $default, true),
            Some(MetaTypeVariant::$variant(ret)) => ret,
            _ if !$builder.fixed => init_any!($builder, $variant, $default, true),
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    };
}

/// Get the `GroupMeta` from any possible type or initialize the required variant.
macro_rules! get_or_init_type {
    ($builder:expr, $variant:ident) => {
        get_or_init_type!($builder, $variant, Default::default())
    };
    ($builder:expr, $variant:ident, $default:expr) => {
        match &mut $builder.variant {
            None => init_any!($builder, $variant, $default, true),
            Some(
                MetaTypeVariant::All(si)
                | MetaTypeVariant::Choice(si)
                | MetaTypeVariant::Sequence(si),
            ) => si,
            _ if !$builder.fixed => init_any!($builder, $variant, $default, true),
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    };
}

/* TypeBuilder */

impl<'a, 'schema, 'state> VariantBuilder<'a, 'schema, 'state> {
    pub(super) fn new(owner: &'a mut SchemaInterpreter<'schema, 'state>) -> Self {
        Self {
            variant: None,
            fixed: false,
            type_mode: TypeMode::Unknown,
            content_mode: ContentMode::Unknown,
            documentation: Vec::new(),
            is_simple_content_unique: false,
            owner,
        }
    }

    pub(super) fn finish(self) -> Result<MetaType, Error> {
        let variant = self.variant.ok_or(Error::NoType)?;

        let mut type_ = MetaType::new(variant);
        type_.documentation = self.documentation;

        Ok(type_)
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_element(
        &mut self,
        ty: &ElementType,
        extract_docs: bool,
    ) -> Result<(), Error> {
        use crate::models::schema::xs::ElementTypeContent as C;

        if let Some(type_) = &ty.type_ {
            let type_ = self.parse_qname(type_)?;

            init_any!(self, Reference, ReferenceMeta::new(type_), true);
        } else if !ty.content.is_empty() {
            let ident = self
                .state
                .current_ident()
                .unwrap()
                .clone()
                .with_type(IdentType::ElementType);

            let mut has_content = false;
            let type_ = self.create_type(ident, |builder| {
                for c in &ty.content {
                    match c {
                        C::Annotation(_)
                        | C::Key(_)
                        | C::Alternative(_)
                        | C::Unique(_)
                        | C::Keyref(_) => {}
                        C::SimpleType(x) => {
                            has_content = true;
                            builder.apply_simple_type(x)?;
                        }
                        C::ComplexType(x) => {
                            has_content = true;
                            builder.apply_complex_type(x)?;
                        }
                    }
                }

                Ok(())
            });

            if has_content {
                init_any!(self, Reference, ReferenceMeta::new(type_?), true);
            } else {
                // No actual type content found, default to xs:anyType
                let xs = self
                    .schemas
                    .resolve_namespace(&Some(Namespace::XS))
                    .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;
                let ident = Ident::ANY_TYPE.with_ns(Some(xs));

                init_any!(self, Reference, ReferenceMeta::new(ident), true);
            }
        } else {
            let xs = self
                .schemas
                .resolve_namespace(&Some(Namespace::XS))
                .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;
            let ident = Ident::ANY_TYPE.with_ns(Some(xs));

            init_any!(self, Reference, ReferenceMeta::new(ident), true);
        }

        if extract_docs {
            for content in &ty.content {
                if let C::Annotation(ty) = content {
                    self.apply_annotation(ty);
                }
            }
        }

        if ty.abstract_ {
            let type_ = match self.variant.take() {
                None => None,
                Some(MetaTypeVariant::Reference(ti)) => Some(ti.type_),
                e => crate::unreachable!("Unexpected type: {:?}", e),
            };

            let ai = init_any!(self, Dynamic);
            ai.type_ = type_;
        }

        if let Some(substitution_group) = &ty.substitution_group {
            self.walk_substitution_groups(substitution_group, |builder, base_ident| {
                let ident = builder.state.current_ident().unwrap().clone();
                let base_ty = builder.get_element_mut(base_ident)?;

                if let MetaTypeVariant::Reference(ti) = &mut base_ty.variant {
                    base_ty.variant = MetaTypeVariant::Dynamic(DynamicMeta {
                        type_: Some(ti.type_.clone()),
                        derived_types: vec![ti.type_.clone()],
                    });
                }

                let MetaTypeVariant::Dynamic(ai) = &mut base_ty.variant else {
                    return Err(Error::ExpectedDynamicElement(base_ident.clone()));
                };

                ai.derived_types.push(ident);

                Ok(())
            })?;
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_attribute(&mut self, ty: &AttributeType) -> Result<(), Error> {
        if let Some(type_) = &ty.type_ {
            let type_ = self.parse_qname(type_)?;
            init_any!(self, Reference, ReferenceMeta::new(type_), false);
        } else if let Some(x) = &ty.simple_type {
            self.apply_simple_type(x)?;
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_simple_type(&mut self, ty: &SimpleBaseType) -> Result<(), Error> {
        use crate::models::schema::xs::SimpleBaseTypeContent as C;

        if self.type_mode == TypeMode::Unknown {
            self.type_mode = TypeMode::Simple;
        }

        self.content_mode = ContentMode::Simple;

        for c in &ty.content {
            match c {
                C::Annotation(x) => self.apply_annotation(x),
                C::Restriction(x) => self.apply_simple_type_restriction(x)?,
                C::Union(x) => self.apply_union(x)?,
                C::List(x) => self.apply_list(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_type_restriction(&mut self, ty: &Restriction) -> Result<(), Error> {
        use crate::models::schema::xs::RestrictionContent as C;

        let base = ty
            .base
            .as_ref()
            .map(|base| {
                let base = self.parse_qname(base)?;

                self.copy_base_type(&base, UpdateMode::Restriction)?;

                Ok(base)
            })
            .transpose()?;

        for c in &ty.content {
            match c {
                C::Annotation(x) => self.apply_annotation(x),
                C::SimpleType(x) => self.apply_simple_type(x)?,
                C::Facet(x) => self.apply_facet(x)?,
            }
        }

        if let Some(base) = base {
            match &mut self.variant {
                Some(MetaTypeVariant::BuildIn(_) | MetaTypeVariant::Reference(_)) => (),
                Some(MetaTypeVariant::Union(e)) => e.base = Base::Extension(base),
                Some(MetaTypeVariant::Enumeration(e)) => e.base = Base::Extension(base),
                Some(MetaTypeVariant::ComplexType(e)) => e.base = Base::Extension(base),
                e => unreachable!("Unexpected type: {e:#?}"),
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_complex_type(&mut self, ty: &ComplexBaseType) -> Result<(), Error> {
        use crate::models::schema::xs::ComplexBaseTypeContent as C;

        self.type_mode = TypeMode::Complex;
        self.content_mode = ContentMode::Complex;

        let ci = get_or_init_any!(self, ComplexType);
        if let Some(mixed) = ty.mixed {
            ci.is_mixed = mixed;
            self.state.type_stack.push(StackEntry::Mixed(mixed));
        }

        let mut ret = Ok(());

        for c in &ty.content {
            ret = match c {
                C::OpenContent(_) | C::Assert(_) => Ok(()),
                C::ComplexContent(x) => {
                    let ci = get_or_init_any!(self, ComplexType);
                    ci.is_dynamic = ty.abstract_;

                    self.apply_complex_content(x)
                }
                C::SimpleContent(x) => self.apply_simple_content(x),
                C::All(x) => self.apply_all(x),
                C::Choice(x) => self.apply_choice(x),
                C::Sequence(x) => self.apply_sequence(x),
                C::Attribute(x) => self.apply_attribute_ref(x),
                C::AnyAttribute(x) => self.apply_any_attribute(x),
                C::Group(x) => self.apply_group_ref(x),
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x),
                C::Annotation(x) => {
                    self.apply_annotation(x);
                    Ok(())
                }
            };

            if ret.is_err() {
                break;
            }
        }

        if ty.mixed.is_some() {
            self.state.type_stack.pop();
        }

        ret
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_content(&mut self, ty: &SimpleContent) -> Result<(), Error> {
        use crate::models::schema::xs::SimpleContentContent as C;

        self.content_mode = ContentMode::Simple;

        for c in &ty.content {
            match c {
                C::Annotation(x) => self.apply_annotation(x),
                C::Extension(x) => self.apply_simple_content_extension(x)?,
                C::Restriction(x) => self.apply_simple_content_restriction(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_complex_content(&mut self, ty: &ComplexContent) -> Result<(), Error> {
        use crate::models::schema::xs::ComplexContentContent as C;

        let ci = get_or_init_any!(self, ComplexType);
        if let Some(mixed) = ty.mixed {
            ci.is_mixed = mixed;
            self.state.type_stack.push(StackEntry::Mixed(mixed));
        }

        let mut ret = Ok(());

        for c in &ty.content {
            ret = match c {
                C::Annotation(x) => {
                    self.apply_annotation(x);
                    Ok(())
                }
                C::Extension(x) => self.apply_complex_content_extension(x),
                C::Restriction(x) => self.apply_complex_content_restriction(x),
            };

            if ret.is_err() {
                break;
            }
        }

        if ty.mixed.is_some() {
            self.state.type_stack.pop();
        }

        ret
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_content_extension(&mut self, ty: &ExtensionType) -> Result<(), Error> {
        let base = self.parse_qname(&ty.base)?;

        self.copy_base_type(&base, UpdateMode::Extension)?;
        self.apply_extension(ty)?;

        match &mut self.variant {
            Some(MetaTypeVariant::Reference(_)) => (),
            Some(MetaTypeVariant::Union(e)) => e.base = Base::Extension(base),
            Some(MetaTypeVariant::Enumeration(e)) => e.base = Base::Extension(base),
            Some(MetaTypeVariant::ComplexType(e)) => e.base = Base::Extension(base),
            e => crate::unreachable!("Unexpected type: {e:#?}!"),
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_content_restriction(&mut self, ty: &RestrictionType) -> Result<(), Error> {
        let base = self.parse_qname(&ty.base)?;

        self.copy_base_type(&base, UpdateMode::Restriction)?;
        self.apply_restriction(ty)?;

        match &mut self.variant {
            Some(MetaTypeVariant::Reference(_)) => (),
            Some(MetaTypeVariant::Union(e)) => e.base = Base::Restriction(base),
            Some(MetaTypeVariant::Enumeration(e)) => e.base = Base::Restriction(base),
            Some(MetaTypeVariant::ComplexType(e)) => e.base = Base::Restriction(base),
            e => crate::unreachable!("Unexpected type: {e:#?}!"),
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_complex_content_extension(&mut self, ty: &ExtensionType) -> Result<(), Error> {
        let base = self.parse_qname(&ty.base)?;

        self.copy_base_type(&base, UpdateMode::Extension)?;
        self.apply_extension(ty)?;

        let ci = get_or_init_any!(self, ComplexType);
        ci.base = Base::Extension(base);

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_complex_content_restriction(&mut self, ty: &RestrictionType) -> Result<(), Error> {
        let base = self.parse_qname(&ty.base)?;

        self.copy_base_type(&base, UpdateMode::Restriction)?;
        self.apply_restriction(ty)?;

        let ci = get_or_init_any!(self, ComplexType);
        ci.base = Base::Restriction(base);

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_extension(&mut self, ty: &ExtensionType) -> Result<(), Error> {
        use crate::models::schema::xs::ExtensionTypeContent as C;

        for c in &ty.content {
            match c {
                C::OpenContent(_) | C::Assert(_) => (),
                C::Annotation(x) => self.apply_annotation(x),
                C::All(x) => self.apply_all(x)?,
                C::AnyAttribute(x) => self.apply_any_attribute(x)?,
                C::Attribute(x) => self.apply_attribute_ref(x)?,
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x)?,
                C::Choice(x) => self.apply_choice(x)?,
                C::Group(x) => self.apply_group_ref(x)?,
                C::Sequence(x) => self.apply_sequence(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_restriction(&mut self, ty: &RestrictionType) -> Result<(), Error> {
        use crate::models::schema::xs::RestrictionTypeContent as C;

        for c in &ty.content {
            match c {
                C::OpenContent(_) | C::Assert(_) => (),
                C::Annotation(x) => self.apply_annotation(x),
                C::All(x) => self.apply_all(x)?,
                C::AnyAttribute(x) => self.apply_any_attribute(x)?,
                C::Attribute(x) => self.apply_attribute_ref(x)?,
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x)?,
                C::Choice(x) => self.apply_choice(x)?,
                C::Facet(x) => self.apply_facet(x)?,
                C::Group(x) => self.apply_group_ref(x)?,
                C::Sequence(x) => self.apply_sequence(x)?,
                C::SimpleType(x) => self.apply_simple_type(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_all(&mut self, ty: &GroupType) -> Result<(), Error> {
        let (field_name, type_ident) = self.make_field_name_and_type(ty);

        self.complex_content_builder(
            field_name,
            ty.min_occurs,
            ty.max_occurs,
            type_ident,
            ComplexContentType::All,
            |builder| builder.apply_group(ty),
        )?;

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_choice(&mut self, ty: &GroupType) -> Result<(), Error> {
        let (field_name, type_ident) = self.make_field_name_and_type(ty);

        self.complex_content_builder(
            field_name,
            ty.min_occurs,
            ty.max_occurs,
            type_ident,
            ComplexContentType::Choice,
            |builder| builder.apply_group(ty),
        )?;

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_sequence(&mut self, ty: &GroupType) -> Result<(), Error> {
        let (field_name, type_ident) = self.make_field_name_and_type(ty);

        self.complex_content_builder(
            field_name,
            ty.min_occurs,
            ty.max_occurs,
            type_ident,
            ComplexContentType::Sequence,
            |builder| builder.apply_group(ty),
        )?;

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_group(&mut self, ty: &GroupType) -> Result<(), Error> {
        use crate::models::schema::xs::GroupTypeContent as C;

        let is_mixed = self.state.is_mixed();
        if let Some(
            MetaTypeVariant::All(gi) | MetaTypeVariant::Choice(gi) | MetaTypeVariant::Sequence(gi),
        ) = &mut self.variant
        {
            gi.is_mixed = is_mixed;
        }

        self.state.type_stack.push(StackEntry::Group);

        let mut ret = Ok(());

        for c in &ty.content {
            ret = match c {
                C::Annotation(x) => {
                    self.apply_annotation(x);
                    Ok(())
                }
                C::All(x) => self.apply_all(x),
                C::Choice(x) => self.apply_choice(x),
                C::Sequence(x) => self.apply_sequence(x),
                C::Any(x) => self.apply_any(x),
                C::Group(x) => self.apply_group_ref(x),
                C::Element(x) => self.apply_element_ref(x),
            };

            if ret.is_err() {
                break;
            }
        }

        self.state.type_stack.pop();

        ret
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_element_ref(&mut self, ty: &ElementType) -> Result<(), Error> {
        use crate::models::schema::xs::ElementTypeContent as C;

        let element = match ty {
            ElementType {
                ref_: Some(ref_),
                name,
                ..
            } => {
                let type_ = self.parse_qname(ref_)?.with_type(IdentType::Element);
                let name = self.state.name_builder().or(name).or(&type_.name).finish();
                let ident = Ident::new(name)
                    .with_ns(type_.ns)
                    .with_type(IdentType::Element);

                let ci = get_or_init_type!(self, Sequence);
                let element = ci.elements.find_or_insert(ident, |ident| {
                    ElementMeta::new(ident, type_, ElementMode::Element)
                });
                crate::assert!(matches!(
                    element.variant,
                    ElementMetaVariant::Type {
                        mode: ElementMode::Element,
                        ..
                    }
                ));
                element.update(ty);

                element
            }
            ty => {
                let field_name = self.state.name_builder().or(&ty.name).finish();
                let field_ident = Ident::new(field_name)
                    .with_ns(self.state.current_ns())
                    .with_type(IdentType::Element);
                let type_name = self
                    .state
                    .name_builder()
                    .extend(true, ty.name.clone())
                    .auto_extend(true, false, self.state);
                let type_name = if type_name.has_extension() {
                    type_name.with_id(false)
                } else {
                    type_name.shared_name("Temp")
                };
                let type_name = type_name.finish();

                let ns = self.state.current_ns();
                let type_ = if let Some(type_) = &ty.type_ {
                    self.parse_qname(type_)?
                } else {
                    self.create_element(ns, Some(type_name), ty, false)?
                };

                let ci = get_or_init_type!(self, Sequence);
                let element = ci.elements.find_or_insert(field_ident, |ident| {
                    ElementMeta::new(ident, type_, ElementMode::Element)
                });
                crate::assert!(matches!(
                    element.variant,
                    ElementMetaVariant::Type {
                        mode: ElementMode::Element,
                        ..
                    }
                ));
                element.update(ty);

                element
            }
        };

        for content in &ty.content {
            if let C::Annotation(x) = content {
                if let Err(error) = x.extract_documentation_into(&mut element.documentation) {
                    tracing::warn!(
                        "Unable to extract documentation for `xs:element` reference: {error}"
                    );
                }
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_group_ref(&mut self, ty: &GroupType) -> Result<(), Error> {
        use crate::models::schema::xs::GroupTypeContent as C;

        let ref_ = ty.ref_.as_ref().ok_or(Error::GroupMissingRef)?;
        let ref_ = self.parse_qname(ref_)?.with_type(IdentType::Group);
        let group = self
            .find_group(ref_.clone())
            .ok_or(Error::UnknownElement(ref_.clone()))?;

        let mut ret = Ok(());

        self.state.type_stack.push(StackEntry::GroupRef(ref_));

        for c in &group.content {
            ret = match c {
                C::Annotation(_) => Ok(()),
                C::Any(x) => self.apply_any(x),
                C::All(x) => self.apply_all(&x.patch(ty)),
                C::Choice(x) => self.apply_choice(&x.patch(ty)),
                C::Sequence(x) => self.apply_sequence(&x.patch(ty)),
                C::Group(x) => self.apply_group_ref(x),
                C::Element(x) => self.apply_element_ref(x),
            };

            if ret.is_err() {
                break;
            }
        }

        self.state.type_stack.pop();

        ret
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_attribute_group_ref(&mut self, ty: &AttributeGroupType) -> Result<(), Error> {
        use crate::models::schema::xs::AttributeGroupTypeContent as C;

        let ref_ = ty.ref_.as_ref().ok_or(Error::AttributeGroupMissingRef)?;
        let ref_ = self.parse_qname(ref_)?.with_type(IdentType::AttributeGroup);
        let group = self
            .find_attribute_group(ref_.clone())
            .ok_or(Error::UnknownElement(ref_))?;

        self.state.type_stack.push(StackEntry::AttributeGroupRef);

        let mut ret = Ok(());

        for c in &group.content {
            ret = match c {
                C::Annotation(x) => {
                    self.apply_annotation(x);
                    Ok(())
                }
                C::Attribute(x) => self.apply_attribute_ref(x),
                C::AnyAttribute(x) => self.apply_any_attribute(x),
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x),
            };

            if ret.is_err() {
                break;
            }
        }

        self.state.type_stack.pop();

        ret
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_attribute_ref(&mut self, ty: &AttributeType) -> Result<(), Error> {
        let attribute = match ty {
            AttributeType {
                name: Some(name),
                type_: Some(type_),
                ..
            } => {
                let type_ = self.parse_qname(type_)?;
                let name = Name::from(name.clone());
                let ident = Ident::new(name)
                    .with_ns(self.state.current_ns())
                    .with_type(IdentType::Attribute);

                let ci = get_or_init_any!(self, ComplexType);
                let attribute = ci
                    .attributes
                    .find_or_insert(ident, |ident| AttributeMeta::new(ident, type_));
                attribute.update(ty);

                attribute
            }
            AttributeType {
                ref_: Some(ref_),
                name,
                ..
            } => {
                let type_ = self.parse_qname(ref_)?.with_type(IdentType::Attribute);
                let name = self.state.name_builder().or(name).or(&type_.name).finish();
                let ident = Ident::new(name)
                    .with_ns(type_.ns)
                    .with_type(IdentType::Attribute);

                let ci = get_or_init_any!(self, ComplexType);
                let attribute = ci
                    .attributes
                    .find_or_insert(ident, |ident| AttributeMeta::new(ident, type_));
                attribute.update(ty);

                attribute
            }
            AttributeType {
                name: Some(name),
                simple_type,
                ..
            } => {
                let type_ = simple_type
                    .as_ref()
                    .map(|x| {
                        let type_name = self
                            .state
                            .name_builder()
                            .or(name)
                            .auto_extend(true, true, self.state)
                            .finish();
                        let ns = self.state.current_ns();

                        self.create_simple_type(ns, Some(type_name), x)
                    })
                    .transpose()?;
                let name = Name::from(name.clone());
                let ident = Ident::new(name)
                    .with_ns(self.state.current_ns())
                    .with_type(IdentType::Attribute);

                let ci = get_or_init_any!(self, ComplexType);
                let attribute = ci.attributes.find_or_insert(ident, |ident| {
                    AttributeMeta::new(ident, type_.unwrap_or(Ident::STRING))
                });
                attribute.update(ty);

                attribute
            }
            e => return Err(Error::InvalidAttributeReference(Box::new(e.clone()))),
        };

        if let Some(x) = &ty.annotation {
            if let Err(error) = x.extract_documentation_into(&mut attribute.documentation) {
                tracing::warn!(
                    "Unable to extract documentation for `xs:attribute` reference: {error}"
                );
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_any(&mut self, ty: &Any) -> Result<(), Error> {
        let name = self
            .state
            .name_builder()
            .shared_name("any")
            .or(&ty.id)
            .finish();
        let ident = Ident::new(name).with_type(IdentType::Element);

        let any = AnyMeta {
            id: ty.id.clone(),
            namespace: ty.namespace.clone(),
            not_q_name: ty.not_q_name.clone(),
            not_namespace: ty.not_namespace.clone(),
            process_contents: ty.process_contents.clone(),
        };

        let mut el = ElementMeta::any(ident, any);
        el.min_occurs = ty.min_occurs;
        el.max_occurs = ty.max_occurs;

        let si = get_or_init_type!(self, Sequence);
        si.elements.push_any(el);

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_any_attribute(&mut self, ty: &AnyAttribute) -> Result<(), Error> {
        let name = self
            .state
            .name_builder()
            .unique_name("any_attribute")
            .or(&ty.id)
            .finish();
        let ident = Ident::new(name).with_type(IdentType::Attribute);

        let ci = get_or_init_any!(self, ComplexType);
        ci.attributes.find_or_insert(ident, |ident| {
            let any = AnyAttributeMeta {
                id: ty.id.clone(),
                namespace: ty.namespace.clone(),
                not_q_name: ty.not_q_name.clone(),
                not_namespace: ty.not_namespace.clone(),
                process_contents: ty.process_contents.clone(),
            };

            AttributeMeta::any(ident, any)
        });

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_facet(&mut self, ty: &Facet) -> Result<(), Error> {
        match ty {
            Facet::Enumeration(x) => self.apply_enumeration(x)?,
            x => tracing::warn!("Unknown facet: {x:#?}"),
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_enumeration(&mut self, ty: &FacetType) -> Result<(), Error> {
        let name = Name::from(ty.value.trim().to_owned());
        let ident = Ident::new(name)
            .with_ns(self.state.current_ns())
            .with_type(IdentType::Enumeration);

        self.simple_content_builder(|builder| {
            let ei = get_or_init_any!(builder, Enumeration);

            let var = ei
                .variants
                .find_or_insert(ident, EnumerationMetaVariant::new);
            var.use_ = Use::Required;

            if let Some(x) = &ty.annotation {
                if let Err(error) = x.extract_documentation_into(&mut var.documentation) {
                    tracing::warn!("Unable to extract documentation for `xs:enumeration`: {error}");
                }
            }

            Ok(())
        })
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_union(&mut self, ty: &Union) -> Result<(), Error> {
        self.simple_content_builder(|builder| {
            let ui = get_or_init_any!(builder, Union);

            if let Some(types) = &ty.member_types {
                for type_ in &types.0 {
                    let type_ = builder.owner.parse_qname(type_)?;
                    ui.types.push(UnionMetaType::new(type_));
                }
            }

            let ns = builder.owner.state.current_ns();

            for x in &ty.simple_type {
                let name = builder
                    .owner
                    .state
                    .name_builder()
                    .or(&x.name)
                    .auto_extend(false, true, builder.owner.state)
                    .finish();
                let type_ = builder.owner.create_simple_type(ns, Some(name), x)?;
                ui.types.push(UnionMetaType::new(type_));
            }

            if let Some(x) = &ty.annotation {
                if let Err(error) = x.extract_documentation_into(&mut builder.documentation) {
                    tracing::warn!("Unable to extract documentation for `xs:union`: {error}");
                }
            }

            Ok(())
        })
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_list(&mut self, ty: &List) -> Result<(), Error> {
        let mut type_ = None;

        if let Some(s) = &ty.item_type {
            type_ = Some(self.owner.parse_qname(s)?);
        }

        if let Some(x) = &ty.simple_type {
            let last_named_type = self.state.last_named_type(false).map(ToOwned::to_owned);
            let name = self
                .state
                .name_builder()
                .or(&x.name)
                .or_else(|| from_utf8(ty.item_type.as_ref()?.local_name()).ok())
                .or_else(|| {
                    let s = last_named_type?;
                    let s = s.as_str();
                    let s = s.strip_suffix("Type").unwrap_or(s);
                    let s = format!("{s}Item");

                    Some(Name::from(s))
                })
                .finish();
            let ns = self.owner.state.current_ns();
            type_ = Some(self.owner.create_simple_type(ns, Some(name), x)?);
        }

        if let Some(type_) = type_ {
            self.simple_content_builder(|builder| {
                let ti = get_or_init_any!(builder, Reference, ReferenceMeta::new(type_.clone()));
                ti.type_ = type_;
                ti.min_occurs = 0;
                ti.max_occurs = MaxOccurs::Unbounded;

                Ok(())
            })?;
        }

        if let Some(x) = &ty.annotation {
            if let Err(error) = x.extract_documentation_into(&mut self.documentation) {
                tracing::warn!("Unable to extract documentation for `xs:list`: {error}");
            }
        }

        Ok(())
    }

    fn apply_annotation(&mut self, ty: &Annotation) {
        if let Err(error) = ty.extract_documentation_into(&mut self.documentation) {
            tracing::warn!("Unable to extract documentation from annotation: {error}");
        }
    }

    fn copy_base_type(&mut self, base: &Ident, mode: UpdateMode) -> Result<(), Error> {
        let mut simple_base_ident = None;
        let base = match (self.type_mode, self.content_mode) {
            (TypeMode::Simple, ContentMode::Simple) => {
                self.fixed = false;

                simple_base_ident = Some(base.clone());

                self.owner.get_simple_type_variant(base)?
            }
            (TypeMode::Complex, ContentMode::Simple) => {
                match self.owner.get_simple_type_variant(base) {
                    Ok(ty) => {
                        self.fixed = false;

                        simple_base_ident = Some(base.clone());

                        ty
                    }
                    Err(Error::UnknownType(_)) => {
                        self.fixed = true;

                        self.owner.get_complex_type_variant(base)?
                    }
                    Err(error) => Err(error)?,
                }
            }
            (TypeMode::Complex, ContentMode::Complex) => {
                self.owner.get_complex_type_variant(base)?
            }
            (_, _) => crate::unreachable!("Unset or invalid combination!"),
        };

        let mut base = base.clone();

        match (self.content_mode, &mut base) {
            (ContentMode::Simple, MetaTypeVariant::Enumeration(ei)) => ei.variants.clear(),
            (ContentMode::Complex, MetaTypeVariant::ComplexType(ci)) => {
                if let Some(content_ident) = &ci.content {
                    let mut content_type = self
                        .owner
                        .state
                        .types
                        .items
                        .get(content_ident)
                        .ok_or_else(|| Error::UnknownType(content_ident.clone()))?
                        .clone();

                    match (&mut content_type.variant, mode) {
                        (MetaTypeVariant::All(si) | MetaTypeVariant::Choice(si) | MetaTypeVariant::Sequence(si), UpdateMode::Restriction) => {
                            si.elements.retain(|element| element.min_occurs > 0);
                        }
                        (_, UpdateMode::Extension) => (),
                        (_, _) => tracing::warn!("Complex type does not has `All`, `Choice` or `Sequence` as content: {content_ident:#?} => {content_type:#?}!"),
                    }

                    let content_name = self.state.make_content_name();
                    let content_ident = Ident::new(content_name).with_ns(self.state.current_ns());

                    self.state
                        .add_type(content_ident.clone(), content_type, false)?;

                    ci.content = Some(content_ident);
                }
            }
            (_, _) => (),
        }

        match (simple_base_ident, self.type_mode, self.content_mode) {
            (Some(_), TypeMode::Simple, _) => self.variant = Some(base),
            (Some(base_ident), TypeMode::Complex, ContentMode::Simple) => {
                let ci = get_or_init_any!(self, ComplexType);
                ci.content.get_or_insert(base_ident);
            }
            (None, TypeMode::Complex, ContentMode::Simple | ContentMode::Complex) => {
                self.variant = Some(base);
            }
            (_, _, _) => crate::unreachable!("Unset or invalid combination!"),
        }

        Ok(())
    }

    fn walk_substitution_groups<F>(&mut self, groups: &QNameList, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&mut Self, &Ident) -> Result<(), Error>,
    {
        fn inner<'x, 'y, 'z, F>(
            builder: &mut VariantBuilder<'x, 'y, 'z>,
            groups: &QNameList,
            f: &mut F,
        ) -> Result<(), Error>
        where
            F: FnMut(&mut VariantBuilder<'x, 'y, 'z>, &Ident) -> Result<(), Error>,
        {
            for head in &groups.0 {
                let ident = builder.parse_qname(head)?.with_type(IdentType::Element);

                f(builder, &ident)?;

                if let Some(groups) = builder
                    .find_element(ident)
                    .and_then(|x| x.substitution_group.as_ref())
                {
                    inner(builder, groups, f)?;
                }
            }

            Ok(())
        }

        inner(self, groups, &mut f)
    }

    fn simple_content_builder<F>(&mut self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut VariantBuilder<'_, 'schema, 'state>) -> Result<(), Error>,
    {
        match (self.type_mode, self.content_mode) {
            (TypeMode::Simple, _) => f(self)?,
            (TypeMode::Complex, ContentMode::Simple) => {
                let ci = get_or_init_any!(self, ComplexType);

                let Some(mut content_ident) = ci.content.clone() else {
                    crate::unreachable!(
                        "Complex type does not have a simple content identifier: {:?}",
                        &self.variant
                    );
                };

                let content = self.owner.get_simple_type_variant(&content_ident)?.clone();
                if !self.is_simple_content_unique {
                    self.is_simple_content_unique = true;
                    let content_name = self.owner.state.make_content_name();
                    content_ident = Ident::new(content_name).with_ns(self.owner.state.current_ns());

                    ci.content = Some(content_ident.clone());
                }

                let mut builder = VariantBuilder::new(&mut *self.owner);
                builder.variant = Some(content);
                builder.type_mode = TypeMode::Simple;
                builder.content_mode = ContentMode::Simple;

                f(&mut builder)?;

                let content = builder.variant.unwrap();
                let content = MetaType::new(content);

                match self.owner.state.types.items.entry(content_ident) {
                    Entry::Vacant(e) => {
                        e.insert(content);
                    }
                    Entry::Occupied(e) => {
                        *e.into_mut() = content;
                    }
                }
            }
            (TypeMode::Complex, ContentMode::Complex) => {
                crate::unreachable!("Complex type with complex content tried to access simple content builder: {:?}", &self.variant);
            }
            (_, _) => crate::unreachable!("Unset or invalid combination!"),
        }

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    #[instrument(err, level = "trace", skip(self, f))]
    fn complex_content_builder<F>(
        &mut self,
        field_name: Name,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        type_ident: Ident,
        complex_content_type: ComplexContentType,
        f: F,
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut VariantBuilder<'_, 'schema, 'state>) -> Result<(), Error>,
    {
        enum UpdateContentMode {
            Unknown,
            Update,
            Append,
        }

        enum Output {
            Type(MetaType),
            Cached(Ident),
        }

        let group_ident = self.state.type_stack.last().and_then(|x| {
            if let StackEntry::GroupRef(x) = x {
                Some(x.clone())
            } else {
                None
            }
        });
        let mut cached_ident = group_ident
            .clone()
            .and_then(|x| self.state.group_cache()?.get(&x))
            .cloned();
        let is_cached = cached_ident.is_some();

        let mut variant = None;
        let mut update_content_mode = UpdateContentMode::Unknown;

        if let Some(MetaTypeVariant::ComplexType(ci)) = &self.variant {
            if let Some(content_ident) = &ci.content {
                let content_ty = self
                    .owner
                    .state
                    .types
                    .items
                    .get(content_ident)
                    .ok_or_else(|| Error::UnknownType(content_ident.clone()))?;

                match (complex_content_type, &content_ty.variant) {
                    (ComplexContentType::All, MetaTypeVariant::All(_))
                    | (ComplexContentType::Choice, MetaTypeVariant::Choice(_))
                    | (ComplexContentType::Sequence, MetaTypeVariant::Sequence(_)) => {
                        variant = Some(content_ty.variant.clone());
                        cached_ident = None;
                        update_content_mode = UpdateContentMode::Update;
                    }
                    (_, _) => update_content_mode = UpdateContentMode::Append,
                }
            }
        }

        let output = if let Some(cached_ident) = cached_ident {
            Output::Cached(cached_ident)
        } else {
            let variant = variant.unwrap_or_else(|| match complex_content_type {
                ComplexContentType::All => MetaTypeVariant::All(Default::default()),
                ComplexContentType::Choice => MetaTypeVariant::Choice(Default::default()),
                ComplexContentType::Sequence => MetaTypeVariant::Sequence(Default::default()),
            });

            let mut builder = VariantBuilder::new(&mut *self.owner);
            builder.type_mode = self.type_mode;
            builder.variant = Some(variant);

            f(&mut builder)?;

            let ty = builder.finish()?;

            let (MetaTypeVariant::All(si)
            | MetaTypeVariant::Choice(si)
            | MetaTypeVariant::Sequence(si)) = &ty.variant
            else {
                return Err(Error::ExpectedGroupType);
            };

            if si.elements.is_empty() {
                return Ok(());
            }

            Output::Type(ty)
        };

        let ns = self.state.current_ns();

        match &mut self.variant {
            Some(MetaTypeVariant::ComplexType(ci)) => match update_content_mode {
                UpdateContentMode::Unknown => {
                    let content_ident = match output {
                        Output::Cached(ident) => ident,
                        Output::Type(ty) => {
                            self.owner.state.add_type(type_ident.clone(), ty, false)?;

                            type_ident.clone()
                        }
                    };

                    ci.content = Some(content_ident);
                    ci.min_occurs = ci.min_occurs.min(min_occurs);
                    ci.max_occurs = ci.max_occurs.max(max_occurs);
                }
                UpdateContentMode::Update => {
                    let Output::Type(ty) = output else {
                        unreachable!();
                    };

                    let content_ident = ci.content.as_ref().unwrap();

                    self.owner
                        .state
                        .types
                        .items
                        .insert(content_ident.clone(), ty);

                    ci.min_occurs = ci.min_occurs.min(min_occurs);
                    ci.max_occurs = ci.max_occurs.max(max_occurs);
                }
                UpdateContentMode::Append => {
                    let element_ident = match output {
                        Output::Cached(ident) => ident,
                        Output::Type(ty) => {
                            self.owner.state.add_type(type_ident.clone(), ty, false)?;

                            type_ident.clone()
                        }
                    };

                    let content_ident = ci.content.as_ref().unwrap();
                    let content_type = self.owner.state.types.items.get_mut(content_ident).unwrap();
                    let content_variant = &mut content_type.variant;

                    let (MetaTypeVariant::All(si)
                    | MetaTypeVariant::Choice(si)
                    | MetaTypeVariant::Sequence(si)) = content_variant
                    else {
                        unreachable!();
                    };

                    let ident = Ident::new(field_name)
                        .with_ns(ns)
                        .with_type(IdentType::Group);
                    let element = si.elements.insert_checked(ident, |ident| {
                        ElementMeta::new(ident, element_ident, ElementMode::Group)
                    });

                    element.min_occurs = element.min_occurs.min(min_occurs);
                    element.max_occurs = element.max_occurs.max(max_occurs);
                }
            },
            Some(
                MetaTypeVariant::All(si)
                | MetaTypeVariant::Choice(si)
                | MetaTypeVariant::Sequence(si),
            ) => {
                let element_ident = match output {
                    Output::Cached(ident) => ident,
                    Output::Type(ty) => {
                        self.owner.state.add_type(type_ident.clone(), ty, false)?;

                        type_ident.clone()
                    }
                };

                let ident = Ident::new(field_name)
                    .with_ns(ns)
                    .with_type(IdentType::Group);
                let element = si.elements.insert_checked(ident, |ident| {
                    ElementMeta::new(ident, element_ident, ElementMode::Group)
                });

                element.min_occurs = element.min_occurs.min(min_occurs);
                element.max_occurs = element.max_occurs.max(max_occurs);
            }
            None => {
                let content_ident = match output {
                    Output::Cached(ident) => ident,
                    Output::Type(ty) => {
                        self.owner.state.add_type(type_ident.clone(), ty, false)?;

                        type_ident.clone()
                    }
                };

                let ci = get_or_init_any!(self, ComplexType);

                ci.content = Some(content_ident);
                ci.min_occurs = ci.min_occurs.min(min_occurs);
                ci.max_occurs = ci.max_occurs.max(max_occurs);
            }
            e => crate::unreachable!("{:?}", e),
        }

        if !is_cached {
            if let (Some(cache), Some(group_ident)) = (self.state.group_cache_mut(), group_ident) {
                cache.insert(group_ident, type_ident);
            }
        }

        Ok(())
    }

    fn make_field_name_and_type(&mut self, ty: &GroupType) -> (Name, Ident) {
        let name = self
            .state
            .name_builder()
            .or(ty.name.clone())
            .or_else(|| self.state.named_group())
            .remove_suffix("Type")
            .remove_suffix("Content")
            .or_else(|| {
                self.state
                    .name_builder()
                    .generate_id()
                    .shared_name("Content")
            });
        let field_name = name.clone().finish();
        let type_name = self
            .state
            .name_builder()
            .auto_extend(false, true, self.state)
            .remove_suffix("Type")
            .remove_suffix("Content")
            .or(name)
            .finish();
        let type_ = Ident::new(type_name).with_ns(self.state.current_ns());

        (field_name, type_)
    }
}

impl<'schema, 'state> Deref for VariantBuilder<'_, 'schema, 'state> {
    type Target = SchemaInterpreter<'schema, 'state>;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}

impl DerefMut for VariantBuilder<'_, '_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.owner
    }
}

/* ElementsMeta */

impl ElementsMeta {
    fn insert_checked<F>(&mut self, ident: Ident, f: F) -> &mut ElementMeta
    where
        F: FnOnce(Ident) -> ElementMeta,
    {
        let vec = &mut **self;

        let Some(index) = vec.iter().position(|x| x.ident == ident) else {
            vec.push(f(ident));

            return vec.last_mut().unwrap();
        };

        if vec[index].display_name.is_none() {
            vec[index].display_name = Some(format!("{}_1", ident.name.as_str()));
        }

        let mut next = 1;
        loop {
            next += next;

            let mut new_ident = ident.clone();
            *new_ident.name.as_mut() = format!("{}_{next}", ident.name.as_str());

            if !vec.iter().any(|x| x.ident == new_ident) {
                vec.push(f(new_ident));

                return vec.last_mut().unwrap();
            }
        }
    }
}

/* Update */

trait Update<T> {
    fn update(&mut self, other: &T);
}

impl<T: Clone> Update<Option<T>> for T {
    fn update(&mut self, other: &Option<T>) {
        if let Some(value) = other {
            *self = value.clone();
        }
    }
}

impl<T: Clone> Update<Option<T>> for Option<T> {
    fn update(&mut self, other: &Option<T>) {
        if let Some(value) = other {
            *self = Some(value.clone());
        }
    }
}

impl Update<GroupType> for ElementMeta {
    fn update(&mut self, other: &GroupType) {
        self.min_occurs = other.min_occurs;
        self.max_occurs = other.max_occurs;
    }
}

impl Update<ElementType> for ElementMeta {
    fn update(&mut self, other: &ElementType) {
        self.min_occurs = other.min_occurs;
        self.max_occurs = other.max_occurs;
    }
}

impl Update<AttributeType> for AttributeMeta {
    fn update(&mut self, other: &AttributeType) {
        self.use_ = other.use_.clone();
        self.default.update(&other.default);
    }
}

/* Patch */

trait Patch<T>: Clone {
    fn patch(&self, other: &T) -> Cow<'_, Self>;
}

impl Patch<GroupType> for GroupType {
    fn patch(&self, other: &GroupType) -> Cow<'_, Self> {
        let mut ret = self.clone();

        ret.min_occurs = other.min_occurs;
        ret.max_occurs = other.max_occurs;

        Cow::Owned(ret)
    }
}
