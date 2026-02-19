use std::borrow::Cow;
use std::mem::swap;
use std::ops::Bound;
use std::str::from_utf8;

use tracing::instrument;

use crate::models::meta::{
    AnyAttributeMeta, AnyMeta, AttributeMeta, Base, DerivedTypeMeta, DynamicMeta, ElementMeta,
    ElementMetaVariant, ElementMode, ElementsMeta, EnumerationMetaVariant, MetaType,
    MetaTypeVariant, ReferenceMeta, SimpleMeta, UnionMetaType, WhiteSpace,
};
use crate::models::schema::xs::{
    Annotation, Any, AnyAttribute, AttributeGroupType, AttributeType, ComplexBaseType,
    ComplexContent, ElementType, ExtensionType, Facet, FacetType, FormChoiceType, GroupType, List,
    QNameList, Restriction, RestrictionType, SimpleBaseType, SimpleContent, Union, Use,
};
use crate::models::schema::{MaxOccurs, MinOccurs};
use crate::models::{AttributeIdent, ElementIdent, EnumerationIdent, IdentType, Name, TypeIdent};
use crate::pipeline::interpreter::state::NodeDependencyKey;
use crate::traits::{NameBuilder, NameBuilderExt as _, VecHelper};

use super::super::{Error, Node};
use super::{NameBuilderExt as _, StackEntry, TypeProcessor, Update};

pub(super) struct VariantProcessor<'a, 'state, 'schema> {
    /// Reference to the owning type processor
    owner: &'a mut TypeProcessor<'state, 'schema>,

    /// Type variant that is constructed by the processor
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

/// Initialize the type of a `$processor` to any type `$variant`.
macro_rules! init_any {
    ($processor:expr, $variant:ident) => {
        init_any!($processor, $variant, Default::default(), true)
    };
    ($processor:expr, $variant:ident, $value:expr, $fixed:expr) => {{
        $processor.variant = Some(MetaTypeVariant::$variant($value));
        $processor.fixed = $fixed;

        let MetaTypeVariant::$variant(ret) = $processor.variant.as_mut().unwrap() else {
            crate::unreachable!();
        };

        ret
    }};
}

