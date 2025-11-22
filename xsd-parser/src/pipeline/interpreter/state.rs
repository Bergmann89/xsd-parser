use std::collections::{btree_map::Entry, BTreeMap, HashMap};

use crate::models::{
    meta::{MetaType, MetaTypes},
    schema::{
        xs::{
            AttributeGroupType, ComplexBaseType, ElementType, GroupType, Schema, SchemaContent,
            SimpleBaseType,
        },
        NamespaceId, Schemas,
    },
    Ident, IdentType, Name,
};
use crate::traits::{NameBuilder, NameBuilderExt as _};

use super::{name_builder::NameBuilderExt as _, Error};

#[derive(Default, Debug)]
pub(super) struct State<'a> {
    pub types: MetaTypes,
    pub node_cache: BTreeMap<Ident, Node<'a>>,
    pub type_stack: Vec<StackEntry>,
}

#[derive(Debug)]
pub(super) enum StackEntry {
    Type(Ident, HashMap<Ident, Ident>),
    GroupRef(Ident, Option<String>),
    AttributeGroupRef,
    Mixed(bool),
    Group,
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
        self.type_stack.iter().rev().find_map(|x| {
            if let StackEntry::Type(x, _) = x {
                Some(x)
            } else {
                None
            }
        })
    }

    pub(super) fn group_cache(&self) -> Option<&HashMap<Ident, Ident>> {
        self.type_stack.iter().rev().find_map(|x| {
            if let StackEntry::Type(_, cache) = x {
                Some(cache)
            } else {
                None
            }
        })
    }

    pub(super) fn group_cache_mut(&mut self) -> Option<&mut HashMap<Ident, Ident>> {
        self.type_stack.iter_mut().rev().find_map(|x| {
            if let StackEntry::Type(_, cache) = x {
                Some(cache)
            } else {
                None
            }
        })
    }

    pub(super) fn current_ns(&self) -> Option<NamespaceId> {
        self.current_ident().and_then(|x| x.ns)
    }

    pub(super) fn last_named_type(&self, stop_at_group_ref: bool) -> Option<&str> {
        for x in self.type_stack.iter().rev() {
            match x {
                StackEntry::Type(x, _) if x.name.is_named() => return Some(x.name.as_str()),
                StackEntry::GroupRef(_, _) | StackEntry::AttributeGroupRef if stop_at_group_ref => {
                    return None
                }
                _ => (),
            }
        }

        None
    }

    pub(super) fn named_group(&self) -> Option<String> {
        if let StackEntry::GroupRef(x, prefix) = self.type_stack.last()? {
            let name = x.name.as_str();
            if let Some(prefix) = prefix {
                Some(format!("{prefix}:{name}"))
            } else {
                Some(name.to_owned())
            }
        } else {
            None
        }
    }

    pub(super) fn is_mixed(&self) -> bool {
        self.type_stack
            .iter()
            .rev()
            .find_map(|x| {
                if let StackEntry::Mixed(x) = x {
                    Some(*x)
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    pub(super) fn name_builder(&self) -> Box<dyn NameBuilder> {
        self.types.name_builder()
    }

    pub(super) fn make_content_name(&mut self) -> Name {
        self.name_builder()
            .auto_extend(false, true, self)
            .remove_suffix("Type")
            .remove_suffix("Content")
            .shared_name("Content")
            .finish()
    }

    pub(super) fn add_type<I, T>(
        &mut self,
        ident: I,
        type_: T,
        allow_overwrite: bool,
    ) -> Result<(), Error>
    where
        I: Into<Ident>,
        T: Into<MetaType>,
    {
        let ident = ident.into();
        if !allow_overwrite && self.types.contains_exact_type(&ident) {
            return Err(Error::TypeAlreadyDefined(ident));
        }
        self.types.insert_type(ident, type_.into());
        Ok(())
    }

    pub(super) fn get_node(
        &mut self,
        schemas: &'a Schemas,
        current: NamespaceId,
        ident: Ident,
    ) -> Option<Node<'a>> {
        match self.node_cache.entry(ident) {
            Entry::Occupied(e) => Some(*e.get()),
            Entry::Vacant(e) => {
                let node = search_in_schemas(schemas, current, e.key())?;

                Some(*e.insert(node))
            }
        }
    }
}

fn search_in_schemas<'a>(
    schemas: &'a Schemas,
    current: NamespaceId,
    ident: &Ident,
) -> Option<Node<'a>> {
    let name = ident.name.as_named_str()?;
    let type_ = ident.type_;

    let ns = ident.ns.as_ref().unwrap_or(&current);
    let ns_info = schemas.get_namespace_info(ns)?;

    for id in &ns_info.schemas {
        if let Some(info) = schemas.get_schema(id) {
            if let Some(node) = search_in_schema(&info.schema, name, type_) {
                return Some(node);
            }
        }
    }

    None
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
