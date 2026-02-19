use std::collections::{btree_map::BTreeMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::str::from_utf8;

use xsd_parser_types::{
    misc::{Namespace, RawByteStr},
    xml::QName,
};

use crate::models::meta::MetaTypes;
use crate::models::schema::xs::{
    AttributeGroupType, AttributeType, ComplexBaseType, ComplexContent, ElementType, ExtensionType,
    GroupType, Import, List, Override, Redefine, Restriction, RestrictionType, SimpleBaseType,
    SimpleContent, Union,
};
use crate::models::schema::{NamespaceId, SchemaId, SchemaInfo, Schemas};
use crate::models::{IdentCache, IdentType, Name, TypeIdent};
use crate::traits::{NameBuilder, NameBuilderExt as _};

use super::parse_qname;
use super::{Error, Node, NodeCache, NodeCacheEntry, NodeDependency, NodeDependencyKey, State};

impl State<'_> {
    /// Creates a node cache for the current state.
    ///
    /// This method initializes and processes the cache of nodes representing types
    /// and their dependencies extracted from the loaded schemas. The node cache
    /// is essential for tracking relationships between schema elements, including
    /// strong, weak, and lazy dependencies, as well as handling redefinitions.
    ///
    /// The processor traverses all schemas, resolves cross-references, and builds
    /// a dependency graph that is later used for code generation and type resolution.
    ///
    /// It also resolved redefinitions and overrides by updating the dependencies
    /// to point to the new identifiers, and cloning entries if necessary to maintain
    /// the integrity of the node cache across different schemas.
    pub(super) fn create_node_cache(&mut self) -> Result<(), Error> {
        let processor = NodeCacheProcessor {
            types: &self.types,
            schemas: self.schemas,
            node_cache: &mut self.node_cache,
            ident_cache: &mut self.ident_cache,

            stack: Vec::new(),
            processed_schemas: HashSet::new(),
        };

        processor.process()?;

        Ok(())
    }
}

struct NodeCacheProcessor<'state, 'schema> {
    types: &'state MetaTypes,
    schemas: &'schema Schemas,
    node_cache: &'state mut NodeCache<'schema>,
    ident_cache: &'state mut IdentCache,

    stack: Vec<StackEntry<'schema>>,
    processed_schemas: HashSet<SchemaId>,
}

enum StackEntry<'schema> {
    Schema {
        id: SchemaId,
        info: &'schema SchemaInfo,
    },
    Node {
        ident: TypeIdent,
        entry: NodeCacheEntry<'schema>,
    },
    Group,
    AttributeGroup,
}

