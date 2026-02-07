use std::collections::{btree_map::Entry, BTreeMap, HashMap};

use crate::models::{
    meta::{ElementMetaVariant, MetaType, MetaTypeVariant, MetaTypes, ModuleMeta, SchemaMeta},
    schema::{
        xs::{
            AttributeGroupType, AttributeType, ComplexBaseType, ElementType, GroupType, Schema,
            SchemaContent, SimpleBaseType,
        },
        NamespaceId, SchemaId, Schemas,
    },
    IdentCache, IdentType, Name, NodeIdent, TypeIdent,
};
use crate::traits::{NameBuilder, NameBuilderExt as _, Naming};

use super::{name_builder::NameBuilderExt as _, Error, SchemaInterpreter};

#[derive(Debug)]
pub(super) struct State<'a> {
    types: MetaTypes,
    type_stack: Vec<StackEntry>,
    node_cache: BTreeMap<(SchemaId, NodeIdent), (SchemaId, Node<'a>)>,
    ident_cache: IdentCache,
}

#[derive(Debug)]
pub(super) enum StackEntry {
    Type(TypeIdent, HashMap<NodeIdent, TypeIdent>),
    GroupRef(NodeIdent, Option<String>),
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
    pub(super) fn new(schemas: &'a Schemas) -> Self {
        let mut ret = Self {
            types: MetaTypes::default(),
            type_stack: Vec::new(),
            node_cache: BTreeMap::default(),
            ident_cache: IdentCache::default(),
        };

        ret.create_ident_cache(schemas);

        ret
    }

    pub(super) fn finish(mut self, schemas: &'a Schemas) -> Result<(MetaTypes, IdentCache), Error> {
        self.prepare_modules(schemas);
        self.process_schemas(schemas)?;

        Ok((self.types, self.ident_cache))
    }

    pub(super) fn types(&self) -> &MetaTypes {
        &self.types
    }

    pub(super) fn type_stack(&self) -> &Vec<StackEntry> {
        &self.type_stack
    }

    pub(super) fn push_stack(&mut self, entry: StackEntry) {
        self.type_stack.push(entry);
    }

    pub(super) fn pop_stack(&mut self) -> StackEntry {
        self.type_stack.pop().unwrap()
    }

    pub(super) fn set_naming(&mut self, naming: Box<dyn Naming>) {
        self.types.naming = naming;
    }

    pub(super) fn group_cache(&self) -> Option<&HashMap<NodeIdent, TypeIdent>> {
        self.type_stack.iter().rev().find_map(|x| {
            if let StackEntry::Type(_, cache) = x {
                Some(cache)
            } else {
                None
            }
        })
    }

    pub(super) fn group_cache_mut(&mut self) -> Option<&mut HashMap<NodeIdent, TypeIdent>> {
        self.type_stack.iter_mut().rev().find_map(|x| {
            if let StackEntry::Type(_, cache) = x {
                Some(cache)
            } else {
                None
            }
        })
    }

    pub(super) fn current_ident(&self) -> Option<&TypeIdent> {
        self.type_stack.iter().rev().find_map(|x| {
            if let StackEntry::Type(x, _) = x {
                Some(x)
            } else {
                None
            }
        })
    }

    pub(super) fn current_ns(&self) -> Option<NamespaceId> {
        self.type_stack.iter().rev().find_map(|x| match x {
            StackEntry::Type(x, _) => Some(x.ns),
            _ => None,
        })
    }

    pub(super) fn current_schema(&self) -> Option<SchemaId> {
        self.type_stack.iter().rev().find_map(|x| match x {
            StackEntry::Type(x, _) => Some(x.schema),
            _ => None,
        })
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
            .content_type_name()
            .shared_name("Content")
            .finish()
    }

    pub(super) fn get_type(&self, ident: &TypeIdent) -> Option<&MetaType> {
        self.types.items.get(ident)
    }

    pub(super) fn get_variant(&self, ident: &TypeIdent) -> Option<&MetaTypeVariant> {
        self.types.get_variant(ident)
    }

