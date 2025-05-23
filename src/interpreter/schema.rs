use std::str::from_utf8;

use tracing::instrument;

use crate::misc::RawByteStr;
use crate::schema::xs::{
    AttributeGroupType, AttributeType, ComplexBaseType, ElementType, GroupType, Schema,
    SchemaContent, SimpleBaseType,
};
use crate::schema::{Namespace, NamespaceId, QName, Schemas};
use crate::types::{Ident, IdentType, Name, Type, TypeVariant};

use super::{Error, Node, State, VariantBuilder};

#[derive(Debug)]
pub(super) struct SchemaInterpreter<'schema, 'state> {
    pub(super) state: &'state mut State<'schema>,
    pub(super) schema: &'schema Schema,
    pub(super) schemas: &'schema Schemas,
}

impl<'schema, 'state> SchemaInterpreter<'schema, 'state> {
    #[allow(clippy::unnecessary_literal_unwrap)]
    #[instrument(level = "trace", skip(state, schema, schemas))]
    pub(super) fn process(
        state: &'state mut State<'schema>,
        schema: &'schema Schema,
        schemas: &'schema Schemas,
    ) -> Result<(), Error> {
        let target_namespace = schema
            .target_namespace
            .as_ref()
            .map(|ns| {
                let ns = ns.as_bytes().to_owned();
                let ns = Some(Namespace::from(ns));

                let Some(ns) = schemas.resolve_namespace(&ns) else {
                    return Err(Error::UnknownNamespace(ns.unwrap()));
                };

                Ok(ns)
            })
            .transpose()?;

        let mut this = Self {
            state,
            schema,
            schemas,
        };

        for c in &this.schema.content {
            match c {
                SchemaContent::Annotation(_)
                | SchemaContent::AttributeGroup(_)
                | SchemaContent::DefaultOpenContent(_)
                | SchemaContent::Group(_)
                | SchemaContent::Import(_)
                | SchemaContent::Include(_)
                | SchemaContent::Notation(_)
                | SchemaContent::Override(_)
                | SchemaContent::Redefine(_) => (),
                SchemaContent::Element(x) => {
                    this.create_element(target_namespace, None, x)?;
                }
                SchemaContent::Attribute(x) => {
                    this.create_attribute(target_namespace, None, x)?;
                }
                SchemaContent::SimpleType(x) => {
                    this.create_simple_type(target_namespace, None, x)?;
                }
                SchemaContent::ComplexType(x) => {
                    this.create_complex_type(target_namespace, None, x)?;
                }
            }
        }

        Ok(())
    }
}

/* Get Methods */

impl SchemaInterpreter<'_, '_> {
    #[instrument(level = "trace", skip(self))]
    pub(super) fn get_element_mut(&mut self, ident: &Ident) -> Result<&mut Type, Error> {
        if !self.state.types.contains_key(ident) {
            let ty = self
                .find_element(ident.clone())
                .ok_or_else(|| Error::UnknownType(ident.clone()))?;
            let new_ident = self.create_element(ident.ns, None, ty)?;

            crate::assert_eq!(ident, &new_ident);
        }

        Ok(self.state.types.get_mut(ident).unwrap())
    }

