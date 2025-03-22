use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::str::from_utf8;

use tracing::instrument;

use crate::schema::xs::{
    Any, AnyAttribute, AttributeGroupType, AttributeType, ComplexBaseType, ComplexContent,
    ElementSubstitutionGroupType, ElementType, ExtensionType, Facet, FacetType, GroupType, List,
    Restriction, RestrictionType, SimpleBaseType, SimpleContent, Union, Use,
};
use crate::schema::{MaxOccurs, MinOccurs, Namespace};
use crate::types::{
    AnyAttributeInfo, AnyInfo, AttributeInfo, Base, DynamicInfo, ElementInfo, ElementMode, Ident,
    IdentType, Name, ReferenceInfo, Type, UnionTypeInfo, VariantInfo, VecHelper,
};

use super::{Error, NameExtend, NameFallback, NameUnwrap, SchemaInterpreter};

#[derive(Debug)]
pub(super) struct TypeBuilder<'a, 'schema, 'state> {
    type_: Option<Type>,

    fixed: bool,
    type_mode: TypeMode,
    content_mode: ContentMode,
    simple_base: Option<Ident>,

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
        $builder.type_ = Some(Type::$variant($value));
        $builder.fixed = $fixed;

        let Type::$variant(ret) = $builder.type_.as_mut().unwrap() else {
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
        match &mut $builder.type_ {
            None => init_any!($builder, $variant, $default, true),
            Some(Type::$variant(ret)) => ret,
            _ if !$builder.fixed => init_any!($builder, $variant, $default, true),
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    };
}

