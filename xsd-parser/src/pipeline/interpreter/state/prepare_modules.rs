use crate::models::{meta::ModuleMeta, Name};

use super::State;

impl State<'_> {
    /// Prepares and registers module metadata for all namespaces in the resulting types collection.
    ///
    /// This method iterates through all namespaces in the schema and extracts their metadata
    /// to create [`ModuleMeta`] entries. For each namespace, it collects:
    /// - The namespace prefix (converted to a [`Name`] if present)
    /// - The namespace name (converted to a [`Name`])
    /// - The namespace URI
    /// - The namespace ID
    /// - The count of schemas associated with the namespace
    ///
    /// The prepared module metadata is then inserted into [`MetaTypes::modules`](crate::MetaTypes::modules),
    /// making it available for later stages of the schema processing pipeline. This step
    /// is essential for maintaining a structured representation of all modules before further
    /// compilation and type resolution.
    pub(super) fn prepare_modules(&mut self) {
        for (id, info) in self.schemas.namespaces() {
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
}
