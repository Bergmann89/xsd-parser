use std::collections::btree_map::Entry;
use std::ops::{Deref, DerefMut};

use tracing::instrument;

use super::{CreateOrUpdate, Patch, Update};
use crate::interpreter::builders::SimpleTypeBuilder;
use crate::schema::xs::{
    Any, AnyAttribute, AttributeGroupType, AttributeType, ComplexBaseType, ComplexContent,
    ElementType, ExtensionType, Facet, GroupType, RestrictionType, SimpleContent,
};
use crate::schema::{MaxOccurs, MinOccurs};
use crate::types::{
    AnyInfo, AttributeInfo, Base, ElementInfo, ElementMode, Ident, IdentType, Name, Type,
    TypeVariant, VecHelper,
};

use super::super::{Error, SchemaInterpreter};

#[derive(Debug)]
pub(crate) struct ComplexTypeBuilder<'a, 'schema, 'state> {
    type_: Option<Type>,

    /// Type variant that is constructed by the builder
    variant: Option<TypeVariant>,

    /// `true` if `type_` is fixed and can not be changed anymore
    fixed: bool,

    /// Mode of the content of the constructed type
    content_mode: ContentMode,

    /// `true` if the simple content type of a complex type was already duplicated
    /// and is now unique for this type.
    is_simple_content_unique: bool,

    owner: &'a mut SchemaInterpreter<'schema, 'state>,
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
    ($builder:expr, $variant:ident, $value:expr, $fixed:expr) => {{
        $builder.variant = Some(TypeVariant::$variant($value));
        $builder.fixed = $fixed;

        let TypeVariant::$variant(ret) = $builder.variant.as_mut().unwrap() else {
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
            Some(TypeVariant::$variant(ret)) => ret,
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
        match &mut $builder.variant {
            None => init_any!($builder, $variant, $default, true),
            Some(TypeVariant::All(si) | TypeVariant::Choice(si) | TypeVariant::Sequence(si)) => si,
            _ if !$builder.fixed => init_any!($builder, $variant, $default, true),
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    };
}

/* TypeBuilder */

impl<'a, 'schema, 'state> ComplexTypeBuilder<'a, 'schema, 'state> {
    pub(crate) fn new(owner: &'a mut SchemaInterpreter<'schema, 'state>) -> Self {
        Self {
            type_: None,
            variant: None,
            fixed: false,
            content_mode: ContentMode::Unknown,
            is_simple_content_unique: false,
            owner,
        }
    }

    pub(crate) fn finish(self) -> Result<Type, Error> {
        self.type_
            .or_else(|| self.variant.map(Type::new))
            .ok_or(Error::NoType)
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(crate) fn apply_complex_type(&mut self, ty: &ComplexBaseType) -> Result<(), Error> {
        use crate::schema::xs::ComplexBaseTypeContent as C;

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
                C::All(x) => self.apply_all(x)?,
                C::Choice(x) => self.apply_choice(x)?,
                C::Sequence(x) => self.apply_sequence(x)?,
                C::Attribute(x) => self.apply_attribute_ref(x)?,
                C::AnyAttribute(x) => self.apply_any_attribute(x)?,
                C::Group(x) => self.apply_group_ref(x)?,
                C::AttributeGroup(x) => self.apply_attribute_group_ref(x)?,
            }
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

        match &mut self.variant {
            Some(TypeVariant::Reference(_)) => (),
            Some(TypeVariant::Union(e)) => e.base = Base::Extension(base),
            Some(TypeVariant::Enumeration(e)) => e.base = Base::Extension(base),
            Some(TypeVariant::ComplexType(e)) => e.base = Base::Extension(base),
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
            Some(TypeVariant::Reference(_)) => (),
            Some(TypeVariant::Union(e)) => e.base = Base::Restriction(base),
            Some(TypeVariant::Enumeration(e)) => e.base = Base::Restriction(base),
            Some(TypeVariant::ComplexType(e)) => e.base = Base::Restriction(base),
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
                C::SimpleType(x) => {
                    let mut builder = SimpleTypeBuilder::new(self.owner);
                    builder.apply_simple_type(x)?;
                    self.type_ = Some(builder.finish()?);
                }
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
                let name = self.state.name_builder().or(name).or(&type_.name).finish();
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
                let field_name = self.state.name_builder().or(&ty.name).finish();
                let field_ident = Ident::new(field_name)
                    .with_ns(self.state.current_ns())
                    .with_type(IdentType::Element);
                let type_name = self
                    .state
                    .name_builder()
                    .extend(true, ty.name.clone())
                    .auto_extend2(true, false, self.state);
                let type_name = if type_name.has_extension() {
                    type_name.with_id(false)
                } else {
                    type_name.shared_name("Temp")
                };
                let type_name = type_name.finish();

                let ns = self.state.current_ns();
                let type_ = self.create_element(ns, Some(type_name), ty)?;

                let ci = get_or_init_type!(self, Sequence);
                let element = ci.elements.find_or_insert(field_ident, |ident| {
                    ElementInfo::new(ident, type_, ElementMode::Element)
                });
                crate::assert_eq!(element.element_mode, ElementMode::Element);
                element.update(ty);
            }
        }

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
                        let type_name = self
                            .state
                            .name_builder()
                            .or(name)
                            .auto_extend2(true, true, self.state)
                            .finish();
                        let ns = self.state.current_ns();

                        self.create_simple_type(ns, Some(type_name), None, x)
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
        }

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
        self.simple_content_builder(|builder| builder.apply_facet(ty))
    }

    fn copy_base_type(&mut self, base: &Ident, mode: UpdateMode) -> Result<(), Error> {
        let mut simple_base_ident = None;
        let base = match self.content_mode {
            ContentMode::Simple => match self.owner.get_simple_type_variant(base) {
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
            },
            ContentMode::Complex => self.owner.get_complex_type_variant(base)?,
            _ => crate::unreachable!("Unset or invalid combination!"),
        };

        tracing::debug!("{base:#?}");

        let mut base = base.clone();

        match (self.content_mode, &mut base) {
            (ContentMode::Simple, TypeVariant::Enumeration(ei)) => ei.variants.clear(),
            (ContentMode::Complex, TypeVariant::ComplexType(ci)) => {
                if let Some(content_ident) = &ci.content {
                    let mut content_type = self
                        .owner
                        .state
                        .types
                        .get(content_ident)
                        .ok_or_else(|| Error::UnknownType(content_ident.clone()))?
                        .clone();

                    match (&mut content_type.variant, mode) {
                        (TypeVariant::All(si) | TypeVariant::Choice(si) | TypeVariant::Sequence(si), UpdateMode::Restriction) => {
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

                    let content_name = self.state.make_content_name();
                    let content_ident = Ident::new(content_name).with_ns(self.state.current_ns());

                    self.state
                        .add_type(content_ident.clone(), content_type, false)?;

                    ci.content = Some(content_ident);
                }
            }
            (_, _) => (),
        }

        match (simple_base_ident, self.content_mode) {
            (Some(base_ident), ContentMode::Simple) => {
                let ci = get_or_init_any!(self, ComplexType);
                ci.content.get_or_insert(base_ident);
            }
            (None, ContentMode::Simple | ContentMode::Complex) => {
                self.variant = Some(base);
            }
            (_, _) => crate::unreachable!("Unset or invalid combination!"),
        }

        tracing::debug!("{:#?}", self.variant);

        Ok(())
    }

    fn simple_content_builder<F>(&mut self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut SimpleTypeBuilder<'_, 'schema, 'state>) -> Result<(), Error>,
    {
        match self.content_mode {
            ContentMode::Simple => {
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

                let mut builder = SimpleTypeBuilder::new(&mut *self.owner);
                builder.variant = Some(content);

                f(&mut builder)?;

                let content = builder.variant.unwrap();
                let content = Type::new(content);

                match self.owner.state.types.entry(content_ident) {
                    Entry::Vacant(e) => {
                        e.insert(content);
                    }
                    Entry::Occupied(e) => {
                        *e.into_mut() = content;
                    }
                }
            }
            ContentMode::Complex => {
                crate::unreachable!("Complex type with complex content tried to access simple content builder: {:?}", &self.variant);
            }
            _ => crate::unreachable!("Unset or invalid combination!"),
        }

        Ok(())
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
        F: FnOnce(&mut ComplexTypeBuilder<'_, 'schema, 'state>) -> Result<(), Error>,
    {
        enum UpdateContentMode {
            Unknown,
            Update,
            Append,
        }

        let mut variant = None;
        let mut update_content_mode = UpdateContentMode::Unknown;

        if let Some(TypeVariant::ComplexType(ci)) = &self.variant {
            if let Some(content_ident) = &ci.content {
                let content_ty = self
                    .owner
                    .state
                    .types
                    .get(content_ident)
                    .ok_or_else(|| Error::UnknownType(content_ident.clone()))?;

                match (complex_content_type, &content_ty.variant) {
                    (ComplexContentType::All, TypeVariant::All(_))
                    | (ComplexContentType::Choice, TypeVariant::Choice(_))
                    | (ComplexContentType::Sequence, TypeVariant::Sequence(_)) => {
                        variant = Some(content_ty.variant.clone());
                        update_content_mode = UpdateContentMode::Update;
                    }
                    (_, _) => update_content_mode = UpdateContentMode::Append,
                }
            }
        }

        let variant = variant.unwrap_or_else(|| match complex_content_type {
            ComplexContentType::All => TypeVariant::All(Default::default()),
            ComplexContentType::Choice => TypeVariant::Choice(Default::default()),
            ComplexContentType::Sequence => TypeVariant::Sequence(Default::default()),
        });

        let mut builder = ComplexTypeBuilder::new(&mut *self.owner);
        builder.variant = Some(variant);

        f(&mut builder)?;

        let ty = builder.finish()?;

        let (TypeVariant::All(si) | TypeVariant::Choice(si) | TypeVariant::Sequence(si)) =
            &ty.variant
        else {
            return Err(Error::ExpectedGroupType);
        };

        if si.elements.is_empty() && si.any.is_none() {
            return Ok(());
        }

        let ns = self.state.current_ns();

        match &mut self.variant {
            Some(TypeVariant::ComplexType(ci)) => match update_content_mode {
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
                    let content_variant = &mut content_type.variant;

                    let (TypeVariant::All(si)
                    | TypeVariant::Choice(si)
                    | TypeVariant::Sequence(si)) = content_variant
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
            Some(TypeVariant::All(si) | TypeVariant::Choice(si) | TypeVariant::Sequence(si)) => {
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
        let name = self
            .state
            .name_builder()
            .generate_id()
            .or(ty.name.clone())
            .remove_suffix("Type")
            .remove_suffix("Content");
        let field_name = name.clone().shared_name("Content").finish();
        let type_name = name
            .auto_extend2(false, true, self.state)
            .remove_suffix("Type")
            .remove_suffix("Content")
            .shared_name("Content")
            .with_id(true)
            .finish();
        let type_ = Ident::new(type_name).with_ns(self.state.current_ns());

        (field_name, type_)
    }
}

impl<'schema, 'state> Deref for ComplexTypeBuilder<'_, 'schema, 'state> {
    type Target = SchemaInterpreter<'schema, 'state>;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}

impl DerefMut for ComplexTypeBuilder<'_, '_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.owner
    }
}