impl<'schema> NodeCacheProcessor<'_, 'schema> {
    fn process(mut self) -> Result<(), Error> {
        for (ns, info) in self.schemas.namespaces() {
            if matches!(&info.namespace, Some(ns) if ns.eq(&Namespace::XML) || ns.eq(&Namespace::XS) || ns.eq(&Namespace::XSI))
            {
                self.ident_cache.add_global_namespace(*ns);
            }
        }

        for (schema, _) in self.schemas.schemas() {
            self.process_schema(*schema)?;
        }

        Ok(())
    }

    fn process_schema(&mut self, id: SchemaId) -> Result<(), Error> {
        if !self.processed_schemas.insert(id) {
            return Ok(());
        }

        let info = self.schemas.get_schema(&id).unwrap();
        self.push_stack(StackEntry::Schema { id, info });

        self.process_schema_imports(id, info)?;
        self.process_schema_content(info)?;

        self.pop_stack();

        Ok(())
    }

    fn process_schema_imports(
        &mut self,
        schema: SchemaId,
        info: &'schema SchemaInfo,
    ) -> Result<(), Error> {
        self.ident_cache.add_schema(info.namespace_id, schema);
        for dep in info.dependencies.values() {
            self.ident_cache.add_dependency(schema, *dep);
        }

        for c in &info.schema.content {
            use crate::models::schema::xs::SchemaContent as C;

            match c {
                C::Import(Import {
                    schema_location: Some(schema_location),
                    ..
                }) => {
                    let base = **info.dependencies.get(schema_location).unwrap();

                    self.process_schema(base)?;
                }
                C::Include(x) => {
                    let base = **info.dependencies.get(&x.schema_location).unwrap();

                    self.process_schema(base)?;
                }
                C::Override(x) => {
                    let base = **info.dependencies.get(&x.schema_location).unwrap();

                    self.process_schema(base)?;
                    self.process_override(x, base)?;
                }
                C::Redefine(x) => {
                    let base = **info.dependencies.get(&x.schema_location).unwrap();

                    self.process_schema(base)?;
                    self.process_redefine(x, base)?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn process_schema_content(&mut self, info: &'schema SchemaInfo) -> Result<(), Error> {
        for c in &info.schema.content {
            use crate::models::schema::xs::SchemaContent as C;

            match c {
                C::Include(_)
                | C::Import(_)
                | C::Override(_)
                | C::Redefine(_)
                | C::Notation(_)
                | C::Annotation(_)
                | C::DefaultOpenContent(_) => (),
                C::Element(
                    x @ ElementType {
                        name: Some(name), ..
                    },
                ) => {
                    self.new_entry(name.clone(), Node::Element(x), |s| s.process_element(x))?;
                }
                C::Attribute(
                    x @ AttributeType {
                        name: Some(name), ..
                    },
                ) => {
                    self.new_entry(name.clone(), Node::Attribute(x), |s| s.process_attribute(x))?;
                }
                C::SimpleType(
                    x @ SimpleBaseType {
                        name: Some(name), ..
                    },
                ) => {
                    self.new_entry(name.clone(), Node::SimpleType(x), |s| {
                        s.process_simple_type(x)
                    })?;
                }
                C::ComplexType(
                    x @ ComplexBaseType {
                        name: Some(name), ..
                    },
                ) => {
                    self.new_entry(name.clone(), Node::ComplexType(x), |s| {
                        s.process_complex_type(x)
                    })?;
                }
                C::Group(
                    x @ GroupType {
                        name: Some(name), ..
                    },
                ) => {
                    self.new_entry(name.clone(), Node::Group(x), |s| s.process_group(x))?;
                }
                C::AttributeGroup(
                    x @ AttributeGroupType {
                        name: Some(name), ..
                    },
                ) => {
                    self.new_entry(name.clone(), Node::AttributeGroup(x), |s| {
                        s.process_attribute_group(x)
                    })?;
                }
                x => crate::unreachable!("{x:#?}"),
            }
        }

        Ok(())
    }

    fn process_override(&mut self, ty: &'schema Override, base: SchemaId) -> Result<(), Error> {
        let mut idents = HashSet::new();

        for c in &ty.content {
            use crate::models::schema::xs::OverrideContent as C;

            let ident = match c {
                C::Notation(_) | C::Annotation(_) => continue,
                C::Attribute(
                    x @ AttributeType {
                        name: Some(name), ..
                    },
                ) => self.new_redefined_entry(name.clone(), base, Node::Attribute(x), |s| {
                    s.process_attribute(x)
                })?,
                C::Element(
                    x @ ElementType {
                        name: Some(name), ..
                    },
                ) => self.new_redefined_entry(name.clone(), base, Node::Element(x), |s| {
                    s.process_element(x)
                })?,
                C::SimpleType(
                    x @ SimpleBaseType {
                        name: Some(name), ..
                    },
                ) => self.new_redefined_entry(name.clone(), base, Node::SimpleType(x), |s| {
                    s.process_simple_type(x)
                })?,
                C::ComplexType(
                    x @ ComplexBaseType {
                        name: Some(name), ..
                    },
                ) => self.new_redefined_entry(name.clone(), base, Node::ComplexType(x), |s| {
                    s.process_complex_type(x)
                })?,
                C::Group(
                    x @ GroupType {
                        name: Some(name), ..
                    },
                ) => self.new_redefined_entry(name.clone(), base, Node::Group(x), |s| {
                    s.process_group(x)
                })?,
                C::AttributeGroup(
                    x @ AttributeGroupType {
                        name: Some(name), ..
                    },
                ) => {
                    self.new_redefined_entry(name.clone(), base, Node::AttributeGroup(x), |s| {
                        s.process_attribute_group(x)
                    })?
                }
                x => crate::unreachable!("{x:#?}"),
            };

            idents.insert(ident.with_schema(base));
        }

        self.resolve_dependencies(idents);

        Ok(())
    }

    fn process_redefine(&mut self, ty: &'schema Redefine, base: SchemaId) -> Result<(), Error> {
        let mut idents = HashSet::new();

        for c in &ty.content {
            use crate::models::schema::xs::RedefineContent as C;

            let ident = match c {
                C::Annotation(_) => continue,
                C::SimpleType(
                    x @ SimpleBaseType {
                        name: Some(name), ..
                    },
                ) => self.new_redefined_entry(name.clone(), base, Node::SimpleType(x), |s| {
                    s.process_simple_type(x)
                })?,
                C::ComplexType(
                    x @ ComplexBaseType {
                        name: Some(name), ..
                    },
                ) => self.new_redefined_entry(name.clone(), base, Node::ComplexType(x), |s| {
                    s.process_complex_type(x)
                })?,
                C::Group(
                    x @ GroupType {
                        name: Some(name), ..
                    },
                ) => self.new_redefined_entry(name.clone(), base, Node::Group(x), |s| {
                    s.process_group(x)
                })?,
                C::AttributeGroup(
                    x @ AttributeGroupType {
                        name: Some(name), ..
                    },
                ) => {
                    self.new_redefined_entry(name.clone(), base, Node::AttributeGroup(x), |s| {
                        s.process_attribute_group(x)
                    })?
                }
                x => crate::unreachable!("{x:#?}"),
            };

            idents.insert(ident.with_schema(base));
        }

        self.resolve_dependencies(idents);

        Ok(())
    }

    /// Resolve a list of existing identifiers that needs to be updated to point
    /// to the redefined/overwritten types.
    fn resolve_dependencies(&mut self, mut idents: HashSet<TypeIdent>) {
        let schema = self.current_schema();

        // Take a old type that needs to be resolved
        while let Some(old_ident) = idents.iter().next().cloned() {
            idents.remove(&old_ident);
            let new_ident = old_ident.clone().with_schema(schema);

            // For each identifier in the current change set (including the active schema)
            for entry_ident in self.ident_cache.schema_set(schema).collect::<Vec<_>>() {
                let Some(entry) = self.node_cache.get_mut(&entry_ident) else {
                    continue;
                };

                // Find a dependency that points to the old identifier
                let dep = entry.dependencies.iter_mut().find(|(_key, dep)| {
                    ***dep == old_ident
                        && !matches!(&entry.redefine_base, Some(base) if *base == old_ident)
                });
                let Some((key, dep)) = dep else {
                    continue;
                };

                // If the entry already lives in the new schema, just update the
                // dependency to point to the new identifier
                if entry_ident.schema == schema {
                    **dep = new_ident.clone();

                // ... otherwise clone the whole entry, update the dependency and
                // store the new entry in the current schema
                } else {
                    let key = key.clone();

                    let mut dep = dep.clone();
                    let mut copy = entry.clone();

                    *dep = new_ident.clone();
                    copy.redefine_base = Some(entry_ident.clone());
                    copy.dependencies.insert(key, dep);

                    let ident = entry_ident.clone().with_schema(schema);
                    self.node_cache.insert(ident.clone(), copy);
                    self.ident_cache.insert(ident.clone());

                    idents.insert(entry_ident.clone());
                }
            }
        }
    }

    fn process_element(&mut self, ty: &'schema ElementType) -> Result<(), Error> {
        use crate::models::schema::xs::ElementTypeContent as C;

        if let Some(name) = &ty.type_ {
            let ident = self.resolve_type_ident(name, IdentType::Type)?;
            self.add_dependency(IdentType::Type, name, NodeDependency::Weak(ident))?;
        }

        for sub_group in ty.substitution_group.iter().flat_map(|v| v.0.iter()) {
            let ident = self.resolve_type_ident(sub_group, IdentType::Element)?;
            self.add_dependency(IdentType::Element, sub_group, NodeDependency::Strong(ident))?;
        }

        for c in &ty.content {
            match c {
                C::SimpleType(ty) => self.process_simple_type(ty)?,
                C::ComplexType(ty) => self.process_complex_type(ty)?,
                C::Annotation(_) | C::Unique(_) | C::Key(_) | C::Keyref(_) | C::Alternative(_) => {}
            }
        }

        Ok(())
    }

    fn process_attribute(&mut self, ty: &'schema AttributeType) -> Result<(), Error> {
        if let Some(name) = &ty.type_ {
            let ident = self.resolve_type_ident(name, IdentType::Type)?;
            self.add_dependency(IdentType::Type, name, NodeDependency::Weak(ident))?;
        }

        if let Some(simple_type) = &ty.simple_type {
            self.process_simple_type(simple_type)?;
        }

        Ok(())
    }

    fn process_simple_type(&mut self, ty: &'schema SimpleBaseType) -> Result<(), Error> {
        for c in &ty.content {
            use crate::models::schema::xs::SimpleBaseTypeContent as C;

            match c {
                C::List(x) => self.process_list(x)?,
                C::Union(x) => self.process_simple_union(x)?,
                C::Restriction(x) => self.process_simple_restriction(x)?,
                C::Annotation(_) => (),
            }
        }

        Ok(())
    }

    fn process_simple_restriction(&mut self, ty: &'schema Restriction) -> Result<(), Error> {
        if let Some(base) = &ty.base {
            let ident = self.resolve_type_ident(base, IdentType::Type)?;
            self.add_dependency(IdentType::Type, base, NodeDependency::Strong(ident))?;
        }

        for c in &ty.content {
            use crate::models::schema::xs::RestrictionContent as C;

            match c {
                C::Facet(_) | C::Annotation(_) => (),
                C::SimpleType(ty) => self.process_simple_type(ty)?,
            }
        }

        Ok(())
    }

    fn process_list(&mut self, ty: &'schema List) -> Result<(), Error> {
        if let Some(item_type) = &ty.item_type {
            let ident = self.resolve_type_ident(item_type, IdentType::Type)?;
            self.add_dependency(IdentType::Type, item_type, NodeDependency::Weak(ident))?;
        }

        if let Some(x) = &ty.simple_type {
            let last_named_type = self.last_named_type();
            let name = self
                .types
                .name_builder()
                .or(&x.name)
                .or_else(|| from_utf8(ty.item_type.as_ref()?.local_name()).ok())
                .or_else(|| {
                    let s = last_named_type?;
                    let s = s.strip_suffix("Type").unwrap_or(s);
                    let s = format!("{s}Item");

                    Some(Name::from(s))
                })
                .finish();

            let ident =
                self.new_inline_entry(name, Node::SimpleType(x), |s| s.process_simple_type(x))?;

            self.add_dependency_with_key(
                NodeDependencyKey::InlineSimpleType(&**x),
                NodeDependency::Weak(ident),
            );
        }

        Ok(())
    }

    fn process_simple_union(&mut self, ty: &'schema Union) -> Result<(), Error> {
        for member_type in ty.member_types.iter().flat_map(|v| v.0.iter()) {
            let ident = self.resolve_type_ident(member_type, IdentType::Type)?;
            self.add_dependency(IdentType::Type, member_type, NodeDependency::Weak(ident))?;
        }

        for x in &ty.simple_type {
            let name = self
                .types
                .name_builder()
                .or(&x.name)
                .auto_extend(false, true, &self.stack)
                .finish();

            let ident =
                self.new_inline_entry(name, Node::SimpleType(x), |s| s.process_simple_type(x))?;

            self.add_dependency_with_key(
                NodeDependencyKey::InlineSimpleType(x),
                NodeDependency::Weak(ident),
            );
        }

        Ok(())
    }

    fn process_complex_type(&mut self, ty: &'schema ComplexBaseType) -> Result<(), Error> {
        for c in &ty.content {
            use crate::models::schema::xs::ComplexBaseTypeContent as C;

            match c {
                C::Annotation(_) | C::OpenContent(_) | C::AnyAttribute(_) | C::Assert(_) => (),
                C::SimpleContent(x) => self.process_simple_content(x)?,
                C::ComplexContent(x) => self.process_complex_content(x)?,
                C::All(x) | C::Choice(x) | C::Sequence(x) => self.process_group_content(x)?,
                C::Attribute(x) => self.process_attribute_ref(x)?,
                C::Group(x) => self.process_group_ref(x)?,
                C::AttributeGroup(x) => self.process_attribute_group_ref(x)?,
            }
        }

        Ok(())
    }

    fn process_simple_content(&mut self, ty: &'schema SimpleContent) -> Result<(), Error> {
        for c in &ty.content {
            use crate::models::schema::xs::SimpleContentContent as C;

            match c {
                C::Annotation(_) => (),
                C::Restriction(x) => self.process_restriction(x)?,
                C::Extension(x) => self.process_extension(x)?,
            }
        }

        Ok(())
    }

    fn process_complex_content(&mut self, ty: &'schema ComplexContent) -> Result<(), Error> {
        for c in &ty.content {
            use crate::models::schema::xs::ComplexContentContent as C;

            match c {
                C::Annotation(_) => (),
                C::Restriction(x) => self.process_restriction(x)?,
                C::Extension(x) => self.process_extension(x)?,
            }
        }

        Ok(())
    }

    fn process_restriction(&mut self, ty: &'schema RestrictionType) -> Result<(), Error> {
        let ident = self.resolve_type_ident(&ty.base, IdentType::Type)?;
        self.add_dependency(IdentType::Type, &ty.base, NodeDependency::Strong(ident))?;

        for c in &ty.content {
            use crate::models::schema::xs::RestrictionTypeContent as C;

            match c {
                C::Facet(_)
                | C::Assert(_)
                | C::Annotation(_)
                | C::OpenContent(_)
                | C::AnyAttribute(_) => (),
                C::Group(x) => self.process_group_ref(x)?,
                C::All(x) | C::Choice(x) | C::Sequence(x) => self.process_group_content(x)?,
                C::SimpleType(x) => self.process_simple_type(x)?,
                C::Attribute(x) => self.process_attribute_ref(x)?,
                C::AttributeGroup(x) => self.process_attribute_group_ref(x)?,
            }
        }

        Ok(())
    }

    fn process_extension(&mut self, ty: &'schema ExtensionType) -> Result<(), Error> {
        let ident = self.resolve_type_ident(&ty.base, IdentType::Type)?;
        self.add_dependency(IdentType::Type, &ty.base, NodeDependency::Strong(ident))?;

        for c in &ty.content {
            use crate::models::schema::xs::ExtensionTypeContent as C;

            match c {
                C::Assert(_) | C::Annotation(_) | C::OpenContent(_) | C::AnyAttribute(_) => (),
                C::Group(x) => self.process_group_ref(x)?,
                C::All(x) | C::Choice(x) | C::Sequence(x) => self.process_group_content(x)?,
                C::Attribute(x) => self.process_attribute_ref(x)?,
                C::AttributeGroup(x) => self.process_attribute_group_ref(x)?,
            }
        }

        Ok(())
    }

    fn process_group(&mut self, ty: &'schema GroupType) -> Result<(), Error> {
        self.push_stack(StackEntry::Group);

        self.process_group_content(ty)?;

        self.pop_stack();

        Ok(())
    }

    fn process_group_content(&mut self, ty: &'schema GroupType) -> Result<(), Error> {
        for c in &ty.content {
            use crate::models::schema::xs::GroupTypeContent as C;

            match c {
                C::Any(_) | C::Annotation(_) => (),
                C::Group(x) => self.process_group_ref(x)?,
                C::Element(x) => self.process_element_ref(x)?,
                C::All(x) | C::Choice(x) | C::Sequence(x) => self.process_group_content(x)?,
            }
        }

        Ok(())
    }

    fn process_attribute_group(&mut self, ty: &'schema AttributeGroupType) -> Result<(), Error> {
        self.push_stack(StackEntry::AttributeGroup);

        for c in &ty.content {
            use crate::models::schema::xs::AttributeGroupTypeContent as C;

            match c {
                C::Annotation(_) | C::AnyAttribute(_) => (),
                C::Attribute(x) => self.process_attribute_ref(x)?,
                C::AttributeGroup(x) => self.process_attribute_group_ref(x)?,
            }
        }

        self.pop_stack();

        Ok(())
    }

    fn process_attribute_ref(&mut self, ty: &'schema AttributeType) -> Result<(), Error> {
        if let Some(name) = &ty.ref_ {
            let ident = self.resolve_type_ident(name, IdentType::Attribute)?;
            self.add_dependency(IdentType::Attribute, name, NodeDependency::Weak(ident))?;
        }

        if let Some(name) = &ty.type_ {
            let ident = self.resolve_type_ident(name, IdentType::Type)?;
            self.add_dependency(IdentType::Type, name, NodeDependency::Weak(ident))?;
        }

        if let Some(x) = &ty.simple_type {
            let name = self
                .types
                .name_builder()
                .or(&ty.name)
                .or(&x.name)
                .auto_extend(true, true, &self.stack)
                .finish();

            let ident =
                self.new_inline_entry(name, Node::SimpleType(x), |s| s.process_simple_type(x))?;

            self.add_dependency_with_key(
                NodeDependencyKey::InlineSimpleType(x),
                NodeDependency::Weak(ident),
            );
        }

        Ok(())
    }

    fn process_element_ref(&mut self, ty: &'schema ElementType) -> Result<(), Error> {
        if let Some(name) = &ty.ref_ {
            let ident = self.resolve_type_ident(name, IdentType::Element)?;
            self.add_dependency(IdentType::Element, name, NodeDependency::Weak(ident))?;
        }

        if let Some(name) = &ty.type_ {
            let ident = self.resolve_type_ident(name, IdentType::Type)?;
            self.add_dependency(IdentType::Type, name, NodeDependency::Weak(ident))?;
        }

        if ty.type_.is_none() && ty.ref_.is_none() {
            let name = self
                .types
                .name_builder()
                .extend(true, ty.name.clone())
                .auto_extend(true, false, &self.stack);
            let name = if name.has_extension() {
                name.with_id(false)
            } else {
                name.shared_name("Temp")
            };
            let name = name.finish();

            let ident =
                self.new_inline_entry(name, Node::Element(ty), |s| s.process_element(ty))?;

            self.add_dependency_with_key(
                NodeDependencyKey::InlineElement(ty),
                NodeDependency::Weak(ident),
            );
        }

        Ok(())
    }

    fn process_group_ref(&mut self, ty: &'schema GroupType) -> Result<(), Error> {
        if let Some(name) = &ty.ref_ {
            let ident = self.resolve_type_ident(name, IdentType::Group)?;
            self.add_dependency(IdentType::Group, name, NodeDependency::Lazy(ident))?;
        }

        self.process_group(ty)?;

        Ok(())
    }

    fn process_attribute_group_ref(
        &mut self,
        ty: &'schema AttributeGroupType,
    ) -> Result<(), Error> {
        if let Some(name) = &ty.ref_ {
            let ident = self.resolve_type_ident(name, IdentType::AttributeGroup)?;
            self.add_dependency(IdentType::AttributeGroup, name, NodeDependency::Lazy(ident))?;
        }

        self.process_attribute_group(ty)?;

        Ok(())
    }

    fn new_entry<F>(&mut self, name: String, node: Node<'schema>, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Self) -> Result<(), Error>,
    {
        let entry = NodeCacheEntry::new(node);
        let ident = TypeIdent {
            ns: self.current_ns(),
            schema: self.current_schema(),
            name: Name::new_named(name),
            type_: entry.ident_type(),
        };

        self.push_stack(StackEntry::Node { ident, entry });

        f(self)?;

        let StackEntry::Node { ident, entry } = self.pop_stack() else {
            unreachable!();
        };

        self.ident_cache.insert(ident.clone());
        self.node_cache.insert(ident, entry);

        Ok(())
    }

    fn new_inline_entry<F>(
        &mut self,
        name: Name,
        node: Node<'schema>,
        f: F,
    ) -> Result<TypeIdent, Error>
    where
        F: FnOnce(&mut Self) -> Result<(), Error>,
    {
        let entry = NodeCacheEntry::new(node);
        let ident = TypeIdent {
            ns: self.current_ns(),
            schema: self.current_schema(),
            name,
            type_: entry.ident_type(),
        };

        self.push_stack(StackEntry::Node { ident, entry });

        f(self)?;

        let StackEntry::Node { ident, entry } = self.pop_stack() else {
            unreachable!();
        };

        self.ident_cache.insert(ident.clone());
        self.node_cache.insert(ident.clone(), entry);

        Ok(ident)
    }

    fn new_redefined_entry<F>(
        &mut self,
        name: String,
        base: SchemaId,
        node: Node<'schema>,
        f: F,
    ) -> Result<TypeIdent, Error>
    where
        F: FnOnce(&mut Self) -> Result<(), Error>,
    {
        let mut ret = None;

        self.new_entry(name.clone(), node, |s| {
            let ns = s.current_ns();
            ret = Some(s.current_type_ident().clone());

            let entry = s.active_entry_mut();
            entry.redefine_base = Some(TypeIdent {
                ns,
                schema: base,
                name: Name::new_named(name),
                type_: entry.ident_type(),
            });

            f(s)
        })?;

        Ok(ret.unwrap())
    }

    fn add_dependency(
        &mut self,
        type_: IdentType,
        name: &QName,
        dependency: NodeDependency,
    ) -> Result<(), Error> {
        let ns = self.current_ns();
        let (ns, name) = parse_qname(name, ns, self.schemas)?;
        let key = NodeDependencyKey::Named(ns, type_, name);

        self.add_dependency_with_key(key, dependency);

        Ok(())
    }

    fn add_dependency_with_key(&mut self, key: NodeDependencyKey, dependency: NodeDependency) {
        self.active_entry_mut().dependencies.insert(key, dependency);
    }

    fn push_stack(&mut self, entry: StackEntry<'schema>) {
        self.stack.push(entry);
    }

    fn pop_stack(&mut self) -> StackEntry<'schema> {
        self.stack.pop().unwrap()
    }

    fn active_entry_mut(&mut self) -> &mut NodeCacheEntry<'schema> {
        for entry in self.stack.iter_mut().rev() {
            if let StackEntry::Node { entry, .. } = entry {
                return entry;
            }
        }

        panic!("No active node cache entry on stack");
    }

    fn current_type_ident(&self) -> &TypeIdent {
        for entry in self.stack.iter().rev() {
            if let StackEntry::Node { ident, .. } = entry {
                return ident;
            }
        }

        panic!("No active node on stack");
    }

    fn current_ns(&self) -> NamespaceId {
        for entry in self.stack.iter().rev() {
            if let StackEntry::Schema { info, .. } = entry {
                return info.namespace_id;
            }
        }

        panic!("No active schema on stack");
    }

    fn current_schema(&self) -> SchemaId {
        for entry in self.stack.iter().rev() {
            if let StackEntry::Schema { id, .. } = entry {
                return *id;
            }
        }

        panic!("No active schema on stack");
    }

    fn last_named_type(&self) -> Option<&str> {
        for x in self.stack.iter().rev() {
            if let StackEntry::Node { ident, .. } = x {
                if ident.name.is_named() {
                    return Some(ident.name.as_str());
                }
            }
        }

        None
    }

    fn resolve_type_ident(&mut self, qname: &QName, type_: IdentType) -> Result<TypeIdent, Error> {
        let ident = self.parse_type_ident(qname, type_)?;

        let entry = self.active_entry_mut();
        if let Some(base) = &entry.redefine_base {
            if base.ns == ident.ns
                && base.name.as_str() == ident.name.as_str()
                && base.type_ == ident.type_
            {
                return Ok(base.clone());
            }
        }

        let ident = self
            .ident_cache
            .resolve_for_schema(self.current_schema(), ident.clone())?;

        Ok(ident)
    }

    fn parse_type_ident(&self, qname: &QName, type_: IdentType) -> Result<TypeIdent, Error> {
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
}

impl<'schema> NodeCacheEntry<'schema> {
    fn new(node: Node<'schema>) -> Self {
        Self {
            node,
            dependencies: BTreeMap::new(),
            redefine_base: None,
        }
    }

    fn ident_type(&self) -> IdentType {
        match self.node {
            Node::Group(_) => IdentType::Group,
            Node::Element(_) => IdentType::Element,
            Node::Attribute(_) => IdentType::Attribute,
            Node::SimpleType(_) | Node::ComplexType(_) => IdentType::Type,
            Node::AttributeGroup(_) => IdentType::AttributeGroup,
        }
    }
}

impl Deref for NodeDependency {
    type Target = TypeIdent;

    fn deref(&self) -> &Self::Target {
        match self {
            NodeDependency::Strong(ident)
            | NodeDependency::Weak(ident)
            | NodeDependency::Lazy(ident) => ident,
        }
    }
}

impl DerefMut for NodeDependency {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            NodeDependency::Strong(ident)
            | NodeDependency::Weak(ident)
            | NodeDependency::Lazy(ident) => ident,
        }
    }
}

trait NameBuilderExt {
    fn auto_extend(self, stop_at_group_ref: bool, replace: bool, stack: &[StackEntry<'_>]) -> Self;
}

impl<X> NameBuilderExt for X
where
    X: NameBuilder,
{
    fn auto_extend(self, stop_at_group: bool, replace: bool, stack: &[StackEntry<'_>]) -> Self {
        for x in stack.iter().rev() {
            match x {
                StackEntry::Node { ident, .. } => {
                    return self.extend(replace, Some(ident.name.as_str()))
                }
                StackEntry::Group | StackEntry::AttributeGroup if stop_at_group => break,
                _ => (),
            }
        }

        self
    }
}
