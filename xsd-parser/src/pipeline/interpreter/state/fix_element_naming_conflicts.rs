use crate::models::meta::{DynamicMeta, GroupMeta, MetaTypeVariant};
use crate::models::schema::{NamespaceId, NamespaceInfo, Schemas};

use super::State;

impl State<'_> {
    /// Resolves naming conflicts for elements within type definitions.
    ///
    /// This method iterates through all types in the state and applies naming conflict resolution
    /// to container types (All, Choice, Sequence) and Dynamic types. This is necessary because
    /// elements with identical names but different namespaces can cause ambiguity during schema
    /// parsing and code generation.
    ///
    /// To resolve conflicts, the prefix of the namespace is added to the element name if known,
    /// or an index is appended to the name if multiple elements share the same name within the
    /// same namespace.
    pub(super) fn fix_element_naming_conflicts(&mut self) {
        for type_ in self.types.items.values_mut() {
            match &mut type_.variant {
                MetaTypeVariant::All(x)
                | MetaTypeVariant::Choice(x)
                | MetaTypeVariant::Sequence(x) => {
                    fix_naming_conflicts(self.schemas, x);
                }
                MetaTypeVariant::Dynamic(x) => fix_naming_conflicts(self.schemas, x),
                _ => {}
            }
        }
    }
}

fn fix_naming_conflicts<X>(schemas: &Schemas, meta: &mut X)
where
    X: IdentSlice,
{
    let len = meta.len();
    for i in 0..len {
        let mut index = 2;
        let mut ns_renamed = false;

        for j in i + 1..len {
            let ident_i = &meta.get_ident(i);
            let ident_j = &meta.get_ident(j);
            if ident_i.1 == ident_j.1 {
                if ident_i.0 == ident_j.0 {
                    let display_name = make_index_display_name(ident_j.1, &mut index);
                    meta.set_display_name(j, Some(display_name));
                } else {
                    ns_renamed = true;

                    let display_name = make_ns_display_name(ident_j.0, ident_j.1, schemas);

                    meta.set_display_name(j, display_name);
                }
            }
        }

        if ns_renamed {
            let ident = meta.get_ident(i);
            let display_name = make_ns_display_name(ident.0, ident.1, schemas);
            meta.set_display_name(i, display_name);
        }
    }
}

fn make_ns_display_name(ns: NamespaceId, name: &str, schemas: &Schemas) -> Option<String> {
    let prefix = schemas
        .get_namespace_info(&ns)
        .and_then(NamespaceInfo::name);

    prefix.map(|prefix| format!("{prefix}_{name}"))
}

fn make_index_display_name(name: &str, index: &mut usize) -> String {
    let name = format!("{name}_{index}");

    *index += 1;

    name
}

trait IdentSlice {
    fn len(&self) -> usize;
    fn get_ident(&self, index: usize) -> (NamespaceId, &str);
    fn set_display_name(&mut self, index: usize, display_name: Option<String>);
}

impl IdentSlice for GroupMeta {
    fn len(&self) -> usize {
        self.elements.len()
    }

    fn get_ident(&self, index: usize) -> (NamespaceId, &str) {
        let el = &self.elements[index];
        (el.ident.ns, el.ident.name.as_str())
    }

    fn set_display_name(&mut self, index: usize, display_name: Option<String>) {
        self.elements[index].display_name = display_name;
    }
}

impl IdentSlice for DynamicMeta {
    fn len(&self) -> usize {
        self.derived_types.len()
    }

    fn get_ident(&self, index: usize) -> (NamespaceId, &str) {
        let type_ = &self.derived_types[index].type_;
        (type_.ns, type_.name.as_str())
    }

    fn set_display_name(&mut self, index: usize, display_name: Option<String>) {
        self.derived_types[index].display_name = display_name;
    }
}
