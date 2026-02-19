use std::collections::{HashMap, VecDeque};
use std::{borrow::Cow, collections::btree_map::Entry};

use tracing::instrument;
use xsd_parser_types::{misc::Namespace, xml::QName};

use crate::models::meta::{BuildInMeta, MetaType, MetaTypeVariant, MetaTypes};
use crate::models::schema::{xs::Schema, NamespaceId, SchemaId, Schemas};
use crate::models::{IdentCache, IdentType, Name, TypeIdent};
use crate::traits::{NameBuilder, NameBuilderExt as _};

use super::super::{parse_qname, Error, Node, NodeCache, NodeCacheEntry, NodeDependencyKey};
use super::{NameBuilderExt as _, StackEntry, VariantProcessor};

pub(super) struct TypeProcessor<'state, 'schema> {
    stack: Vec<StackEntry<'state, 'schema>>,
    pending_nodes: VecDeque<(TypeIdent, &'state NodeCacheEntry<'schema>, Node<'schema>)>,

    schemas: &'schema Schemas,

    types: &'state mut MetaTypes,
    node_cache: &'state NodeCache<'schema>,
    ident_cache: &'state mut IdentCache,
}

impl<'state, 'schema> TypeProcessor<'state, 'schema> {
    pub(super) fn new(
        schemas: &'schema Schemas,
        types: &'state mut MetaTypes,
        node_cache: &'state NodeCache<'schema>,
        ident_cache: &'state mut IdentCache,
    ) -> Self {
        Self {
            stack: Vec::new(),
            pending_nodes: VecDeque::new(),

            schemas,
            types,
            node_cache,
            ident_cache,
        }
    }

    pub(super) fn process_ident(&mut self, ident: TypeIdent) -> Result<(), Error> {
        let Some(entry) = self.node_cache.get(&ident) else {
            return Ok(());
        };

        self.push_stack(StackEntry::NodeEntry { entry });
        self.process_node(ident, entry.node)?;
        self.pop_stack();

        while let Some((ident, entry, node)) = self.pending_nodes.pop_front() {
            self.push_stack(StackEntry::NodeEntry { entry });
            self.process_node(ident, node)?;
            self.pop_stack();
        }

        Ok(())
    }

