use crate::models::{meta::SchemaMeta, Name};

use super::State;

impl State<'_> {
    /// Prepares schema metadata for all schemas in the pipeline.
    ///
    /// This method iterates through all available schemas and creates [`SchemaMeta`] entries
    /// for each one. It extracts the schema's name and namespace information from the schema info,
    /// storing the results in [`MetaTypes::schemas`](crate::MetaTypes::schemas) for later use.
    pub(super) fn prepare_schemas(&mut self) {
        for (id, info) in self.schemas.schemas() {
            let schema = SchemaMeta {
                name: info.name.clone().map(Name::new_named),
                namespace: info.namespace_id(),
            };

            self.types.schemas.insert(*id, schema);
        }
    }
}
