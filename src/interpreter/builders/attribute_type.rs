use std::ops::{Deref, DerefMut};

use tracing::instrument;

use super::Update;
use crate::schema::xs::AttributeType;
use crate::types::{
    AttributeInfo, ComplexType, ComplexTypeVariant, Ident, IdentType, Name, ReferenceInfo, Type,
    VecHelper,
};

use super::super::{Error, SchemaInterpreter};
use super::SimpleTypeBuilder;

#[derive(Debug)]
pub(crate) struct AttributeTypeBuilder<'a, 'schema, 'state> {
    type_: Option<Type>,

    /// Type variant that is constructed by the builder
    variant: Option<ComplexTypeVariant>,

    /// `true` if `type_` is fixed and can not be changed anymore
    fixed: bool,

    owner: &'a mut SchemaInterpreter<'schema, 'state>,
}

/* any type */

/// Initialize the type of a `$builder` to any type `$variant`.
macro_rules! init_any {
    ($builder:expr, $variant:ident, $value:expr, $fixed:expr) => {{
        $builder.variant = Some(ComplexTypeVariant::$variant($value));
        $builder.fixed = $fixed;

        let ComplexTypeVariant::$variant(ret) = $builder.variant.as_mut().unwrap() else {
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
            Some(ComplexTypeVariant::$variant(ret)) => ret,
            _ if !$builder.fixed => init_any!($builder, $variant, $default, true),
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    };
}

/* TypeBuilder */

impl<'a, 'schema, 'state> AttributeTypeBuilder<'a, 'schema, 'state> {
    pub(crate) fn new(owner: &'a mut SchemaInterpreter<'schema, 'state>) -> Self {
        Self {
            type_: None,
            variant: None,
            fixed: false,
            owner,
        }
    }

    pub(crate) fn finish(self) -> Result<Type, Error> {
        self.type_
            .or_else(|| self.variant.map(ComplexType::new).map(Type::ComplexType))
            .ok_or(Error::NoType)
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(crate) fn apply_attribute(&mut self, ty: &AttributeType) -> Result<(), Error> {
        if let Some(type_) = &ty.type_ {
            let type_ = self.parse_qname(type_)?;
            init_any!(self, Reference, ReferenceInfo::new(type_), false);
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