/// Get the `SimpleInfo` from any possible type or initialize the required variant.
macro_rules! get_or_init_type {
    ($builder:expr, $variant:ident) => {
        get_or_init_type!($builder, $variant, Default::default())
    };
    ($builder:expr, $variant:ident, $default:expr) => {
        match &mut $builder.type_ {
            None => init_any!($builder, $variant, $default, true),
            Some(Type::All(si) | Type::Choice(si) | Type::Sequence(si)) => si,
            _ if !$builder.fixed => init_any!($builder, $variant, $default, true),
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    };
}

/* TypeBuilder */

impl<'a, 'schema, 'state> TypeBuilder<'a, 'schema, 'state> {
    pub(super) fn new(owner: &'a mut SchemaInterpreter<'schema, 'state>) -> Self {
        Self {
            type_: None,
            fixed: false,
            type_mode: TypeMode::Unknown,
            content_mode: ContentMode::Unknown,
            simple_base: None,
            owner,
        }
    }

    pub(super) fn finish(self) -> Result<Type, Error> {
        self.type_.ok_or(Error::NoType)
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_element(&mut self, ty: &ElementType) -> Result<(), Error> {
        use crate::schema::xs::ElementTypeContent as C;

        if let Some(type_) = &ty.type_ {
            let type_ = self.parse_qname(type_)?;

            init_any!(self, Reference, ReferenceInfo::new(type_), true);
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
                init_any!(self, Reference, ReferenceInfo::new(type_?), true);
            }
        } else {
            let xs = self
                .schemas
                .resolve_namespace(&Some(Namespace::XS))
                .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;
            let ident = Ident::ANY_TYPE.with_ns(Some(xs));

            init_any!(self, Reference, ReferenceInfo::new(ident), true);
        };

        if ty.abstract_ {
            let type_ = match self.type_.take() {
                None => None,
                Some(Type::Reference(ti)) => Some(ti.type_),
                e => crate::unreachable!("Unexpected type: {:?}", e),
            };

            let ai = init_any!(self, Dynamic);
            ai.type_ = type_;
        }

        if let Some(substitution_group) = &ty.substitution_group {
            self.walk_substitution_groups(substitution_group, |builder, base_ident| {
                let ident = builder.state.current_ident().unwrap().clone();
                let base_ty = builder.get_element_mut(base_ident)?;

                if let Type::Reference(ti) = base_ty {
                    *base_ty = Type::Dynamic(DynamicInfo {
                        type_: Some(ti.type_.clone()),
                        derived_types: vec![ti.type_.clone()],
                    });
                }

                let Type::Dynamic(ai) = base_ty else {
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
            init_any!(self, Reference, ReferenceInfo::new(type_), false);
        } else if let Some(x) = &ty.simple_type {
            self.apply_simple_type(x)?;
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_simple_type(&mut self, ty: &SimpleBaseType) -> Result<(), Error> {
        use crate::schema::xs::SimpleBaseTypeContent as C;

        self.type_mode = TypeMode::Simple;
        self.content_mode = ContentMode::Simple;

        for c in &ty.content {
            match c {
                C::Annotation(_) => (),
                C::Restriction(x) => self.apply_simple_type_restriction(x)?,
                C::Union(x) => self.apply_union(x)?,
                C::List(x) => self.apply_list(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_type_restriction(&mut self, ty: &Restriction) -> Result<(), Error> {
        use crate::schema::xs::RestrictionContent as C;

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
                C::Annotation(_) => (),
                C::SimpleType(x) => self.apply_simple_type(x)?,
                C::Facet(x) => self.apply_facet(x)?,
            }
        }

        if let Some(base) = base {
            match &mut self.type_ {
                Some(Type::Reference(_)) => (),
                Some(Type::Union(e)) => e.base = Base::Extension(base),
                Some(Type::Enumeration(e)) => e.base = Base::Extension(base),
                Some(Type::ComplexType(e)) => e.base = Base::Extension(base),
                e => unreachable!("Unexpected type: {e:#?}"),
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn apply_complex_type(&mut self, ty: &ComplexBaseType) -> Result<(), Error> {
        use crate::schema::xs::ComplexBaseTypeContent as C;

        self.type_mode = TypeMode::Complex;
        self.content_mode = ContentMode::Complex;

        get_or_init_any!(self, ComplexType);

        for c in &ty.content {
            match c {
                C::Annotation(_) | C::OpenContent(_) | C::Assert(_) => (),
                C::ComplexContent(x) => {
                    let ci = get_or_init_any!(self, ComplexType);
                    ci.is_dynamic = ty.abstract_;
                    self.apply_complex_content(x)?;
                }
                C::SimpleContent(x) => self.apply_simple_content(x)?,
                C::All(x) => {
                    get_or_init_any!(self, ComplexType);
                    self.apply_all(x)?;
                }
                C::Choice(x) => {
                    get_or_init_any!(self, ComplexType);
                    self.apply_choice(x)?;
                }
                C::Sequence(x) => {
                    get_or_init_any!(self, ComplexType);
                    self.apply_sequence(x)?;
                }
                C::Attribute(x) => self.apply_attribute_ref(x)?,
                C::AnyAttribute(x) => self.apply_any_attribute(x)?,
                C::Group(x) => self.apply_group_ref(x)?,
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x)?,
            }
        }

        if ty.abstract_ {
            get_or_init_any!(self, ComplexType);
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_content(&mut self, ty: &SimpleContent) -> Result<(), Error> {
        use crate::schema::xs::SimpleContentContent as C;

        self.content_mode = ContentMode::Simple;

        for c in &ty.content {
            match c {
                C::Annotation(_) => (),
                C::Extension(x) => self.apply_simple_content_extension(x)?,
                C::Restriction(x) => self.apply_simple_content_restriction(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_complex_content(&mut self, ty: &ComplexContent) -> Result<(), Error> {
        use crate::schema::xs::ComplexContentContent as C;

        get_or_init_any!(self, ComplexType);

        for c in &ty.content {
            match c {
                C::Annotation(_) => (),
                C::Extension(x) => self.apply_complex_content_extension(x)?,
                C::Restriction(x) => self.apply_complex_content_restriction(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_content_extension(&mut self, ty: &ExtensionType) -> Result<(), Error> {
        let base = self.parse_qname(&ty.base)?;

        self.copy_base_type(&base, UpdateMode::Extension)?;
        self.apply_extension(ty)?;

        match &mut self.type_ {
            Some(Type::Reference(_)) => (),
            Some(Type::Union(e)) => e.base = Base::Extension(base),
            Some(Type::Enumeration(e)) => e.base = Base::Extension(base),
            Some(Type::ComplexType(e)) => e.base = Base::Extension(base),
            e => crate::unreachable!("Unexpected type: {e:#?}!"),
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_content_restriction(&mut self, ty: &RestrictionType) -> Result<(), Error> {
        let base = self.parse_qname(&ty.base)?;

        self.copy_base_type(&base, UpdateMode::Restriction)?;
        self.apply_restriction(ty)?;

        match &mut self.type_ {
            Some(Type::Reference(_)) => (),
            Some(Type::Union(e)) => e.base = Base::Restriction(base),
            Some(Type::Enumeration(e)) => e.base = Base::Restriction(base),
            Some(Type::ComplexType(e)) => e.base = Base::Restriction(base),
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
        use crate::schema::xs::ExtensionTypeContent as C;

        for c in &ty.content {
            match c {
                C::Annotation(_) | C::OpenContent(_) | C::Assert(_) => (),
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
        use crate::schema::xs::RestrictionTypeContent as C;

        for c in &ty.content {
            match c {
                C::Annotation(_) | C::OpenContent(_) | C::Assert(_) => (),
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

        self.create_subtype_builder(
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

        self.create_subtype_builder(
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

        self.create_subtype_builder(
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
        use crate::schema::xs::GroupTypeContent as C;

        for c in &ty.content {
            match c {
                C::Annotation(_) => (),
                C::All(x) => self.apply_all(x)?,
                C::Choice(x) => self.apply_choice(x)?,
                C::Sequence(x) => self.apply_sequence(x)?,
                C::Any(x) => self.apply_any(x)?,
                C::Group(x) => self.apply_group_ref(x)?,
                C::Element(x) => self.apply_element_ref(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_element_ref(&mut self, ty: &ElementType) -> Result<(), Error> {
        match ty {
            ElementType {
                ref_: Some(ref_),
                name,
                ..
            } => {
                let type_ = self.parse_qname(ref_)?.with_type(IdentType::Element);
                let name = name.clone().or_fallback(type_.name.clone());
                let ident = Ident::new(name)
                    .with_ns(type_.ns)
                    .with_type(IdentType::Element);

                let ci = get_or_init_type!(self, Sequence);
                let element = ci.elements.find_or_insert(ident, |ident| {
                    ElementInfo::new(ident, type_, ElementMode::Element)
                });
                crate::assert_eq!(element.element_mode, ElementMode::Element);
                element.update(ty);
            }
            ty => {
                let field_name = ty.name.clone().unwrap_or_unnamed(self.state);
                let field_ident = Ident::new(field_name)
                    .with_ns(self.state.current_ns())
                    .with_type(IdentType::Element);
                let type_name = self
                    .state
                    .make_unnamed()
                    .extend(true, ty.name.clone())
                    .auto_extend(true, false, self.state);
                let type_name = if type_name.has_extension() {
                    type_name.to_type_name(false, Some(""))
                } else {
                    type_name.to_type_name(true, Some("Temp"))
                };

                let ns = self.state.current_ns();
                let type_ = self.create_element(ns, Some(type_name), ty)?;

                let ci = get_or_init_type!(self, Sequence);
                let element = ci.elements.find_or_insert(field_ident, |ident| {
                    ElementInfo::new(ident, type_, ElementMode::Element)
                });
                crate::assert_eq!(element.element_mode, ElementMode::Element);
                element.update(ty);
            }
        };

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_group_ref(&mut self, ty: &GroupType) -> Result<(), Error> {
        use crate::schema::xs::GroupTypeContent as C;

        let ref_ = ty.ref_.as_ref().ok_or(Error::GroupMissingRef)?;
        let ref_ = self.parse_qname(ref_)?.with_type(IdentType::Group);
        let group = self
            .find_group(ref_.clone())
            .ok_or(Error::UnknownElement(ref_))?;

        self.state.type_stack.push(None);

        for c in &group.content {
            match c {
                C::Annotation(_) => (),
                C::Any(x) => self.apply_any(x)?,
                C::All(x) => self.apply_all(&x.patch(ty))?,
                C::Choice(x) => self.apply_choice(&x.patch(ty))?,
                C::Sequence(x) => self.apply_sequence(&x.patch(ty))?,
                C::Group(x) => self.apply_group_ref(x)?,
                C::Element(x) => self.apply_element_ref(x)?,
            }
        }

        self.state.type_stack.pop();

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_attribute_group_ref(&mut self, ty: &AttributeGroupType) -> Result<(), Error> {
        use crate::schema::xs::AttributeGroupTypeContent as C;

        let ref_ = ty.ref_.as_ref().ok_or(Error::AttributeGroupMissingRef)?;
        let ref_ = self.parse_qname(ref_)?.with_type(IdentType::AttributeGroup);
        let group = self
            .find_attribute_group(ref_.clone())
            .ok_or(Error::UnknownElement(ref_))?;

        self.state.type_stack.push(None);

        for c in &group.content {
            match c {
                C::Annotation(_) => (),
                C::Attribute(x) => self.apply_attribute_ref(x)?,
                C::AnyAttribute(x) => self.apply_any_attribute(x)?,
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x)?,
            }
        }

        self.state.type_stack.pop();

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_attribute_ref(&mut self, ty: &AttributeType) -> Result<(), Error> {
        match ty {
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
                ci.attributes
                    .find_or_insert(ident, |ident| AttributeInfo::new(ident, type_))
                    .update(ty);

                if let Some(content) = self.simple_base.take() {
                    ci.content = Some(content);
                }
            }
            AttributeType {
                ref_: Some(ref_),
                name,
                ..
            } => {
                let type_ = self.parse_qname(ref_)?.with_type(IdentType::Attribute);
                let name = name.clone().or_fallback(type_.name.clone());
                let ident = Ident::new(name)
                    .with_ns(type_.ns)
                    .with_type(IdentType::Attribute);

                let ci = get_or_init_any!(self, ComplexType);
                ci.attributes
                    .find_or_insert(ident, |ident| AttributeInfo::new(ident, type_))
                    .update(ty);
            }
            AttributeType {
                name: Some(name),
                simple_type,
                ..
            } => {
                let type_ = simple_type
                    .as_ref()
                    .map(|x| {
                        let type_name = name.clone().auto_extend(true, true, self.state);
                        let ns = self.state.current_ns();

                        self.create_simple_type(ns, Some(type_name), x)
                    })
                    .transpose()?;
                let name = Name::from(name.clone());
                let ident = Ident::new(name)
                    .with_ns(self.state.current_ns())
                    .with_type(IdentType::Attribute);

                let ci = get_or_init_any!(self, ComplexType);
                ci.attributes
                    .find_or_insert(ident, |ident| {
                        AttributeInfo::new(ident, type_.unwrap_or(Ident::STRING))
                    })
                    .update(ty);
            }
            e => return Err(Error::InvalidAttributeReference(Box::new(e.clone()))),
        };

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_any(&mut self, ty: &Any) -> Result<(), Error> {
        let si = get_or_init_type!(self, Sequence);
        si.any.create_or_update(ty);

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_any_attribute(&mut self, ty: &AnyAttribute) -> Result<(), Error> {
        let ci = get_or_init_any!(self, ComplexType);
        ci.any_attribute.create_or_update(ty);

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

        let ei = get_or_init_any!(self, Enumeration);
        let var = ei.variants.find_or_insert(ident, VariantInfo::new);
        var.use_ = Use::Required;

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_union(&mut self, ty: &Union) -> Result<(), Error> {
        let ui = get_or_init_any!(self, Union);

        if let Some(types) = &ty.member_types {
            for type_ in &types.0 {
                let type_ = self.owner.parse_qname(type_)?;
                ui.types.push(UnionTypeInfo::new(type_));
            }
        }

        let ns = self.owner.state.current_ns();

        for x in &ty.simple_type {
            let name = x
                .name
                .clone()
                .auto_extend(false, true, self.owner.state)
                .unwrap_or_unnamed(self.owner.state);
            let type_ = self.owner.create_simple_type(ns, Some(name), x)?;
            ui.types.push(UnionTypeInfo::new(type_));
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_list(&mut self, ty: &List) -> Result<(), Error> {
        let mut type_ = None;

        if let Some(s) = &ty.item_type {
            type_ = Some(self.owner.parse_qname(s)?);
        }

        if let Some(x) = &ty.simple_type {
            let last_named_type = self.state.last_named_type(false).map(ToOwned::to_owned);
            let name = x
                .name
                .clone()
                .or_fallback(|| {
                    Some(Name::new(
                        from_utf8(ty.item_type.as_ref()?.local_name()).ok()?,
                    ))
                })
                .or_fallback(|| {
                    let s = last_named_type?;
                    let s = s.as_str();
                    let s = s.strip_suffix("Type").unwrap_or(s);
                    let s = format!("{s}Item");

                    Some(Name::from(s))
                })
                .unwrap_or_unnamed(self.state);
            let ns = self.owner.state.current_ns();
            type_ = Some(self.owner.create_simple_type(ns, Some(name), x)?);
        }

        if let Some(type_) = type_ {
            let ti = get_or_init_any!(self, Reference, ReferenceInfo::new(type_.clone()));
            ti.type_ = type_;
            ti.min_occurs = 0;
            ti.max_occurs = MaxOccurs::Unbounded;
        }

        Ok(())
    }

    fn copy_base_type(&mut self, base: &Ident, mode: UpdateMode) -> Result<(), Error> {
        let base = match (self.type_mode, self.content_mode) {
            (TypeMode::Simple, ContentMode::Simple) => {
                self.fixed = false;
                self.simple_base = Some(base.clone());

                self.owner.get_simple_type(base)?
            }
            (TypeMode::Complex, ContentMode::Simple) => match self.owner.get_simple_type(base) {
                Ok(ty) => {
                    self.fixed = false;
                    self.simple_base = Some(base.clone());

                    ty
                }
                Err(Error::UnknownType(_)) => {
                    self.fixed = true;

                    self.owner.get_complex_type(base)?
                }
                Err(error) => Err(error)?,
            },
            (TypeMode::Complex, ContentMode::Complex) => self.owner.get_complex_type(base)?,
            (_, _) => crate::unreachable!("Unset or invalid combination!"),
        };

        tracing::debug!("{base:#?}");

        let mut base = base.clone();

        match (self.content_mode, &mut base) {
            (ContentMode::Simple, Type::Enumeration(ei)) => ei.variants.clear(),
            (ContentMode::Complex, Type::ComplexType(ci)) => {
                if let Some(content_ident) = &ci.content {
                    let mut content_type = self
                        .owner
                        .state
                        .types
                        .get(content_ident)
                        .ok_or_else(|| Error::UnknownType(content_ident.clone()))?
                        .clone();

                    match (&mut content_type, mode) {
                        (Type::All(si) | Type::Choice(si) | Type::Sequence(si), UpdateMode::Restriction) => {
                            si.elements.retain(|element| element.min_occurs > 0);

                            if matches!(
                                si.any,
                                Some(AnyInfo {
                                    min_occurs: Some(0),
                                    ..
                                })
                            ) {
                                si.any = None;
                            }
                        }
                        (_, UpdateMode::Extension) => (),
                        (_, _) => tracing::warn!("Complex type does not has `All`, `Choice` or `Sequence` as content: {content_ident:#?} => {content_type:#?}!"),
                    }

                    let content_ident = self
                        .state
                        .make_unnamed()
                        .auto_extend(false, true, self.state)
                        .remove_suffix("Type")
                        .remove_suffix("Content")
                        .to_type_name(true, Some("Content"));
                    let content_ident = Ident::new(content_ident).with_ns(self.state.current_ns());

                    self.state
                        .add_type(content_ident.clone(), content_type, false)?;

                    ci.content = Some(content_ident);
                }
            }
            (_, _) => (),
        }

        self.type_ = Some(base);

        Ok(())
    }

    // #[instrument(err, level = "trace", skip(self, f))]
    fn walk_substitution_groups<F>(
        &mut self,
        groups: &ElementSubstitutionGroupType,
        mut f: F,
    ) -> Result<(), Error>
    where
        F: FnMut(&mut Self, &Ident) -> Result<(), Error>,
    {
        fn inner<'x, 'y, 'z, F>(
            builder: &mut TypeBuilder<'x, 'y, 'z>,
            groups: &ElementSubstitutionGroupType,
            f: &mut F,
        ) -> Result<(), Error>
        where
            F: FnMut(&mut TypeBuilder<'x, 'y, 'z>, &Ident) -> Result<(), Error>,
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

    #[instrument(err, level = "trace", skip(self, f))]
    fn create_subtype_builder<F>(
        &mut self,
        field_name: Name,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        type_ident: Ident,
        complex_content_type: ComplexContentType,
        f: F,
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut TypeBuilder<'_, 'schema, 'state>) -> Result<(), Error>,
    {
        enum UpdateContentMode {
            Unknown,
            Update,
            Append,
        }

        let mut ty = None;
        let mut update_content_mode = UpdateContentMode::Unknown;

        if let Some(Type::ComplexType(ci)) = &self.type_ {
            if let Some(content_ident) = &ci.content {
                let content_ty = self
                    .owner
                    .state
                    .types
                    .get(content_ident)
                    .ok_or_else(|| Error::UnknownType(content_ident.clone()))?;

                match (complex_content_type, content_ty) {
                    (ComplexContentType::All, Type::All(_))
                    | (ComplexContentType::Choice, Type::Choice(_))
                    | (ComplexContentType::Sequence, Type::Sequence(_)) => {
                        ty = Some(content_ty.clone());
                        update_content_mode = UpdateContentMode::Update;
                    }
                    (_, _) => update_content_mode = UpdateContentMode::Append,
                }
            }
        }

        let ty = ty.unwrap_or_else(|| match complex_content_type {
            ComplexContentType::All => Type::All(Default::default()),
            ComplexContentType::Choice => Type::Choice(Default::default()),
            ComplexContentType::Sequence => Type::Sequence(Default::default()),
        });

        let mut builder = TypeBuilder::new(&mut *self.owner);
        builder.type_mode = self.type_mode;
        builder.type_ = Some(ty);

        f(&mut builder)?;

        let ty = builder.finish()?;

        let (Type::All(si) | Type::Choice(si) | Type::Sequence(si)) = &ty else {
            return Err(Error::ExpectedGroupType);
        };

        if si.elements.is_empty() && si.any.is_none() {
            return Ok(());
        }

        let ns = self.state.current_ns();

        match &mut self.type_ {
            Some(Type::ComplexType(ci)) => match update_content_mode {
                UpdateContentMode::Unknown => {
                    self.owner.state.add_type(type_ident.clone(), ty, false)?;

                    ci.content = Some(type_ident);
                    ci.min_occurs = ci.min_occurs.min(min_occurs);
                    ci.max_occurs = ci.max_occurs.max(max_occurs);
                }
                UpdateContentMode::Update => {
                    let content_ident = ci.content.as_ref().unwrap();

                    self.owner.state.types.insert(content_ident.clone(), ty);

                    ci.min_occurs = ci.min_occurs.min(min_occurs);
                    ci.max_occurs = ci.max_occurs.max(max_occurs);
                }
                UpdateContentMode::Append => {
                    self.owner.state.add_type(type_ident.clone(), ty, false)?;

                    let content_ident = ci.content.as_ref().unwrap();
                    let content_type = self.owner.state.types.get_mut(content_ident).unwrap();

                    let (Type::All(si) | Type::Choice(si) | Type::Sequence(si)) = content_type
                    else {
                        unreachable!();
                    };

                    let ident = Ident::new(field_name)
                        .with_ns(ns)
                        .with_type(IdentType::Group);
                    let element = si.elements.find_or_insert(ident, |ident| {
                        ElementInfo::new(ident, type_ident, ElementMode::Group)
                    });

                    element.min_occurs = element.min_occurs.min(min_occurs);
                    element.max_occurs = element.max_occurs.max(max_occurs);
                }
            },
            Some(Type::All(si) | Type::Choice(si) | Type::Sequence(si)) => {
                self.owner.state.add_type(type_ident.clone(), ty, false)?;

                let ident = Ident::new(field_name)
                    .with_ns(ns)
                    .with_type(IdentType::Group);
                let element = si.elements.find_or_insert(ident, |ident| {
                    ElementInfo::new(ident, type_ident, ElementMode::Group)
                });

                element.min_occurs = element.min_occurs.min(min_occurs);
                element.max_occurs = element.max_occurs.max(max_occurs);
            }
            None => {
                self.owner.state.add_type(type_ident.clone(), ty, false)?;

                let ci = get_or_init_any!(self, ComplexType);

                ci.content = Some(type_ident);
                ci.min_occurs = ci.min_occurs.min(min_occurs);
                ci.max_occurs = ci.max_occurs.max(max_occurs);
            }
            e => crate::unreachable!("{:?}", e),
        }

        Ok(())
    }

    fn make_field_name_and_type(&mut self, ty: &GroupType) -> (Name, Ident) {
        let name = ty
            .name
            .clone()
            .unwrap_or_unnamed(self.state)
            .remove_suffix("Type")
            .remove_suffix("Content");
        let field_name = name.to_type_name(true, Some("Content"));
        let type_name = name
            .auto_extend(false, true, self.state)
            .remove_suffix("Type")
            .remove_suffix("Content")
            .to_type_name(true, Some("Content"));
        let type_ = Ident::new(type_name).with_ns(self.state.current_ns());

        (field_name, type_)
    }
}

impl<'schema, 'state> Deref for TypeBuilder<'_, 'schema, 'state> {
    type Target = SchemaInterpreter<'schema, 'state>;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}

impl DerefMut for TypeBuilder<'_, '_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.owner
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

impl Update<GroupType> for ElementInfo {
    fn update(&mut self, other: &GroupType) {
        self.min_occurs = other.min_occurs;
        self.max_occurs = other.max_occurs;
    }
}

impl Update<Any> for AnyInfo {
    fn update(&mut self, other: &Any) {
        self.min_occurs = Some(other.min_occurs);
        self.max_occurs = Some(other.max_occurs);
        self.process_contents = Some(other.process_contents.clone());

        self.namespace.update(&other.namespace);
        self.not_q_name.update(&other.not_q_name);
        self.not_namespace.update(&other.not_namespace);
    }
}

impl Update<AnyAttribute> for AnyAttributeInfo {
    fn update(&mut self, other: &AnyAttribute) {
        self.process_contents = Some(other.process_contents.clone());

        self.namespace.update(&other.namespace);
        self.not_q_name.update(&other.not_q_name);
        self.not_namespace.update(&other.not_namespace);
    }
}

impl Update<ElementType> for ElementInfo {
    fn update(&mut self, other: &ElementType) {
        self.min_occurs = other.min_occurs;
        self.max_occurs = other.max_occurs;
    }
}

impl Update<AttributeType> for AttributeInfo {
    fn update(&mut self, other: &AttributeType) {
        self.use_ = other.use_.clone();
        self.default.update(&other.default);
    }
}

/* CreateOrUpdate */

trait CreateOrUpdate<T> {
    fn create_or_update(&mut self, other: &T);
}

impl<T, X> CreateOrUpdate<T> for Option<X>
where
    X: Update<T> + Default,
{
    fn create_or_update(&mut self, other: &T) {
        if let Some(x) = self {
            x.update(other);
        } else {
            let mut x = X::default();
            x.update(other);
            *self = Some(x);
        }
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
