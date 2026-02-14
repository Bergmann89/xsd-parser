use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::str::from_utf8;

use tracing::instrument;

use xsd_parser_types::misc::{Namespace, RawByteStr};

use crate::models::{
    meta::{BuildInMeta, MetaType, MetaTypeVariant},
    schema::{
        xs::{
            AttributeGroupType, AttributeType, ComplexBaseType, ElementType, GroupType, Schema,
            SchemaContent, SimpleBaseType,
        },
        NamespaceId, QName, SchemaId, Schemas,
    },
    IdentType, Name, TypeIdent,
};
use crate::traits::NameBuilderExt as _;

use super::{state::StackEntry, Error, Node, State, VariantBuilder};

#[derive(Debug)]
pub(super) struct SchemaInterpreter<'schema, 'state> {
    pub(super) state: &'state mut State<'schema>,
    pub(super) schema: &'schema Schema,
    pub(super) schemas: &'schema Schemas,
    pub(super) schema_id: SchemaId,
    pub(super) namespace_id: NamespaceId,

    pending_element_types: VecDeque<(TypeIdent, &'schema ElementType)>,
}

impl<'schema, 'state> SchemaInterpreter<'schema, 'state> {
    #[allow(clippy::unnecessary_literal_unwrap)]
    #[instrument(level = "trace", skip(state, schema, schemas))]
    pub(super) fn process(
        state: &'state mut State<'schema>,
        schema: &'schema Schema,
        schemas: &'schema Schemas,
        schema_id: SchemaId,
        namespace_id: NamespaceId,
    ) -> Result<(), Error> {
        let mut this = Self {
            state,
            schema,
            schemas,
            schema_id,
            namespace_id,
            pending_element_types: VecDeque::new(),
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
                    this.create_element(namespace_id, schema_id, None, x)?;
                }
                SchemaContent::Attribute(x) => {
                    this.create_attribute(namespace_id, schema_id, None, x)?;
                }
                SchemaContent::SimpleType(x) => {
                    this.create_simple_type(namespace_id, schema_id, None, x)?;
                }
                SchemaContent::ComplexType(x) => {
                    this.create_complex_type(namespace_id, schema_id, None, x)?;
                }
            }
        }

        while let Some((ident, ty)) = this.pending_element_types.pop_front() {
            this.create_type(ident, |builder| builder.apply_element(ty, false))?;
        }

        Ok(())
    }
}

/* Get Methods */

impl SchemaInterpreter<'_, '_> {
    #[instrument(level = "trace", skip(self))]
    pub(super) fn get_substitution_group_element_mut(
        &mut self,
        mut ident: TypeIdent,
    ) -> Result<&mut MetaType, Error> {
        if self.state.get_type(&ident).is_none() {
            let ty = self.find_element(ident.clone())?;
            ident = self.create_element(ident.ns, ident.schema, None, ty)?;
        }

        Ok(self.state.get_type_mut(&ident).unwrap())
    }