    pub(super) fn process_node(
        &mut self,
        ident: TypeIdent,
        node: Node<'schema>,
    ) -> Result<(), Error> {
        match node {
            Node::Element(ty) => {
                self.create_type(ident, |processor| processor.apply_element(ty, true))?;
            }
            Node::Attribute(ty) => {
                self.create_type(ident, |processor| processor.apply_attribute(ty))?;
            }
            Node::SimpleType(ty) => {
                self.create_type(ident, |processor| processor.apply_simple_type(ty))?;
            }
            Node::ComplexType(ty) => {
                self.create_type(ident, |processor| processor.apply_complex_type(ty))?;
            }
            Node::Group(_) | Node::AttributeGroup(_) => (),
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    pub(super) fn get_substitution_group_element_mut(
        &mut self,
        ident: &TypeIdent,
    ) -> &mut MetaType {
        self.get_type_mut(ident)
    }

    #[instrument(level = "trace", skip(self))]
    pub(super) fn get_simple_type_variant(
        &mut self,
        ident: &TypeIdent,
    ) -> Option<Cow<'_, MetaTypeVariant>> {
        match self.get_variant(ident) {
            ty if ty.is_mixed(self.types) && ty.is_emptiable(self.types) => {
                Some(Cow::Owned(MetaTypeVariant::BuildIn(BuildInMeta::String)))
            }
            MetaTypeVariant::ComplexType(_)
            | MetaTypeVariant::All(_)
            | MetaTypeVariant::Choice(_)
            | MetaTypeVariant::Sequence(_)
            | MetaTypeVariant::Dynamic(_) => None,
            ty @ (MetaTypeVariant::Enumeration(_)
            | MetaTypeVariant::BuildIn(_)
            | MetaTypeVariant::Custom(_)
            | MetaTypeVariant::Union(_)
            | MetaTypeVariant::Reference(_)
            | MetaTypeVariant::SimpleType(_)) => Some(Cow::Borrowed(ty)),
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub(super) fn get_complex_type_variant(
        &mut self,
        ident: &TypeIdent,
    ) -> Option<&MetaTypeVariant> {
        match self.get_variant(ident) {
            MetaTypeVariant::Enumeration(_)
            | MetaTypeVariant::BuildIn(_)
            | MetaTypeVariant::Custom(_)
            | MetaTypeVariant::Union(_)
            | MetaTypeVariant::Reference(_)
            | MetaTypeVariant::SimpleType(_) => None,
            ty @ (MetaTypeVariant::ComplexType(_)
            | MetaTypeVariant::All(_)
            | MetaTypeVariant::Choice(_)
            | MetaTypeVariant::Sequence(_)
            | MetaTypeVariant::Dynamic(_)) => Some(ty),
        }
    }

    pub(super) fn create_type<F>(&mut self, ident: TypeIdent, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut VariantProcessor<'_, 'state, 'schema>) -> Result<(), Error>,
    {
        if self.types.items.contains_key(&ident) || self.type_is_active(&ident) {
            return Ok(());
        }

        self.push_stack(StackEntry::Type {
            ident,
            group_cache: HashMap::new(),
        });

        let mut variant_processor = VariantProcessor::new(self);
        let ret = f(&mut variant_processor).and_then(|()| variant_processor.finish());

        let StackEntry::Type { ident, .. } = self.pop_stack() else {
            unreachable!("Unexpected stack entry");
        };

        let type_ = ret?;
        self.add_type(ident, type_, false)?;

        Ok(())
    }

    pub(super) fn get_type(&self, ident: &TypeIdent) -> &MetaType {
        match self.types.items.get(ident) {
            Some(ty) => ty,
            None => {
                // Hard dependencies should be available here
                panic!("Unable to get type for {ident}");
            }
        }
    }

    pub(super) fn get_type_mut(&mut self, ident: &TypeIdent) -> &mut MetaType {
        match self.types.items.get_mut(ident) {
            Some(ty) => ty,
            None => {
                // Hard dependencies should be available here
                panic!("Unable to get type for {ident}");
            }
        }
    }

    pub(super) fn get_variant(&self, ident: &TypeIdent) -> &MetaTypeVariant {
        &self.get_type(ident).variant
    }

    pub(super) fn add_type(
        &mut self,
        ident: TypeIdent,
        type_: MetaType,
        allow_overwrite: bool,
    ) -> Result<(), Error> {
        let entry = self.types.items.entry(ident);
        if !allow_overwrite && matches!(&entry, Entry::Occupied(_)) {
            return Err(Error::TypeAlreadyDefined(entry.key().clone()));
        }

        let ident = entry.key().clone();
        self.ident_cache.insert(ident);

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

    pub(super) fn stack(&self) -> &[StackEntry<'state, 'schema>] {
        &self.stack
    }

    pub(super) fn push_stack(&mut self, entry: StackEntry<'state, 'schema>) {
        self.stack.push(entry);
    }

    pub(super) fn pop_stack(&mut self) -> StackEntry<'state, 'schema> {
        self.stack.pop().unwrap()
    }

    pub(super) fn type_is_active(&self, ident: &TypeIdent) -> bool {
        self.stack
            .iter()
            .any(|x| matches!(x, StackEntry::Type { ident: x, .. } if x == ident))
    }

    pub(super) fn is_mixed(&self) -> bool {
        self.stack
            .iter()
            .rev()
            .find_map(|x| {
                if let StackEntry::Mixed { mixed } = x {
                    Some(*mixed)
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    pub(super) fn group_cache(&self) -> Option<&HashMap<TypeIdent, TypeIdent>> {
        self.stack.iter().rev().find_map(|x| {
            if let StackEntry::Type { group_cache, .. } = x {
                Some(group_cache)
            } else {
                None
            }
        })
    }

    pub(super) fn group_cache_mut(&mut self) -> Option<&mut HashMap<TypeIdent, TypeIdent>> {
        self.stack.iter_mut().rev().find_map(|x| {
            if let StackEntry::Type { group_cache, .. } = x {
                Some(group_cache)
            } else {
                None
            }
        })
    }

    pub(super) fn named_group(&self) -> Option<String> {
        if let StackEntry::GroupRef {
            ident,
            name: prefix,
        } = self.stack.last()?
        {
            let name = ident.name.as_str();
            if let Some(prefix) = prefix {
                Some(format!("{prefix}:{name}"))
            } else {
                Some(name.to_owned())
            }
        } else {
            None
        }
    }

    pub(super) fn current_ident(&self) -> &TypeIdent {
        self.stack
            .iter()
            .rev()
            .find_map(|entry| match entry {
                StackEntry::Type { ident, .. } => Some(ident),
                _ => None,
            })
            .expect("Unable to get current type")
    }

    pub(super) fn current_ns_id(&self) -> NamespaceId {
        self.current_ident().ns
    }

    pub(super) fn current_schema_id(&self) -> SchemaId {
        self.current_ident().schema
    }

    pub(super) fn current_schema(&self) -> &Schema {
        let id = self.current_schema_id();
        let info = self
            .schemas
            .get_schema(&id)
            .expect("Unable to get current schema info");

        &info.schema
    }

    pub(super) fn get_node_entry(&self, ident: &TypeIdent) -> &'state NodeCacheEntry<'schema> {
        let Some(entry) = self.node_cache.get(ident) else {
            panic!("Unable to get node node entry for {ident}");
        };

        entry
    }

    pub(super) fn current_node_entry(&self) -> &'state NodeCacheEntry<'schema> {
        self.stack
            .iter()
            .rev()
            .find_map(|entry| match entry {
                StackEntry::NodeEntry { entry } => Some(*entry),
                _ => None,
            })
            .expect("Unable to get current node entry")
    }

    pub(super) fn resolve_type_ident(
        &self,
        qname: &QName,
        ident_type: IdentType,
    ) -> Result<TypeIdent, Error> {
        let ns = self.current_ns_id();
        let (ns, name) = parse_qname(qname, ns, self.schemas)?;
        let key = NodeDependencyKey::Named(ns, ident_type, name);

        Ok(self.resolve_type_ident_with_key(&key))
    }

    pub(super) fn resolve_type_ident_with_key(&self, key: &NodeDependencyKey) -> TypeIdent {
        let entry = self.current_node_entry();

        match entry.dependencies.get(key) {
            Some(dep) => TypeIdent::clone(dep),
            None => {
                // All dependencies should have been processed by the node cache
                // and should be available in the current context
                panic!("Unable to resolve type ident for key: {key:?}");
            }
        }
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

        self.ident_cache.resolve(ident)
    }

    pub(super) fn name_builder(&self) -> Box<dyn NameBuilder> {
        self.types.name_builder()
    }

    pub(super) fn make_content_name(&mut self) -> Name {
        self.name_builder()
            .auto_extend(&self.stack)
            .content_type_name()
            .shared_name("Content")
            .finish()
    }
}