/// Get the type `$variant` of a `$processor` or set the type variant if unset.
macro_rules! get_or_init_any {
    ($processor:expr, $variant:ident) => {
        get_or_init_any!($processor, $variant, Default::default())
    };
    ($processor:expr, $variant:ident, $default:expr) => {
        match &mut $processor.variant {
            None => init_any!($processor, $variant, $default, true),
            Some(MetaTypeVariant::$variant(ret)) => ret,
            _ if !$processor.fixed => init_any!($processor, $variant, $default, true),
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    };
}

/// Get the `GroupMeta` from any possible type or initialize the required variant.
macro_rules! get_or_init_type {
    ($processor:expr, $variant:ident) => {
        get_or_init_type!($processor, $variant, Default::default())
    };
    ($processor:expr, $variant:ident, $default:expr) => {
        match &mut $processor.variant {
            None => init_any!($processor, $variant, $default, true),
            Some(
                MetaTypeVariant::All(si)
                | MetaTypeVariant::Choice(si)
                | MetaTypeVariant::Sequence(si),
            ) => si,
            _ if !$processor.fixed => init_any!($processor, $variant, $default, true),
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    };
}

macro_rules! impl_with_node {
    ($fn:ident, $variant:ident, $ty:ty) => {
        pub(super) fn $fn<F, R>(&mut self, ident: TypeIdent, f: F) -> R
        where
            F: FnOnce(&mut Self, &'schema $ty, TypeIdent) -> R,
        {
            let entry = self.owner.get_node_entry(&ident);

            self.owner.push_stack(StackEntry::NodeEntry { entry });

            let Node::$variant(x) = entry.node else {
                unreachable!("Unexpected node type: {ident} => {:?}", entry.node);
            };

            let ret = f(self, x, ident);

            self.owner.pop_stack();

            ret
        }
    };
}

impl<'a, 'state, 'schema> VariantProcessor<'a, 'state, 'schema> {
    pub(super) fn new(owner: &'a mut TypeProcessor<'state, 'schema>) -> Self {
        Self {
            owner,
            variant: None,
            fixed: false,
            type_mode: TypeMode::Unknown,
            content_mode: ContentMode::Unknown,
            documentation: Vec::new(),
            is_simple_content_unique: false,
        }
    }

    pub(super) fn finish(self) -> Result<MetaType, Error> {
        let Self { variant, owner, .. } = self;
        let variant = variant.ok_or(Error::NoType)?;

        let mut type_ = MetaType::new(variant);
        type_.form = Some(owner.current_schema().element_form_default);
        type_.schema = Some(owner.current_schema_id());
        type_.documentation = self.documentation;

        Ok(type_)
    }

    fn finish_mut(&mut self) -> Result<MetaType, Error> {
        let variant = self.variant.take().ok_or(Error::NoType)?;

        let mut type_ = MetaType::new(variant);
        type_.form = Some(self.owner.current_schema().element_form_default);
        type_.schema = Some(self.owner.current_schema_id());
        swap(&mut type_.documentation, &mut self.documentation);

        self.reset();

        Ok(type_)
    }

    fn reset(&mut self) {
        self.variant = None;
        self.fixed = false;
        self.type_mode = TypeMode::Unknown;
        self.content_mode = ContentMode::Unknown;
        self.is_simple_content_unique = false;

        self.documentation.clear();
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_element(
        &mut self,
        ty: &'schema ElementType,
        extract_docs: bool,
    ) -> Result<(), Error> {
        use crate::models::schema::xs::ElementTypeContent as C;

        if let Some(type_) = &ty.type_ {
            let type_ = self.owner.resolve_type_ident(type_, IdentType::Type)?;

            init_any!(self, Reference, ReferenceMeta::new(type_), true);
        } else if !ty.content.is_empty() {
            let type_ = self
                .owner
                .current_ident()
                .clone()
                .with_type(IdentType::ElementType);

            let mut has_content = false;
            let ret = self.owner.create_type(type_.clone(), |processor| {
                for c in &ty.content {
                    match c {
                        C::Annotation(_)
                        | C::Key(_)
                        | C::Alternative(_)
                        | C::Unique(_)
                        | C::Keyref(_) => {}
                        C::SimpleType(x) => {
                            has_content = true;
                            processor.apply_simple_type(x)?;
                        }
                        C::ComplexType(x) => {
                            has_content = true;
                            processor.apply_complex_type(x)?;
                        }
                    }
                }

                Ok(())
            });

            if has_content {
                ret?;
                init_any!(self, Reference, ReferenceMeta::new(type_), true);
            } else {
                // No actual type content found, default to `xs:anyType`
                let ident = self.owner.resolve_xs_type(Name::ANY_TYPE)?;

                init_any!(self, Reference, ReferenceMeta::new(ident), true);
            }
        } else {
            // No content, default to `xs:anyType`
            let ident = self.owner.resolve_xs_type(Name::ANY_TYPE)?;

            init_any!(self, Reference, ReferenceMeta::new(ident), true);
        }

        if extract_docs {
            for content in &ty.content {
                if let C::Annotation(ty) = content {
                    self.apply_annotation(ty);
                }
            }
        }

        if ty.nillable.unwrap_or_default() {
            let mut ident = self.owner.current_ident().clone();
            ident.type_ = IdentType::NillableContent;

            let type_ = self.finish_mut()?;
            self.owner.add_type(ident.clone(), type_, false)?;

            let meta = init_any!(self, Reference, ReferenceMeta::new(ident), true);
            meta.nillable = true;
        }

        if let Some(substitution_group) = &ty.substitution_group {
            let mut ident = self.owner.current_ident().clone();
            ident.type_ = IdentType::DynamicElement;

            let type_ = self.finish_mut()?;
            self.owner.add_type(ident.clone(), type_, false)?;

            init_any!(self, Reference, ReferenceMeta::new(ident), true);

            self.walk_substitution_groups(substitution_group, |processor, base_ident| {
                let ident = processor.owner.current_ident().clone();
                let base_ty = processor
                    .owner
                    .get_substitution_group_element_mut(base_ident);

                if let MetaTypeVariant::Reference(ti) = &mut base_ty.variant {
                    base_ty.variant = MetaTypeVariant::Dynamic(DynamicMeta {
                        type_: Some(ti.type_.clone()),
                        derived_types: vec![DerivedTypeMeta::new(ti.type_.clone())],
                    });
                }

                let MetaTypeVariant::Dynamic(ai) = &mut base_ty.variant else {
                    return Err(Error::ExpectedDynamicElement(base_ident.clone()));
                };

                ai.derived_types.push(DerivedTypeMeta::new(ident));

                Ok(())
            })?;
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

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_attribute(&mut self, ty: &'schema AttributeType) -> Result<(), Error> {
        if let Some(type_) = &ty.type_ {
            let type_ = self.owner.resolve_type_ident(type_, IdentType::Type)?;

            init_any!(self, Reference, ReferenceMeta::new(type_), false);
        } else if let Some(x) = &ty.simple_type {
            self.apply_simple_type(x)?;
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_simple_type(&mut self, ty: &'schema SimpleBaseType) -> Result<(), Error> {
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
    pub(super) fn apply_complex_type(&mut self, ty: &'schema ComplexBaseType) -> Result<(), Error> {
        use crate::models::schema::xs::ComplexBaseTypeContent as C;

        self.type_mode = TypeMode::Complex;
        self.content_mode = ContentMode::Complex;

        let ci = get_or_init_any!(self, ComplexType);
        if let Some(mixed) = ty.mixed {
            ci.is_mixed = mixed;
            self.owner.push_stack(StackEntry::Mixed { mixed });
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
                C::All(x) => self.apply_all(x, x.min_occurs, x.max_occurs),
                C::Choice(x) => self.apply_choice(x, x.min_occurs, x.max_occurs),
                C::Sequence(x) => self.apply_sequence(x, x.min_occurs, x.max_occurs),
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
            self.owner.pop_stack();
        }

        ret
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_type_restriction(&mut self, ty: &'schema Restriction) -> Result<(), Error> {
        use crate::models::schema::xs::RestrictionContent as C;

        let base = ty
            .base
            .as_ref()
            .map(|base| {
                let base = self.owner.resolve_type_ident(base, IdentType::Type)?;

                self.copy_base_type(base, UpdateMode::Restriction)
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
                Some(
                    MetaTypeVariant::BuildIn(_)
                    | MetaTypeVariant::Reference(_)
                    | MetaTypeVariant::SimpleType(_),
                ) => (),
                Some(MetaTypeVariant::Union(e)) => e.base = Base::Extension(base),
                Some(MetaTypeVariant::Enumeration(e)) => e.base = Base::Extension(base),
                Some(MetaTypeVariant::ComplexType(e)) => e.base = Base::Extension(base),
                e => crate::unreachable!("Unexpected type: {e:#?}"),
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_content(&mut self, ty: &'schema SimpleContent) -> Result<(), Error> {
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
    fn apply_complex_content(&mut self, ty: &'schema ComplexContent) -> Result<(), Error> {
        use crate::models::schema::xs::ComplexContentContent as C;

        let ci = get_or_init_any!(self, ComplexType);
        if let Some(mixed) = ty.mixed {
            ci.is_mixed = mixed;
            self.owner.push_stack(StackEntry::Mixed { mixed });
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
            self.owner.pop_stack();
        }

        ret
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_content_extension(&mut self, ty: &'schema ExtensionType) -> Result<(), Error> {
        let base = self.owner.resolve_type_ident(&ty.base, IdentType::Type)?;
        let base = self.copy_base_type(base, UpdateMode::Extension)?;

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
    fn apply_simple_content_restriction(
        &mut self,
        ty: &'schema RestrictionType,
    ) -> Result<(), Error> {
        let base = self.owner.resolve_type_ident(&ty.base, IdentType::Type)?;
        let base = self.copy_base_type(base, UpdateMode::Restriction)?;

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
    fn apply_complex_content_extension(&mut self, ty: &'schema ExtensionType) -> Result<(), Error> {
        let base = self.owner.resolve_type_ident(&ty.base, IdentType::Type)?;
        let base = self.copy_base_type(base, UpdateMode::Extension)?;

        self.apply_extension(ty)?;

        let ci = get_or_init_any!(self, ComplexType);
        ci.base = Base::Extension(base);

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_complex_content_restriction(
        &mut self,
        ty: &'schema RestrictionType,
    ) -> Result<(), Error> {
        let base = self.owner.resolve_type_ident(&ty.base, IdentType::Type)?;
        let base = self.copy_base_type(base, UpdateMode::Restriction)?;

        self.apply_restriction(ty)?;

        let ci = get_or_init_any!(self, ComplexType);
        ci.base = Base::Restriction(base);

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_extension(&mut self, ty: &'schema ExtensionType) -> Result<(), Error> {
        use crate::models::schema::xs::ExtensionTypeContent as C;

        for c in &ty.content {
            match c {
                C::OpenContent(_) | C::Assert(_) => (),
                C::Annotation(x) => self.apply_annotation(x),
                C::All(x) => self.apply_all(x, x.min_occurs, x.max_occurs)?,
                C::Choice(x) => self.apply_choice(x, x.min_occurs, x.max_occurs)?,
                C::Sequence(x) => self.apply_sequence(x, x.min_occurs, x.max_occurs)?,
                C::Group(x) => self.apply_group_ref(x)?,
                C::Attribute(x) => self.apply_attribute_ref(x)?,
                C::AnyAttribute(x) => self.apply_any_attribute(x)?,
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_restriction(&mut self, ty: &'schema RestrictionType) -> Result<(), Error> {
        use crate::models::schema::xs::RestrictionTypeContent as C;

        for c in &ty.content {
            match c {
                C::OpenContent(_) | C::Assert(_) => (),
                C::Annotation(x) => self.apply_annotation(x),
                C::All(x) => self.apply_all(x, x.min_occurs, x.max_occurs)?,
                C::Choice(x) => self.apply_choice(x, x.min_occurs, x.max_occurs)?,
                C::Sequence(x) => self.apply_sequence(x, x.min_occurs, x.max_occurs)?,
                C::Attribute(x) => self.apply_attribute_ref(x)?,
                C::AnyAttribute(x) => self.apply_any_attribute(x)?,
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x)?,
                C::Facet(x) => self.apply_facet(x)?,
                C::Group(x) => self.apply_group_ref(x)?,
                C::SimpleType(x) => self.apply_simple_type(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_all(
        &mut self,
        ty: &'schema GroupType,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
    ) -> Result<(), Error> {
        let (field_name, type_ident) = self.make_field_name_and_type(ty);

        self.complex_content_builder(
            field_name,
            min_occurs,
            max_occurs,
            type_ident,
            ComplexContentType::All,
            |processor| processor.apply_group(ty),
        )?;

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_choice(
        &mut self,
        ty: &'schema GroupType,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
    ) -> Result<(), Error> {
        let (field_name, type_ident) = self.make_field_name_and_type(ty);

        self.complex_content_builder(
            field_name,
            min_occurs,
            max_occurs,
            type_ident,
            ComplexContentType::Choice,
            |processor| processor.apply_group(ty),
        )?;

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_sequence(
        &mut self,
        ty: &'schema GroupType,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
    ) -> Result<(), Error> {
        let (field_name, type_ident) = self.make_field_name_and_type(ty);

        self.complex_content_builder(
            field_name,
            min_occurs,
            max_occurs,
            type_ident,
            ComplexContentType::Sequence,
            |processor| processor.apply_group(ty),
        )?;

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_group(&mut self, ty: &'schema GroupType) -> Result<(), Error> {
        use crate::models::schema::xs::GroupTypeContent as C;

        let is_mixed = self.owner.is_mixed();
        if let Some(
            MetaTypeVariant::All(gi) | MetaTypeVariant::Choice(gi) | MetaTypeVariant::Sequence(gi),
        ) = &mut self.variant
        {
            gi.is_mixed = is_mixed;
        }

        self.owner.push_stack(StackEntry::Group);

        let mut ret = Ok(());

        for c in &ty.content {
            ret = match c {
                C::Annotation(x) => {
                    self.apply_annotation(x);
                    Ok(())
                }
                C::All(x) => self.apply_all(x, x.min_occurs, x.max_occurs),
                C::Choice(x) => self.apply_choice(x, x.min_occurs, x.max_occurs),
                C::Sequence(x) => self.apply_sequence(x, x.min_occurs, x.max_occurs),
                C::Any(x) => self.apply_any(x),
                C::Group(x) => self.apply_group_ref(x),
                C::Element(x) => self.apply_element_ref(x),
            };

            if ret.is_err() {
                break;
            }
        }

        self.owner.pop_stack();

        ret
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_attribute_group(&mut self, ty: &'schema AttributeGroupType) -> Result<(), Error> {
        use crate::models::schema::xs::AttributeGroupTypeContent as C;

        for c in &ty.content {
            match c {
                C::Annotation(x) => self.apply_annotation(x),
                C::Attribute(x) => self.apply_attribute_ref(x)?,
                C::AnyAttribute(x) => self.apply_any_attribute(x)?,
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_element_ref(&mut self, ty: &'schema ElementType) -> Result<(), Error> {
        use crate::models::schema::xs::ElementTypeContent as C;

        let element = match ty {
            ElementType {
                ref_: Some(ref_),
                name,
                ..
            } => {
                let type_ = self.owner.resolve_type_ident(ref_, IdentType::Element)?;
                let name = self.owner.name_builder().or(name).or(&type_.name).finish();
                let ident = ElementIdent::new(name).with_ns(type_.ns);
                let form = ty
                    .form
                    .unwrap_or_else(|| self.owner.current_schema().element_form_default);

                let ci = get_or_init_type!(self, Sequence);
                let element = ci.elements.find_or_insert(ident, |ident| {
                    ElementMeta::new(ident, type_, ElementMode::Element, form)
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
                let field_name = self.owner.name_builder().or(&ty.name).finish();
                let field_ident = ElementIdent::new(field_name).with_ns(self.owner.current_ns_id());

                let type_ = if let Some(type_) = &ty.type_ {
                    self.owner.resolve_type_ident(type_, IdentType::Type)?
                } else {
                    self.owner
                        .resolve_type_ident_with_key(&NodeDependencyKey::InlineElement(ty))
                };
                let form = ty
                    .form
                    .unwrap_or_else(|| self.owner.current_schema().element_form_default);

                let ci = get_or_init_type!(self, Sequence);
                let element = ci.elements.find_or_insert(field_ident, |ident| {
                    ElementMeta::new(ident, type_, ElementMode::Element, form)
                });

                crate::assert!(matches!(
                    element.variant,
                    ElementMetaVariant::Type {
                        mode: ElementMode::Element,
                        ..
                    }
                ));

                element
            }
        };

        element.update(ty);

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
    fn apply_attribute_ref(&mut self, ty: &'schema AttributeType) -> Result<(), Error> {
        let attribute = match ty {
            AttributeType {
                name: Some(name),
                type_: Some(type_),
                ..
            } => {
                let type_ = self.owner.resolve_type_ident(type_, IdentType::Type)?;
                let name = Name::from(name.clone());
                let ident = AttributeIdent::new(name).with_ns(self.owner.current_ns_id());
                let form = ty
                    .form
                    .unwrap_or_else(|| self.owner.current_schema().attribute_form_default);

                get_or_init_any!(self, ComplexType)
                    .attributes
                    .find_or_insert(ident, |ident| AttributeMeta::new(ident, type_, form))
            }
            AttributeType {
                ref_: Some(ref_),
                name,
                ..
            } => {
                let type_ = self.owner.resolve_type_ident(ref_, IdentType::Attribute)?;
                let name = self.owner.name_builder().or(name).or(&type_.name).finish();
                let ident = AttributeIdent::new(name).with_ns(type_.ns);
                let form = ty
                    .form
                    .unwrap_or_else(|| self.owner.current_schema().attribute_form_default);

                get_or_init_any!(self, ComplexType)
                    .attributes
                    .find_or_insert(ident, |ident| AttributeMeta::new(ident, type_, form))
            }
            AttributeType {
                name: Some(name),
                simple_type,
                ..
            } => {
                let type_ = simple_type.as_ref().map(|x| {
                    self.owner
                        .resolve_type_ident_with_key(&NodeDependencyKey::InlineSimpleType(x))
                });

                let name = Name::from(name.clone());
                let ident = AttributeIdent::new(name).with_ns(self.owner.current_ns_id());
                let form = ty
                    .form
                    .unwrap_or_else(|| self.owner.current_schema().attribute_form_default);

                get_or_init_any!(self, ComplexType)
                    .attributes
                    .find_or_insert(ident, |ident| {
                        AttributeMeta::new(ident, type_.unwrap_or(TypeIdent::STRING), form)
                    })
            }
            e => return Err(Error::InvalidAttributeReference(Box::new(e.clone()))),
        };

        attribute.update(ty);

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
    fn apply_group_ref(&mut self, ty: &'schema GroupType) -> Result<(), Error> {
        use crate::models::schema::xs::GroupTypeContent as C;

        let ref_ = ty.ref_.as_ref().ok_or(Error::GroupMissingRef)?;
        let prefix = ref_.prefix();
        let ref_ = self.owner.resolve_type_ident(ref_, IdentType::Group)?;

        self.with_group_node(ref_, move |processor, group, ref_| {
            let mut ret = Ok(());

            let prefix = if ref_.ns == processor.owner.current_ns_id() {
                None
            } else {
                prefix
                    .and_then(|prefix| from_utf8(prefix).ok())
                    .map(ToOwned::to_owned)
            };

            processor.owner.push_stack(StackEntry::GroupRef {
                ident: ref_,
                name: prefix,
            });

            for c in &group.content {
                ret = match c {
                    C::Annotation(_) => Ok(()),
                    C::Any(x) => processor.apply_any(x),
                    C::All(x) => processor.apply_all(x, ty.min_occurs, ty.max_occurs),
                    C::Choice(x) => processor.apply_choice(x, ty.min_occurs, ty.max_occurs),
                    C::Sequence(x) => processor.apply_sequence(x, ty.min_occurs, ty.max_occurs),
                    C::Group(x) => processor.apply_group_ref(x),
                    C::Element(x) => processor.apply_element_ref(x),
                };

                if ret.is_err() {
                    break;
                }
            }

            processor.owner.pop_stack();

            ret
        })
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_attribute_group_ref(&mut self, ty: &'schema AttributeGroupType) -> Result<(), Error> {
        let ref_ = ty.ref_.as_ref().ok_or(Error::AttributeGroupMissingRef)?;
        let ref_ = self
            .owner
            .resolve_type_ident(ref_, IdentType::AttributeGroup)?;

        self.with_attribute_group_node(ref_, |processor, group, _ref| {
            processor.apply_attribute_group(group)
        })
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_any(&mut self, ty: &'schema Any) -> Result<(), Error> {
        let name = self
            .owner
            .name_builder()
            .shared_name("any")
            .or(&ty.id)
            .finish();
        let ident = ElementIdent::new(name);

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
    fn apply_any_attribute(&mut self, ty: &'schema AnyAttribute) -> Result<(), Error> {
        let name = self
            .owner
            .name_builder()
            .unique_name("any_attribute")
            .or(&ty.id)
            .finish();
        let ident = AttributeIdent::new(name);

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
    fn apply_facet(&mut self, ty: &'schema Facet) -> Result<(), Error> {
        match ty {
            Facet::Enumeration(x) => self.apply_enumeration(x)?,
            x @ (Facet::MinExclusive(_)
            | Facet::MinInclusive(_)
            | Facet::MaxExclusive(_)
            | Facet::MaxInclusive(_)
            | Facet::TotalDigits(_)
            | Facet::FractionDigits(_)
            | Facet::Length(_)
            | Facet::MinLength(_)
            | Facet::MaxLength(_)
            | Facet::WhiteSpace(_)
            | Facet::Pattern(_)) => self.apply_simple_type_facet(x)?,
            x => tracing::warn!("Unknown facet: {x:#?}"),
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_type_facet(&mut self, ty: &'schema Facet) -> Result<(), Error> {
        self.simple_content_builder(|processor| {
            let constrains = match &mut processor.variant {
                Some(MetaTypeVariant::SimpleType(x)) => &mut x.constrains,
                Some(MetaTypeVariant::Reference(x)) => {
                    let base = x.type_.clone();
                    let min = x.min_occurs;
                    let max = x.max_occurs;

                    let si = init_any!(processor, SimpleType, SimpleMeta::new(base), true);

                    if min != 1 {
                        si.is_list = true;
                        si.constrains.min_length = Some(min);
                    }

                    if max != MaxOccurs::Bounded(1) {
                        si.is_list = true;
                        if let MaxOccurs::Bounded(max) = max {
                            si.constrains.max_length = Some(max);
                        }
                    }

                    &mut si.constrains
                }
                Some(MetaTypeVariant::Enumeration(em)) => &mut em.constrains,
                Some(MetaTypeVariant::Union(um)) => &mut um.constrains,
                Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
                None => crate::unreachable!("Type variant is not set yet"),
            };

            match ty {
                Facet::MinExclusive(x) => constrains.range.start = Bound::Excluded(x.value.clone()),
                Facet::MinInclusive(x) => constrains.range.start = Bound::Included(x.value.clone()),
                Facet::MaxExclusive(x) => constrains.range.end = Bound::Excluded(x.value.clone()),
                Facet::MaxInclusive(x) => constrains.range.end = Bound::Included(x.value.clone()),
                Facet::TotalDigits(x) => {
                    constrains.total_digits = Some(
                        x.value
                            .parse()
                            .map_err(|_| Error::InvalidFacet(ty.clone()))?,
                    );
                }
                Facet::FractionDigits(x) => {
                    constrains.fraction_digits = Some(
                        x.value
                            .parse()
                            .map_err(|_| Error::InvalidFacet(ty.clone()))?,
                    );
                }
                Facet::Length(x) => {
                    let len = x
                        .value
                        .parse()
                        .map_err(|_| Error::InvalidFacet(ty.clone()))?;

                    constrains.min_length = Some(len);
                    constrains.max_length = Some(len);
                }
                Facet::MinLength(x) => {
                    constrains.min_length = Some(
                        x.value
                            .parse()
                            .map_err(|_| Error::InvalidFacet(ty.clone()))?,
                    );
                }
                Facet::MaxLength(x) => {
                    constrains.max_length = Some(
                        x.value
                            .parse()
                            .map_err(|_| Error::InvalidFacet(ty.clone()))?,
                    );
                }
                Facet::WhiteSpace(x) => {
                    constrains.whitespace = match x.value.to_ascii_lowercase().as_str() {
                        "preserve" => WhiteSpace::Preserve,
                        "replace" => WhiteSpace::Replace,
                        "collapse" => WhiteSpace::Collapse,
                        _ => return Err(Error::InvalidFacet(ty.clone())),
                    }
                }
                Facet::Pattern(x) => constrains.patterns.push(x.value.clone()),
                _ => crate::unreachable!("Not a valid facet for a simple type!"),
            }

            Ok(())
        })
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_enumeration(&mut self, ty: &'schema FacetType) -> Result<(), Error> {
        let name = Name::from(ty.value.trim().to_owned());
        let ident = EnumerationIdent::new(name).with_ns(self.owner.current_ns_id());

        self.simple_content_builder(|processor| {
            if let Some(MetaTypeVariant::SimpleType(sm)) = &processor.variant {
                let constrains = sm.constrains.clone();
                let em = init_any!(processor, Enumeration);
                em.constrains = constrains;
            }

            let ei = get_or_init_any!(processor, Enumeration);

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
    fn apply_union(&mut self, ty: &'schema Union) -> Result<(), Error> {
        self.simple_content_builder(|processor| {
            let ui = get_or_init_any!(processor, Union);

            if let Some(types) = &ty.member_types {
                for type_ in &types.0 {
                    let type_ = processor.owner.resolve_type_ident(type_, IdentType::Type)?;
                    ui.types.push(UnionMetaType::new(type_));
                }
            }

            for x in &ty.simple_type {
                let ident = processor
                    .owner
                    .resolve_type_ident_with_key(&NodeDependencyKey::InlineSimpleType(x));
                ui.types.push(UnionMetaType::new(ident));
            }

            if let Some(x) = &ty.annotation {
                if let Err(error) = x.extract_documentation_into(&mut processor.documentation) {
                    tracing::warn!("Unable to extract documentation for `xs:union`: {error}");
                }
            }

            Ok(())
        })
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_list(&mut self, ty: &'schema List) -> Result<(), Error> {
        let mut type_ = None;

        if let Some(s) = &ty.item_type {
            type_ = Some(self.owner.resolve_type_ident(s, IdentType::Type)?);
        }

        if let Some(x) = &ty.simple_type {
            let ident = self
                .owner
                .resolve_type_ident_with_key(&NodeDependencyKey::InlineSimpleType(&**x));

            type_ = Some(ident);
        }

        if let Some(type_) = type_ {
            self.simple_content_builder(|processor| {
                let ti = get_or_init_any!(processor, Reference, ReferenceMeta::new(type_.clone()));
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

    #[instrument(level = "trace", skip(self))]
    fn apply_annotation(&mut self, ty: &'schema Annotation) {
        if let Err(error) = ty.extract_documentation_into(&mut self.documentation) {
            tracing::warn!("Unable to extract documentation from annotation: {error}");
        }
    }

    #[instrument(err, level = "trace", skip(self))]
    fn copy_base_type(&mut self, base: TypeIdent, mode: UpdateMode) -> Result<TypeIdent, Error> {
        enum BaseIdent {
            Simple(TypeIdent),
            Complex(TypeIdent),
        }

        let (ident, base) = match (self.type_mode, self.content_mode) {
            (TypeMode::Simple, ContentMode::Simple) => {
                self.fixed = false;
                let ty = self
                    .owner
                    .get_simple_type_variant(&base)
                    .ok_or_else(|| Error::ExpectedSimpleType(base.clone()))?;

                (BaseIdent::Simple(base), ty)
            }
            (TypeMode::Complex, ContentMode::Simple) => {
                match self.owner.get_simple_type_variant(&base) {
                    Some(ty) => {
                        self.fixed = false;

                        (BaseIdent::Simple(base), ty)
                    }
                    None => {
                        self.fixed = true;
                        let ty = self
                            .owner
                            .get_complex_type_variant(&base)
                            .ok_or_else(|| Error::ExpectedComplexType(base.clone()))?;

                        (BaseIdent::Complex(base), Cow::Borrowed(ty))
                    }
                }
            }
            (TypeMode::Complex, ContentMode::Complex) => {
                let ty = self
                    .owner
                    .get_complex_type_variant(&base)
                    .ok_or_else(|| Error::ExpectedComplexType(base.clone()))?;

                (BaseIdent::Complex(base), Cow::Borrowed(ty))
            }
            (_, _) => crate::unreachable!("Unset or invalid combination!"),
        };

        let mut base = base.into_owned();

        match (self.content_mode, &mut base) {
            (ContentMode::Simple, MetaTypeVariant::Enumeration(ei)) => ei.variants.clear(),
            (ContentMode::Complex, MetaTypeVariant::ComplexType(ci)) => {
                if let Some(content_ident) = &ci.content {
                    let mut content_type = self.owner.get_type(content_ident).clone();

                    match (&mut content_type.variant, mode) {
                        (MetaTypeVariant::All(si) | MetaTypeVariant::Choice(si) | MetaTypeVariant::Sequence(si), UpdateMode::Restriction) => {
                            si.elements.retain(|element| element.min_occurs > 0);
                        }
                        (_, UpdateMode::Extension) => (),
                        (_, _) => tracing::warn!("Complex type does not has `All`, `Choice` or `Sequence` as content: {content_ident:#?} => {content_type:#?}!"),
                    }

                    let content_name = self.owner.make_content_name();
                    let content_ident = TypeIdent {
                        ns: self.owner.current_ns_id(),
                        schema: self.owner.current_schema_id(),
                        name: content_name,
                        type_: IdentType::Type,
                    };

                    self.owner
                        .add_type(content_ident.clone(), content_type, false)?;

                    ci.content = Some(content_ident);
                }
            }
            (_, _) => (),
        }

        match (ident, self.type_mode, self.content_mode) {
            (BaseIdent::Simple(ident), TypeMode::Simple, _) => {
                self.variant = Some(base);

                Ok(ident)
            }
            (BaseIdent::Simple(ident), TypeMode::Complex, ContentMode::Simple) => {
                let ci = get_or_init_any!(self, ComplexType);
                ci.content.get_or_insert_with(|| ident.clone());

                Ok(ident)
            }
            (
                BaseIdent::Complex(ident),
                TypeMode::Complex,
                ContentMode::Simple | ContentMode::Complex,
            ) => {
                if let MetaTypeVariant::ComplexType(ci) = &mut base {
                    ci.is_mixed = self.owner.is_mixed();
                }

                self.variant = Some(base);

                Ok(ident)
            }
            (_, _, _) => crate::unreachable!("Unset or invalid combination!"),
        }
    }

    #[instrument(err, level = "trace", skip(self, f))]
    fn walk_substitution_groups<F>(&mut self, groups: &QNameList, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&mut Self, &TypeIdent) -> Result<(), Error>,
    {
        fn inner<'x, 'y, 'z, F>(
            processor: &mut VariantProcessor<'x, 'y, 'z>,
            groups: &QNameList,
            f: &mut F,
        ) -> Result<(), Error>
        where
            F: FnMut(&mut VariantProcessor<'x, 'y, 'z>, &TypeIdent) -> Result<(), Error>,
        {
            for head in &groups.0 {
                let ident = processor
                    .owner
                    .resolve_type_ident(head, IdentType::Element)?;

                f(processor, &ident)?;

                processor.with_element_node(ident, |processor, element, _ident| {
                    if let Some(groups) = &element.substitution_group {
                        inner(processor, groups, f)?;
                    }

                    Ok(())
                })?;
            }

            Ok(())
        }

        inner(self, groups, &mut f)
    }

    fn simple_content_builder<F>(&mut self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut VariantProcessor<'_, 'state, 'schema>) -> Result<(), Error>,
    {
        match (self.type_mode, self.content_mode) {
            (TypeMode::Simple, _) => f(self)?,
            (TypeMode::Complex, ContentMode::Simple) => {
                let ns = self.owner.current_ns_id();
                let schema = self.owner.current_schema_id();

                let ci = get_or_init_any!(self, ComplexType);

                let Some(content_ident) = ci.content.clone() else {
                    crate::unreachable!(
                        "Complex type does not have a simple content identifier: {:?}",
                        &self.variant
                    );
                };

                let mut content_ident = content_ident;

                let content = self
                    .owner
                    .get_simple_type_variant(&content_ident)
                    .ok_or_else(|| Error::ExpectedSimpleType(content_ident.clone()))?
                    .into_owned();

                if !self.is_simple_content_unique {
                    self.is_simple_content_unique = true;
                    let content_name = self.owner.make_content_name();

                    content_ident = TypeIdent {
                        ns,
                        schema,
                        name: content_name,
                        type_: IdentType::Type,
                    };

                    ci.content = Some(content_ident.clone());
                }

                let mut processor = VariantProcessor::new(&mut *self.owner);
                processor.variant = Some(content);
                processor.type_mode = TypeMode::Simple;
                processor.content_mode = ContentMode::Simple;

                f(&mut processor)?;

                let content = processor.variant.unwrap();
                let content = MetaType::new(content);

                self.owner.add_type(content_ident, content, true)?;
            }
            (TypeMode::Complex, ContentMode::Complex) => {
                crate::unreachable!("Complex type with complex content tried to access simple content processor: {:?}", &self.variant);
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
        type_ident: TypeIdent,
        complex_content_type: ComplexContentType,
        f: F,
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut VariantProcessor<'_, 'state, 'schema>) -> Result<(), Error>,
    {
        enum UpdateContentMode {
            Unknown,
            Update,
            Append,
        }

        #[allow(clippy::large_enum_variant)]
        enum Output {
            Type(MetaType),
            Cached(TypeIdent),
        }

        let group_ident = self.owner.stack().last().and_then(|x| {
            if let StackEntry::GroupRef { ident, .. } = x {
                Some(ident.clone())
            } else {
                None
            }
        });
        let mut cached_ident = group_ident
            .clone()
            .and_then(|x| self.owner.group_cache()?.get(&x))
            .cloned();
        let is_cached = cached_ident.is_some();

        let mut variant = None;
        let mut update_content_mode = UpdateContentMode::Unknown;

        if let Some(MetaTypeVariant::ComplexType(ci)) = &self.variant {
            if let Some(content_ident) = &ci.content {
                let content_ty = self.owner.get_type(content_ident);

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

            let mut processor = VariantProcessor::new(&mut *self.owner);
            processor.type_mode = self.type_mode;
            processor.variant = Some(variant);

            f(&mut processor)?;

            let ty = processor.finish()?;

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

        let ns = self.owner.current_ns_id();

        match &mut self.variant {
            Some(MetaTypeVariant::ComplexType(ci)) => match update_content_mode {
                UpdateContentMode::Unknown => {
                    let content_ident = match output {
                        Output::Cached(ident) => ident,
                        Output::Type(ty) => {
                            self.owner.add_type(type_ident.clone(), ty, false)?;

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

                    self.owner.add_type(content_ident.clone(), ty, true)?;

                    ci.min_occurs = ci.min_occurs.min(min_occurs);
                    ci.max_occurs = ci.max_occurs.max(max_occurs);
                }
                UpdateContentMode::Append => {
                    let element_ident = match output {
                        Output::Cached(ident) => ident,
                        Output::Type(ty) => {
                            self.owner.add_type(type_ident.clone(), ty, false)?;

                            type_ident.clone()
                        }
                    };

                    let content_ident = ci.content.as_ref().unwrap();
                    let content_type = self.owner.get_type_mut(content_ident);
                    let content_variant = &mut content_type.variant;

                    let (MetaTypeVariant::All(si)
                    | MetaTypeVariant::Choice(si)
                    | MetaTypeVariant::Sequence(si)) = content_variant
                    else {
                        unreachable!();
                    };

                    let ident = ElementIdent::new(field_name).with_ns(ns);
                    let element = si.elements.insert_checked(ident, |ident| {
                        ElementMeta::new(
                            ident,
                            element_ident,
                            ElementMode::Group,
                            FormChoiceType::Unqualified,
                        )
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
                        self.owner.add_type(type_ident.clone(), ty, false)?;

                        type_ident.clone()
                    }
                };

                let ident = ElementIdent::new(field_name).with_ns(ns);
                let element = si.elements.insert_checked(ident, |ident| {
                    ElementMeta::new(
                        ident,
                        element_ident,
                        ElementMode::Group,
                        FormChoiceType::Unqualified,
                    )
                });

                element.min_occurs = element.min_occurs.min(min_occurs);
                element.max_occurs = element.max_occurs.max(max_occurs);
            }
            None => {
                let content_ident = match output {
                    Output::Cached(ident) => ident,
                    Output::Type(ty) => {
                        self.owner.add_type(type_ident.clone(), ty, false)?;

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
            if let (Some(cache), Some(group_ident)) = (self.owner.group_cache_mut(), group_ident) {
                cache.insert(group_ident, type_ident);
            }
        }

        Ok(())
    }

    fn make_field_name_and_type(&mut self, ty: &'schema GroupType) -> (Name, TypeIdent) {
        let name = self
            .owner
            .name_builder()
            .or(ty.name.clone())
            .or_else(|| self.owner.named_group())
            .field_name()
            .or_else(|| {
                self.owner
                    .name_builder()
                    .generate_id()
                    .shared_name("Content")
            });
        let field_name = name.finish();
        let type_name = self
            .owner
            .name_builder()
            .auto_extend(self.owner.stack())
            .type_name()
            .or(name)
            .finish();
        let type_ = TypeIdent {
            ns: self.owner.current_ns_id(),
            schema: self.owner.current_schema_id(),
            name: type_name,
            type_: IdentType::Type,
        };

        (field_name, type_)
    }

    impl_with_node!(with_element_node, Element, ElementType);
    impl_with_node!(with_group_node, Group, GroupType);
    impl_with_node!(
        with_attribute_group_node,
        AttributeGroup,
        AttributeGroupType
    );
}

/* ElementsMeta */

impl ElementsMeta {
    fn insert_checked<F>(&mut self, ident: ElementIdent, f: F) -> &mut ElementMeta
    where
        F: FnOnce(ElementIdent) -> ElementMeta,
    {
        let vec = &mut **self;

        let Some(index) = vec.iter().position(|x| x.ident.eq(&ident)) else {
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

            if !vec.iter().any(|x| x.ident.eq(&new_ident)) {
                vec.push(f(new_ident));

                return vec.last_mut().unwrap();
            }
        }
    }
}