    #[instrument(level = "trace", skip(self))]
    pub(super) fn get_simple_type_variant(
        &mut self,
        mut ident: TypeIdent,
    ) -> Result<(TypeIdent, Cow<'_, MetaTypeVariant>), Error> {
        if self.state.get_type(&ident).is_none() {
            let ty = self.find_simple_type(ident.clone())?;
            ident = self.create_simple_type(ident.ns, ident.schema, None, ty)?;
        }

        match self.state.get_variant(&ident) {
            Some(ty) if ty.is_mixed(self.state.types()) && ty.is_emptiable(self.state.types()) => {
                Ok((
                    ident,
                    Cow::Owned(MetaTypeVariant::BuildIn(BuildInMeta::String)),
                ))
            }
            None
            | Some(
                MetaTypeVariant::ComplexType(_)
                | MetaTypeVariant::All(_)
                | MetaTypeVariant::Choice(_)
                | MetaTypeVariant::Sequence(_)
                | MetaTypeVariant::Dynamic(_),
            ) => Err(Error::UnknownNode(ident.clone())),
            Some(
                ty @ (MetaTypeVariant::Enumeration(_)
                | MetaTypeVariant::BuildIn(_)
                | MetaTypeVariant::Custom(_)
                | MetaTypeVariant::Union(_)
                | MetaTypeVariant::Reference(_)
                | MetaTypeVariant::SimpleType(_)),
            ) => Ok((ident, Cow::Borrowed(ty))),
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(super) fn get_complex_type_variant(
        &mut self,
        mut ident: TypeIdent,
    ) -> Result<(TypeIdent, &MetaTypeVariant), Error> {
        if self.state.get_type(&ident).is_none() {
            let ty = self.find_complex_type(ident.clone())?;
            ident = self.create_complex_type(ident.ns, ident.schema, None, ty)?;
        }

        match self.state.get_variant(&ident) {
            None
            | Some(
                MetaTypeVariant::Enumeration(_)
                | MetaTypeVariant::BuildIn(_)
                | MetaTypeVariant::Custom(_)
                | MetaTypeVariant::Union(_)
                | MetaTypeVariant::Reference(_)
                | MetaTypeVariant::SimpleType(_),
            ) => Err(Error::UnknownNode(ident.clone())),
            Some(
                ty @ (MetaTypeVariant::ComplexType(_)
                | MetaTypeVariant::All(_)
                | MetaTypeVariant::Choice(_)
                | MetaTypeVariant::Sequence(_)
                | MetaTypeVariant::Dynamic(_)),
            ) => Ok((ident, ty)),
        }
    }
}

/* Create Methods */

impl<'schema> SchemaInterpreter<'schema, '_> {
    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn create_element(
        &mut self,
        ns: NamespaceId,
        schema: SchemaId,
        name: Option<Name>,
        ty: &'schema ElementType,
    ) -> Result<TypeIdent, Error> {
        let ident = TypeIdent {
            ns,
            schema,
            name: self.state.name_builder().or(name).or(&ty.name).finish(),
            type_: IdentType::Element,
        };

        self.create_type(ident, |builder| builder.apply_element(ty, true))
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn create_element_lazy(
        &mut self,
        ns: NamespaceId,
        name: Option<Name>,
        ty: &'schema ElementType,
    ) -> Result<TypeIdent, Error> {
        let ident = TypeIdent {
            ns,
            name: self.state.name_builder().or(name).or(&ty.name).finish(),
            type_: IdentType::Element,
            schema: self.current_schema(),
        };

        self.pending_element_types.push_back((ident.clone(), ty));

        Ok(ident)
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn create_attribute(
        &mut self,
        ns: NamespaceId,
        schema: SchemaId,
        name: Option<Name>,
        ty: &'schema AttributeType,
    ) -> Result<TypeIdent, Error> {
        let ident = TypeIdent {
            ns,
            schema,
            name: self.state.name_builder().or(name).or(&ty.name).finish(),
            type_: IdentType::Attribute,
        };

        self.create_type(ident.clone(), |builder| builder.apply_attribute(ty))
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn create_simple_type(
        &mut self,
        ns: NamespaceId,
        schema: SchemaId,
        name: Option<Name>,
        ty: &'schema SimpleBaseType,
    ) -> Result<TypeIdent, Error> {
        let ident = TypeIdent {
            ns,
            schema,
            name: self.state.name_builder().or(name).or(&ty.name).finish(),
            type_: IdentType::Type,
        };

        self.create_type(ident, |builder| builder.apply_simple_type(ty))
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(super) fn create_complex_type(
        &mut self,
        ns: NamespaceId,
        schema: SchemaId,
        name: Option<Name>,
        ty: &'schema ComplexBaseType,
    ) -> Result<TypeIdent, Error> {
        let ident = TypeIdent {
            ns,
            schema,
            name: self.state.name_builder().or(name).or(&ty.name).finish(),
            type_: IdentType::Type,
        };

        self.create_type(ident, |builder| builder.apply_complex_type(ty))
    }

    pub(super) fn create_type<F>(&mut self, ident: TypeIdent, mut f: F) -> Result<TypeIdent, Error>
    where
        F: FnMut(&mut VariantBuilder<'_, 'schema, '_>) -> Result<(), Error>,
    {
        if self.state.types().items.contains_key(&ident)
            || self
                .state
                .type_stack()
                .iter()
                .any(|x| matches!(x, StackEntry::Type(x, _) if x == &ident))
        {
            return Ok(ident);
        }

        self.state
            .push_stack(StackEntry::Type(ident, HashMap::new()));

        let mut builder = VariantBuilder::new(self);
        let type_ = f(&mut builder).and_then(|()| builder.finish());

        let StackEntry::Type(ident, _) = self.state.pop_stack() else {
            unreachable!("Unexpected stack entry!");
        };

        self.state.add_type(ident.clone(), type_?, false, false)?;

        Ok(ident)
    }
}

/* Find Methods */

macro_rules! impl_find_node {
    ($name:ident, $result:ty, $variant:ident) => {
        pub(super) fn $name(&mut self, ident: TypeIdent) -> Result<&'schema $result, Error> {
            match self.state.get_node(&ident) {
                Some(Node::$variant(x)) => Ok(x),
                _ => Err(Error::UnknownNode(ident)),
            }
        }
    };
}

impl<'schema> SchemaInterpreter<'schema, '_> {
    impl_find_node!(find_element, ElementType, Element);
    impl_find_node!(find_simple_type, SimpleBaseType, SimpleType);
    impl_find_node!(find_complex_type, ComplexBaseType, ComplexType);
    impl_find_node!(find_group, GroupType, Group);
    impl_find_node!(find_attribute_group, AttributeGroupType, AttributeGroup);
}

/* Helper Methods */

impl SchemaInterpreter<'_, '_> {
    pub(super) fn current_ns(&self) -> NamespaceId {
        self.state.current_ns().unwrap_or(self.namespace_id)
    }

    pub(super) fn current_schema(&self) -> SchemaId {
        self.state.current_schema().unwrap_or(self.schema_id)
    }

    pub(super) fn parse_type_ident(
        &self,
        qname: &QName,
        type_: IdentType,
    ) -> Result<TypeIdent, Error> {
        let ns = qname
            .namespace()
            .map(|ns| {
                let ns = Some(ns.clone());

                #[allow(clippy::unnecessary_literal_unwrap)]
                self.schemas
                    .resolve_namespace(&ns)
                    .ok_or_else(|| Error::UnknownNamespace(ns.unwrap()))
            })
            .transpose()?
            .unwrap_or_else(|| self.current_ns());

        let name = qname.local_name();
        let name =
            from_utf8(name).map_err(|_| Error::InvalidLocalName(RawByteStr::from_slice(name)))?;
        let name = name.to_owned();

        Ok(TypeIdent {
            ns,
            schema: SchemaId::UNKNOWN,
            name: Name::new_named(name),
            type_,
        })
    }

    pub(super) fn resolve_type_ident(
        &self,
        qname: &QName,
        type_: IdentType,
    ) -> Result<TypeIdent, Error> {
        let ident = self.parse_type_ident(qname, type_)?;

        self.state.resolve_type_ident(ident)
    }

    pub(super) fn resolve_xs_type(&self, name: Name) -> Result<TypeIdent, Error> {
        let ns = self
            .schemas
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;

        let ident = TypeIdent {
            ns,
            schema: SchemaId::UNKNOWN,
            name,
            type_: IdentType::Type,
        };

        self.state.resolve_type_ident(ident)
    }
}
