use xsd_parser_types::misc::Namespace;

use crate::models::schema::xs::{
    AttributeGroupType, AttributeType, ComplexBaseType, ElementType, GroupType, SimpleBaseType,
};
use crate::models::{IdentType, Name, TypeIdent};

use super::State;

impl State<'_> {
    /// Creates and populates an identifier cache for all schemas and their contents.
    ///
    /// This method builds a comprehensive cache of all named identifiers (elements, attributes,
    /// types, groups, etc.) across all schemas in the project. The cache is used for efficient
    /// name resolution and validation during schema interpretation.
    ///
    /// This cache enables the interpreter to quickly resolve type references, validate identifier
    /// uniqueness, and manage namespace-scoped definitions throughout the schema interpretation pipeline.
    ///
    /// # Operations
    ///
    /// 1. Iterates through all namespaces and registers schemas:
    ///    - Maps each namespace to its associated schemas
    ///    - Marks global namespaces (XML, XS, XSI) for special handling
    ///
    /// 2. Processes each schema's content:
    ///    - Extracts named elements, attributes, simple types, complex types, groups, and attribute groups
    ///    - Creates and caches `TypeIdent` entries for each named definition
    ///
    /// 3. Handles schema redefinitions and overrides:
    ///    - Processes content within `<xsd:redefine>` sections (simple/complex types, groups, attribute groups)
    ///    - Processes content within `<xsd:override>` sections (including elements and attributes)
    ///    - Caches identifiers with their defining schema and namespace context
    pub(super) fn create_ident_cache(&mut self) {
        for (ns, info) in self.schemas.namespaces() {
            for schema in &info.schemas {
                self.ident_cache.add_schema(*ns, *schema);
            }

            if matches!(&info.namespace, Some(ns) if ns.eq(&Namespace::XML) || ns.eq(&Namespace::XS) || ns.eq(&Namespace::XSI))
            {
                self.ident_cache.add_global_namespace(*ns);
            }
        }

        for (schema, info) in self.schemas.schemas() {
            let ns = info.namespace_id;
            let schema = *schema;

            self.ident_cache.add_schema(ns, schema);
            for dep in info.dependencies.values() {
                self.ident_cache.add_dependency(schema, *dep);
            }

            for c in &info.schema.content {
                use crate::models::schema::xs::SchemaContent as C;

                #[rustfmt::skip]
                let (name, type_) = match c {
                    C::Element(ElementType { name: Some(name), .. }) => (name, IdentType::Element),
                    C::Attribute(AttributeType { name: Some(name), .. }) => (name, IdentType::Attribute),
                    C::SimpleType(SimpleBaseType { name: Some(name), .. }) => (name, IdentType::Type),
                    C::ComplexType(ComplexBaseType { name: Some(name), .. }) => (name, IdentType::Type),
                    C::Group(GroupType { name: Some(name), .. }) => (name, IdentType::Group),
                    C::AttributeGroup(AttributeGroupType { name: Some(name), .. }) => (name, IdentType::AttributeGroup),
                    C::Redefine(x) => {
                        use crate::models::schema::xs::RedefineContent as C;

                        for c in &x.content {
                            let (name, type_) = match c {
                                C::SimpleType(SimpleBaseType { name: Some(name), .. }) => (name, IdentType::Type),
                                C::ComplexType(ComplexBaseType { name: Some(name), .. }) => (name, IdentType::Type),
                                C::Group(GroupType { name: Some(name), .. }) => (name, IdentType::Group),
                                C::AttributeGroup(AttributeGroupType { name: Some(name), .. }) => (name, IdentType::AttributeGroup),
                                _ => continue,
                            };

                            let name = Name::new_named(name.clone());
                            let ident = TypeIdent {
                                ns,
                                schema,
                                name: name.clone(),
                                type_,
                            };

                            self.ident_cache.insert(ident.clone());
                        }

                        continue;
                    }
                    C::Override(x) => {
                        use crate::models::schema::xs::OverrideContent as C;

                        for c in &x.content {
                            let (name, type_) = match c {
                                C::SimpleType(SimpleBaseType { name: Some(name), .. }) => (name, IdentType::Type),
                                C::ComplexType(ComplexBaseType { name: Some(name), .. }) => (name, IdentType::Type),
                                C::Group(GroupType { name: Some(name), .. }) => (name, IdentType::Group),
                                C::AttributeGroup(AttributeGroupType { name: Some(name), .. }) => (name, IdentType::AttributeGroup),
                                C::Element(ElementType { name: Some(name), .. }) => (name, IdentType::Element),
                                C::Attribute(AttributeType { name: Some(name), .. }) => (name, IdentType::Attribute),
                                _ => continue,
                            };

                            let name = Name::new_named(name.clone());
                            let ident = TypeIdent {
                                ns,
                                schema,
                                name: name.clone(),
                                type_,
                            };

                            self.ident_cache.insert(ident.clone());
                        }

                        continue;
                    }
                    _ => continue,
                };

                let name = Name::new_named(name.clone());
                let ident = TypeIdent {
                    ns,
                    schema,
                    name,
                    type_,
                };

                self.ident_cache.insert(ident.clone());
            }
        }
    }
}