    pub(super) fn get_type_mut(&mut self, ident: &TypeIdent) -> Option<&mut MetaType> {
        self.types.items.get_mut(ident)
    }

    pub(super) fn add_type<I, T>(
        &mut self,
        ident: I,
        type_: T,
        allow_overwrite: bool,
        resolve_idents: bool,
    ) -> Result<(), Error>
    where
        I: Into<TypeIdent>,
        T: Into<MetaType>,
    {
        let mut ident = ident.into();
        let mut type_ = type_.into();

        if resolve_idents {
            let resolved = self.resolve_type_ident_allow_unknown(ident)?;

            self.push_stack(StackEntry::Type(resolved, HashMap::new()));
            self.resolve_type_idents(&mut type_)?;

            let StackEntry::Type(resolved, _) = self.pop_stack() else {
                crate::unreachable!();
            };

            ident = resolved;
        }

        let entry = self.types.items.entry(ident);
        if !allow_overwrite && matches!(&entry, Entry::Occupied(_)) {
            return Err(Error::TypeAlreadyDefined(entry.key().clone()));
        }

        let ident = entry.key().clone();
        let ns = ident.ns;
        let schema = ident.schema;
        self.ident_cache.insert(ident, ns, schema);

        match entry {
            Entry::Vacant(e) => {
                e.insert(type_);
            }
            Entry::Occupied(mut e) => {
                e.insert(type_);
            }
        }

        Ok(())
    }

