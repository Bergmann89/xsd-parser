use std::collections::{btree_map::Entry, BTreeMap};

use crate::schema::{
    xs::{
        AttributeGroupType, ComplexBaseType, ElementType, GroupType, Schema, SchemaContent,
        SimpleBaseType,
    },
    NamespaceId, Schemas,
};
use crate::types::{Ident, IdentType, Name, Type, Types};

use super::Error;

#[derive(Default, Debug)]
pub(super) struct State<'a> {
    pub types: Types,
    pub node_cache: BTreeMap<Ident, Node<'a>>,
    pub type_stack: Vec<Option<Ident>>,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Node<'a> {
    Group(&'a GroupType),
    Element(&'a ElementType),
    SimpleType(&'a SimpleBaseType),
    ComplexType(&'a ComplexBaseType),
    AttributeGroup(&'a AttributeGroupType),
}

impl<'a> State<'a> {
    pub(super) fn current_ident(&self) -> Option<&Ident> {
        self.type_stack.iter().rev().flatten().next()
    }

    pub(super) fn current_ns(&self) -> Option<NamespaceId> {
        self.current_ident().and_then(|x| x.ns)
    }

    pub(super) fn last_named_type(&self, stop_at_group: bool) -> Option<&str> {
        for x in self.type_stack.iter().rev() {
            match x {
                Some(x) if x.name.is_named() => return x.name.as_str(),
                None if stop_at_group => return None,
                _ => (),
            }
        }

        None
    }

    pub(super) fn make_unnamed(&mut self) -> Name {
        self.types.make_unnamed()
    }

    pub(super) fn add_type<I, T>(
        &mut self,
        ident: I,
        type_: T,
        allow_overwrite: bool,
    ) -> Result<(), Error>
    where
        I: Into<Ident>,
        T: Into<Type>,
    {
        match self.types.entry(ident.into()) {
            Entry::Vacant(e) => {
                e.insert(type_.into());

                Ok(())
            }
            Entry::Occupied(mut e) if allow_overwrite => {
                e.insert(type_.into());

                Ok(())
            }
            Entry::Occupied(e) => Err(Error::TypeAlreadyDefined(e.key().clone())),
        }
    }

    pub(super) fn get_node(
        &mut self,
        schemas: &'a Schemas,
        schema: &'a Schema,
        ident: Ident,
    ) -> Option<Node<'a>> {
        match self.node_cache.entry(ident) {
            Entry::Occupied(e) => Some(*e.get()),
            Entry::Vacant(e) => {
                let node = search_in_schemas(schemas, schema, e.key())?;

                Some(*e.insert(node))
            }
        }
    }
}

fn search_in_schemas<'a>(
    schemas: &'a Schemas,
    schema: &'a Schema,
    ident: &Ident,
) -> Option<Node<'a>> {
    let name = ident.name.as_str()?;
    let type_ = ident.type_;

    if let Some(ns) = &ident.ns {
        let ns_info = schemas.get_namespace_info(ns)?;

        for id in &ns_info.schemas {
            if let Some(schema) = schemas.get_schema(id) {
                if let Some(node) = search_in_schema(schema, name, type_) {
                    return Some(node);
                }
            }
        }

        None
    } else {
        search_in_schema(schema, name, type_)
    }
}

fn search_in_schema<'a>(schema: &'a Schema, name: &str, type_: IdentType) -> Option<Node<'a>> {
    for c in &schema.content {
        match (type_, c) {
            (IdentType::Element, SchemaContent::Element(x)) if matches!(&x.name, Some(n) if n == name) =>
            {
                return Some(Node::Element(x));
            }
            (IdentType::Type, SchemaContent::SimpleType(x)) if matches!(&x.name, Some(n) if n == name) =>
            {
                return Some(Node::SimpleType(x));
            }
            (IdentType::Type, SchemaContent::ComplexType(x)) if matches!(&x.name, Some(n) if n == name) =>
            {
                return Some(Node::ComplexType(x));
            }
            (IdentType::Group, SchemaContent::Group(x)) if matches!(&x.name, Some(n) if n == name) =>
            {
                return Some(Node::Group(x));
            }
            (IdentType::AttributeGroup, SchemaContent::AttributeGroup(x)) if matches!(&x.name, Some(n) if n == name) =>
            {
                return Some(Node::AttributeGroup(x));
            }
            (_, _) => (),
        }
    }

    None
}
