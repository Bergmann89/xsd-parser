use std::ops::{Deref, DerefMut};

use tracing::instrument;

use super::Update;
use crate::schema::xs::{ElementSubstitutionGroupType, ElementType, GroupType};
use crate::schema::Namespace;
use crate::types::{
    DynamicInfo, ElementInfo, ElementMode, Ident, IdentType, Name, ReferenceInfo, Type,
    TypeVariant, VecHelper,
};

use super::super::{Error, SchemaInterpreter};

#[derive(Debug)]
pub(crate) struct ElementBuilder<'a, 'schema, 'state> {
    /// Type variant that is constructed by the builder
    variant: Option<TypeVariant>,

    /// `true` if `type_` is fixed and can not be changed anymore
    fixed: bool,

    owner: &'a mut SchemaInterpreter<'schema, 'state>,
}

/* any type */

/// Initialize the type of a `$builder` to any type `$variant`.
macro_rules! init_any {
    ($builder:expr, $variant:ident) => {
        init_any!($builder, $variant, Default::default(), true)
    };
    ($builder:expr, $variant:ident, $value:expr, $fixed:expr) => {{
        $builder.variant = Some(TypeVariant::$variant($value));
        $builder.fixed = $fixed;

        let TypeVariant::$variant(ret) = $builder.variant.as_mut().unwrap() else {
            crate::unreachable!();
        };

        ret
    }};
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

impl<'a, 'schema, 'state> ElementBuilder<'a, 'schema, 'state> {
    pub(crate) fn new(owner: &'a mut SchemaInterpreter<'schema, 'state>) -> Self {
        Self {
            variant: None,
            fixed: false,
            owner,
        }
    }

    pub(crate) fn finish(self) -> Result<Type, Error> {
        println!("ElementBuilder::finish");
        let variant = self.variant.ok_or(Error::NoType)?;

        Ok(Type::new(variant))
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(crate) fn apply_element(&mut self, ty: &ElementType) -> Result<(), Error> {
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

            let mut type_ = None;
            for c in &ty.content {
                match c {
                    C::Annotation(_)
                    | C::Key(_)
                    | C::Alternative(_)
                    | C::Unique(_)
                    | C::Keyref(_) => {}
                    C::SimpleType(x) => {
                        type_ = Some(self.create_simple_type(
                            ident.ns,
                            Some(ident.name),
                            Some(ident.type_),
                            x,
                        )?);
                        break;
                    }
                    C::ComplexType(x) => {
                        type_ = Some(self.create_complex_type(
                            ident.ns,
                            Some(ident.name),
                            Some(ident.type_),
                            x,
                        )?);
                        break;
                    }
                }
            }

            if let Some(type_) = type_ {
                init_any!(self, Reference, ReferenceInfo::new(type_), true);
            }
        } else {
            let xs = self
                .schemas
                .resolve_namespace(&Some(Namespace::XS))
                .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;
            let ident = Ident::ANY_TYPE.with_ns(Some(xs));

            init_any!(self, Reference, ReferenceInfo::new(ident), true);
        }

        if ty.abstract_ {
            let type_ = match self.variant.take() {
                None => None,
                Some(TypeVariant::Reference(ti)) => Some(ti.type_),
                e => crate::unreachable!("Unexpected type: {:?}", e),
            };

            let ai = init_any!(self, Dynamic);
            ai.type_ = type_;
        }

        if let Some(substitution_group) = &ty.substitution_group {
            self.walk_substitution_groups(substitution_group, |builder, base_ident| {
                let ident = builder.state.current_ident().unwrap().clone();
                let base_ty = builder.get_element_mut(base_ident)?;

                if let TypeVariant::Reference(ti) = &mut base_ty.variant {
                    base_ty.variant = TypeVariant::Dynamic(DynamicInfo {
                        type_: Some(ti.type_.clone()),
                        derived_types: vec![ti.type_.clone()],
                    });
                }

                let TypeVariant::Dynamic(ai) = &mut base_ty.variant else {
                    return Err(Error::ExpectedDynamicElement(base_ident.clone()));
                };

                ai.derived_types.push(ident);

                Ok(())
            })?;
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

    fn walk_substitution_groups<F>(
        &mut self,
        groups: &ElementSubstitutionGroupType,
        mut f: F,
    ) -> Result<(), Error>
    where
        F: FnMut(&mut Self, &Ident) -> Result<(), Error>,
    {
        fn inner<'x, 'y, 'z, F>(
            builder: &mut ElementBuilder<'x, 'y, 'z>,
            groups: &ElementSubstitutionGroupType,
            f: &mut F,
        ) -> Result<(), Error>
        where
            F: FnMut(&mut ElementBuilder<'x, 'y, 'z>, &Ident) -> Result<(), Error>,
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

impl<'schema, 'state> Deref for ElementBuilder<'_, 'schema, 'state> {
    type Target = SchemaInterpreter<'schema, 'state>;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}

impl DerefMut for ElementBuilder<'_, '_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.owner
    }
}
