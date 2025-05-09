use std::ops::{Deref, DerefMut};

use tracing::instrument;

use super::Update;
use crate::schema::xs::AttributeType;
use crate::types::{
    AttributeInfo, ComplexInfo, ComplexType, ComplexTypeVariant, Ident, IdentType, Name,
    ReferenceInfo, Type, VecHelper,
};

use super::super::{Error, SchemaInterpreter};
use super::SimpleTypeBuilder;

#[derive(Debug)]
pub(crate) struct AttributeTypeBuilder<'a, 'schema, 'state> {
    type_: Option<Type>,

    /// Type variant that is constructed by the builder
    variant: Option<ComplexTypeVariant>,

    owner: &'a mut SchemaInterpreter<'schema, 'state>,
}

/* TypeBuilder */

impl<'a, 'schema, 'state> AttributeTypeBuilder<'a, 'schema, 'state> {
    pub(crate) fn new(owner: &'a mut SchemaInterpreter<'schema, 'state>) -> Self {
        Self {
            type_: None,
            variant: None,
            owner,
        }
    }

    pub(crate) fn finish(self) -> Result<Type, Error> {
        self.type_
            .or_else(|| self.variant.map(ComplexType::new).map(Type::ComplexType))
            .ok_or(Error::NoType)
    }

    fn get_or_init_complex(&mut self) -> &mut ComplexInfo {
        match &mut self.variant {
            Some(ComplexTypeVariant::ComplexType(ci)) => ci,
            a @ None => {
                *a = Some(ComplexTypeVariant::ComplexType(Default::default()));

                match a {
                    Some(ComplexTypeVariant::ComplexType(si)) => si,
                    _ => crate::unreachable!(),
                }
            }
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(crate) fn apply_attribute(&mut self, ty: &AttributeType) -> Result<(), Error> {
        if let Some(type_) = &ty.type_ {
            let type_ = self.parse_qname(type_)?;

            self.variant = Some(ComplexTypeVariant::Reference(ReferenceInfo::new(type_)));
        } else if let Some(x) = &ty.simple_type {
            let mut builder = SimpleTypeBuilder::new(self.owner);
            builder.apply_simple_type(x)?;

            self.type_ = Some(builder.finish()?);
        }

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

                let ci = self.get_or_init_complex();
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

                let ci = self.get_or_init_complex();
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
                            .auto_extend(true, true, self.state)
                            .finish();
                        let ns = self.state.current_ns();

                        self.create_simple_type(ns, Some(type_name), None, x)
                    })
                    .transpose()?;
                let name = Name::from(name.clone());
                let ident = Ident::new(name)
                    .with_ns(self.state.current_ns())
                    .with_type(IdentType::Attribute);

                let ci = self.get_or_init_complex();
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
}

impl<'schema, 'state> Deref for AttributeTypeBuilder<'_, 'schema, 'state> {
    type Target = SchemaInterpreter<'schema, 'state>;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}

impl DerefMut for AttributeTypeBuilder<'_, '_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.owner
    }
}
