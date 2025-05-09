use std::ops::{Deref, DerefMut};

use tracing::instrument;

use crate::schema::xs::{ElementSubstitutionGroupType, ElementType};
use crate::schema::Namespace;
use crate::types::{
    ComplexType, ComplexTypeVariant, DynamicInfo, Ident, IdentType, ReferenceInfo,
    SimpleTypeVariant, Type, TypeDescriptor,
};

use super::super::{Error, SchemaInterpreter};

#[derive(Debug)]
pub(crate) struct ElementBuilder<'a, 'schema, 'state> {
    /// Type variant that is constructed by the builder
    variant: Option<ComplexTypeVariant>,

    owner: &'a mut SchemaInterpreter<'schema, 'state>,
}

/* TypeBuilder */

impl ElementBuilder<'_, '_, '_> {
    fn init_any(&mut self, variant: ComplexTypeVariant) -> &mut ComplexTypeVariant {
        self.variant = Some(variant);
        self.variant.as_mut().unwrap()
    }
}

impl<'a, 'schema, 'state> ElementBuilder<'a, 'schema, 'state> {
    pub(crate) fn new(owner: &'a mut SchemaInterpreter<'schema, 'state>) -> Self {
        Self {
            variant: None,
            owner,
        }
    }

    pub(crate) fn finish(self) -> Result<Type, Error> {
        self.variant
            .map(ComplexType::new)
            .map(Type::ComplexType)
            .ok_or(Error::NoType)
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(crate) fn apply_element(&mut self, ty: &ElementType) -> Result<(), Error> {
        use crate::schema::xs::ElementTypeContent as C;

        if let Some(type_) = &ty.type_ {
            let type_ = self.parse_qname(type_)?;

            self.init_any(ComplexTypeVariant::Reference(ReferenceInfo::new(type_)));
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
                self.init_any(ComplexTypeVariant::Reference(ReferenceInfo::new(type_)));
            }
        } else {
            let xs = self
                .schemas
                .resolve_namespace(&Some(Namespace::XS))
                .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;
            let ident = Ident::ANY_TYPE.with_ns(Some(xs));

            self.init_any(ComplexTypeVariant::Reference(ReferenceInfo::new(ident)));
        }

        if ty.abstract_ {
            let type_ = match self.variant.take() {
                None => None,
                Some(ComplexTypeVariant::Reference(ti)) => Some(ti.type_),
                e => crate::unreachable!("Unexpected type: {:?}", e),
            };

            let ComplexTypeVariant::Dynamic(ai) =
                self.init_any(ComplexTypeVariant::Dynamic(Default::default()))
            else {
                crate::unreachable!();
            };
            ai.type_ = type_;
        }

        if let Some(substitution_group) = &ty.substitution_group {
            self.walk_substitution_groups(substitution_group, |builder, base_ident| {
                let ident = builder.state.current_ident().unwrap().clone();
                let base_ty = builder.get_element_mut(base_ident)?;

                if let Type::ComplexType(TypeDescriptor {
                    display_name,
                    variant: ComplexTypeVariant::Reference(ti),
                    ..
                })
                | Type::SimpleType(TypeDescriptor {
                    display_name,
                    variant: SimpleTypeVariant::Reference(ti),
                    ..
                }) = base_ty
                {
                    let dyn_info = DynamicInfo {
                        type_: Some(ti.type_.clone()),
                        derived_types: vec![ti.type_.clone()],
                    };

                    let display_name = display_name.take();

                    *base_ty = Type::ComplexType(TypeDescriptor {
                        display_name,
                        variant: ComplexTypeVariant::Dynamic(dyn_info),
                    });
                }

                let Type::ComplexType(TypeDescriptor {
                    variant: ComplexTypeVariant::Dynamic(ai),
                    ..
                }) = base_ty
                else {
                    return Err(Error::ExpectedDynamicElement(base_ident.clone()));
                };

                ai.derived_types.push(ident);

                Ok(())
            })?;
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