    pub(super) fn get_node(
        &mut self,
        schemas: &'a Schemas,
        schema: SchemaId,
        ident: NodeIdent,
    ) -> Result<Option<(SchemaId, Node<'a>)>, Error> {
        match self.node_cache.entry((schema, ident)) {
            Entry::Occupied(e) => Ok(Some(*e.get())),
            Entry::Vacant(e) => {
                if let Some((schema, node)) = search_in_schemas(schemas, schema, &e.key().1)? {
                    Ok(Some(*e.insert((schema, node))))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub(super) fn resolve_type_ident_allow_unknown(
        &self,
        ident: TypeIdent,
    ) -> Result<TypeIdent, Error> {
        match self.resolve_type_ident(ident) {
            Ok(ident) => Ok(ident),
            Err(Error::UnknownType(ident)) => Ok(ident),
            Err(error) => Err(error),
        }
    }

    pub(super) fn resolve_type_ident(&self, mut ident: TypeIdent) -> Result<TypeIdent, Error> {
        if ident.schema.is_unknown() {
            if let Some(schema) = self.current_schema() {
                ident.schema = schema;

                if let Some(entry) = self.ident_cache.get(&ident) {
                    return entry.resolve(ident);
                }

                ident.schema = SchemaId::UNKNOWN;
            }
        }

        if let Some(entry) = self.ident_cache.get(&ident) {
            return entry.resolve(ident);
        }

        Err(Error::UnknownType(ident))
    }

    /// This method is used to patch types that were defined by the user
    /// using the config that type identifiers are maybe not fully qualified.
    /// The unqualified identifiers are resolved by this method. For type
    /// generated by the interpreter this is usually not necessary.
    fn resolve_type_idents(&self, type_: &mut MetaType) -> Result<(), Error> {
        match &mut type_.variant {
            MetaTypeVariant::Union(x) => {
                for ty in &mut *x.types {
                    ty.type_ = self.resolve_type_ident_allow_unknown(ty.type_.clone())?;
                }
            }
            MetaTypeVariant::Enumeration(x) => {
                for var in &mut *x.variants {
                    if let Some(ty) = &mut var.type_ {
                        *ty = self.resolve_type_ident_allow_unknown(ty.clone())?;
                    }
                }
            }
            MetaTypeVariant::Dynamic(x) => {
                if let Some(ty) = &mut x.type_ {
                    *ty = self.resolve_type_ident_allow_unknown(ty.clone())?;
                }

                for meta in &mut x.derived_types {
                    meta.type_ = self.resolve_type_ident_allow_unknown(meta.type_.clone())?;
                }
            }
            MetaTypeVariant::Reference(x) => {
                x.type_ = self.resolve_type_ident_allow_unknown(x.type_.clone())?;
            }
            MetaTypeVariant::All(x) | MetaTypeVariant::Choice(x) | MetaTypeVariant::Sequence(x) => {
                for el in &mut *x.elements {
                    if let ElementMetaVariant::Type { type_, .. } = &mut el.variant {
                        *type_ = self.resolve_type_ident_allow_unknown(type_.clone())?;
                    }
                }
            }
            MetaTypeVariant::ComplexType(x) => {
                if let Some(ty) = &mut x.content {
                    *ty = self.resolve_type_ident_allow_unknown(ty.clone())?;
                }
            }
            MetaTypeVariant::SimpleType(x) => {
                x.base = self.resolve_type_ident_allow_unknown(x.base.clone())?;
            }
            _ => (),
        }

        Ok(())
    }

    fn create_ident_cache(&mut self, schemas: &'a Schemas) {
        for (schema, info) in &schemas.schemas {
            let ns = info.namespace_id;
            let schema = *schema;

            for c in &info.schema.content {
                let (type_, name) = match c {
                    SchemaContent::Element(ElementType {
                        name: Some(name), ..
                    }) => (IdentType::Element, Name::new_named(name.clone())),
                    SchemaContent::Attribute(AttributeType {
                        name: Some(name), ..
                    }) => (IdentType::Attribute, Name::new_named(name.clone())),
                    SchemaContent::SimpleType(SimpleBaseType {
                        name: Some(name), ..
                    }) => (IdentType::Type, Name::new_named(name.clone())),
                    SchemaContent::ComplexType(ComplexBaseType {
                        name: Some(name), ..
                    }) => (IdentType::Type, Name::new_named(name.clone())),
                    _ => continue,
                };

                let ident = TypeIdent {
                    ns,
                    schema,
                    name,
                    type_,
                };

                self.ident_cache.insert(ident, ns, schema);
            }
        }
    }

    fn prepare_modules(&mut self, schemas: &Schemas) {
        for (id, info) in schemas.namespaces() {
            let prefix = info
                .prefix
                .as_ref()
                .map(ToString::to_string)
                .map(Name::new_named);
            let name = info.name().map(Name::new_named);
            let namespace = info.namespace.clone();
            let schema_count = info.schemas.len();

            let module = ModuleMeta {
                name,
                prefix,
                namespace,
                namespace_id: *id,
                schema_count,
            };

            self.types.modules.insert(*id, module);
        }
    }

    fn process_schemas(&mut self, schemas: &'a Schemas) -> Result<(), Error> {
        for (id, info) in schemas.schemas() {
            let schema = SchemaMeta {
                name: info.name.clone().map(Name::new_named),
                namespace: info.namespace_id(),
            };

            self.types.schemas.insert(*id, schema);

            SchemaInterpreter::process(self, &info.schema, schemas, *id, info.namespace_id())?;
        }

        Ok(())
    }
}

fn search_in_schemas<'a>(
    schemas: &'a Schemas,
    schema: SchemaId,
    ident: &NodeIdent,
) -> Result<Option<(SchemaId, Node<'a>)>, Error> {
    let Some(name) = ident.name.as_named_str() else {
        return Ok(None);
    };

    let type_ = ident.type_;

    let Some(ns_info) = schemas.get_namespace_info(&ident.ns) else {
        return Ok(None);
    };

    let mut err = false;
    let mut ret = None;

    for id in &ns_info.schemas {
        if let Some(info) = schemas.get_schema(id) {
            if let Some(node) = search_in_schema(&info.schema, name, type_) {
                if *id == schema {
                    // If the node was defined in the current schema we
                    // can ignore any other node we might find in other
                    // schemas and return instantly.
                    return Ok(Some((*id, node)));
                }

                if ret.is_some() {
                    err = true;
                    // If the node was already set, we have more than one
                    // matching type, so we should return an error, but we
                    // still continue if we find another matching node in the
                    // current schema
                } else {
                    ret = Some((*id, node));
                }
            }
        }
    }

    if err {
        Err(Error::AmbiguousNode(ident.clone()))
    } else {
        Ok(ret)
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