    #[instrument(level = "trace", skip(self))]
    pub(super) fn get_simple_type_variant(&mut self, ident: &Ident) -> Result<&TypeVariant, Error> {
        if !self.state.types.contains_key(ident) {
            let ty = self
                .find_simple_type(ident.clone())
                .ok_or_else(|| Error::UnknownType(ident.clone()))?;
            let new_ident = self.create_simple_type(ident.ns, None, ty)?;

            crate::assert_eq!(ident, &new_ident);
        }

        match self.state.types.get_variant(ident) {
            None
            | Some(
                TypeVariant::ComplexType(_)
                | TypeVariant::All(_)
                | TypeVariant::Choice(_)
                | TypeVariant::Sequence(_)
                | TypeVariant::Dynamic(_),
            ) => Err(Error::UnknownType(ident.clone())),
            Some(
                ty @ (TypeVariant::Enumeration(_)
                | TypeVariant::BuildIn(_)
                | TypeVariant::Custom(_)
                | TypeVariant::Union(_)
                | TypeVariant::Reference(_)),
            ) => Ok(ty),
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(super) fn get_complex_type_variant(
        &mut self,
        ident: &Ident,
    ) -> Result<&TypeVariant, Error> {
        if !self.state.types.contains_key(ident) {
            let ty = self
                .find_complex_type(ident.clone())
                .ok_or_else(|| Error::UnknownType(ident.clone()))?;
            let new_ident = self.create_complex_type(ident.ns, None, ty)?;

            crate::assert_eq!(ident, &new_ident);
        }

        match self.state.types.get_variant(ident) {
            None
            | Some(
                TypeVariant::Enumeration(_)
                | TypeVariant::BuildIn(_)
                | TypeVariant::Custom(_)
                | TypeVariant::Union(_)
                | TypeVariant::Reference(_),
            ) => Err(Error::UnknownType(ident.clone())),
            Some(
                ty @ (TypeVariant::ComplexType(_)
                | TypeVariant::All(_)
                | TypeVariant::Choice(_)
                | TypeVariant::Sequence(_)
                | TypeVariant::Dynamic(_)),
            ) => Ok(ty),
        }
    }
}

/* Create Methods */

impl SchemaInterpreter<'_, '_> {
    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn create_element(
        &mut self,
        ns: Option<NamespaceId>,
        name: Option<Name>,
        ty: &ElementType,
    ) -> Result<Ident, Error> {
        let ident = Ident {
            ns,
            name: self.state.name_builder().or(name).or(&ty.name).finish(),
            type_: IdentType::Element,
        };

        self.create_type(ident, |builder| builder.apply_element(ty))
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn create_attribute(
        &mut self,
        ns: Option<NamespaceId>,
        name: Option<Name>,
        ty: &AttributeType,
    ) -> Result<Ident, Error> {
        let ident = Ident {
            ns,
            name: self.state.name_builder().or(name).or(&ty.name).finish(),
            type_: IdentType::Attribute,
        };

        self.create_type(ident.clone(), |builder| builder.apply_attribute(ty))
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn create_simple_type(
        &mut self,
        ns: Option<NamespaceId>,
        name: Option<Name>,
        ty: &SimpleBaseType,
    ) -> Result<Ident, Error> {
        let ident = Ident {
            ns,
            name: self.state.name_builder().or(name).or(&ty.name).finish(),
            type_: IdentType::Type,
        };

        self.create_type(ident, |builder| builder.apply_simple_type(ty))
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn create_complex_type(
        &mut self,
        ns: Option<NamespaceId>,
        name: Option<Name>,
        ty: &ComplexBaseType,
    ) -> Result<Ident, Error> {
        let ident = Ident {
            ns,
            name: self.state.name_builder().or(name).or(&ty.name).finish(),
            type_: IdentType::Type,
        };

        self.create_type(ident, |builder| builder.apply_complex_type(ty))
    }

    pub(super) fn create_type<F>(&mut self, ident: Ident, mut f: F) -> Result<Ident, Error>
    where
        F: FnMut(&mut VariantBuilder<'_, '_, '_>) -> Result<(), Error>,
    {
        if self.state.types.types.contains_key(&ident)
            || self
                .state
                .type_stack
                .iter()
                .any(|x| x.as_ref() == Some(&ident))
        {
            return Ok(ident);
        }

        self.state.type_stack.push(Some(ident));

        let mut builder = VariantBuilder::new(self);
        let type_ = f(&mut builder).and_then(|()| builder.finish());

        let ident = self.state.type_stack.pop().unwrap().unwrap();

        self.state.add_type(ident.clone(), type_?, false)?;

        Ok(ident)
    }
}

/* Find Methods */

impl<'schema> SchemaInterpreter<'schema, '_> {
    pub(super) fn find_element(&mut self, ident: Ident) -> Option<&'schema ElementType> {
        if let Some(Node::Element(x)) = self.state.get_node(self.schemas, self.schema, ident) {
            Some(x)
        } else {
            None
        }
    }

    pub(super) fn find_simple_type(&mut self, ident: Ident) -> Option<&'schema SimpleBaseType> {
        if let Some(Node::SimpleType(x)) = self.state.get_node(self.schemas, self.schema, ident) {
            Some(x)
        } else {
            None
        }
    }

    pub(super) fn find_complex_type(&mut self, ident: Ident) -> Option<&'schema ComplexBaseType> {
        if let Some(Node::ComplexType(x)) = self.state.get_node(self.schemas, self.schema, ident) {
            Some(x)
        } else {
            None
        }
    }

    pub(super) fn find_group(&mut self, ident: Ident) -> Option<&'schema GroupType> {
        if let Some(Node::Group(x)) = self.state.get_node(self.schemas, self.schema, ident) {
            Some(x)
        } else {
            None
        }
    }

    pub(super) fn find_attribute_group(
        &mut self,
        ident: Ident,
    ) -> Option<&'schema AttributeGroupType> {
        if let Some(Node::AttributeGroup(x)) = self.state.get_node(self.schemas, self.schema, ident)
        {
            Some(x)
        } else {
            None
        }
    }
}

/* Helper Methods */

impl SchemaInterpreter<'_, '_> {
    #[allow(clippy::unnecessary_literal_unwrap)]
    pub(super) fn parse_qname(&self, qname: &QName) -> Result<Ident, Error> {
        let ns = qname
            .namespace()
            .map(|ns| {
                let ns = Some(ns.clone());

                self.schemas
                    .resolve_namespace(&ns)
                    .ok_or_else(|| Error::UnknownNamespace(ns.unwrap()))
            })
            .transpose()?
            .or(self.state.current_ns());
        let name = qname.local_name();
        let name =
            from_utf8(name).map_err(|_| Error::InvalidLocalName(RawByteStr::from_slice(name)))?;
        let name = name.to_owned();

        Ok(Ident {
            name: Name::new_named(name),
            ns,
            type_: IdentType::Type,
        })
    }
}
