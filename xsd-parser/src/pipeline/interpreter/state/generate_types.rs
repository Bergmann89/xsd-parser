mod name_builder;
mod type_processor;
mod update;
mod variant_processor;

use std::collections::HashMap;

use crate::models::TypeIdent;

use super::{Error, NodeCacheEntry, State};

use self::name_builder::NameBuilderExt;
use self::type_processor::TypeProcessor;
use self::update::Update;
use self::variant_processor::VariantProcessor;

impl State<'_> {
    /// Executes the actual generation step to produce [`MetaType`](crate::models::meta::MetaType)
    /// instances from the provided schema and type identifiers.
    ///
    /// This method generates at least one new [`MetaType`](crate::models::meta::MetaType) instance
    /// for each identifier in the passed `nodes` list. It may generate more than one instance per
    /// identifier if the type definitions in the schema require it (e.g., due to complex type hierarchies
    /// or group references).
    ///
    /// The method will fail if the passed `nodes` list contains identifiers that do not correspond
    /// to any type definition in the schema, or if the list does not respect the hard dependencies
    /// between types (e.g., if a type is listed before its dependencies).
    pub(super) fn generate_types(&mut self, nodes: Vec<TypeIdent>) -> Result<(), Error> {
        let mut processor = TypeProcessor::new(
            self.schemas,
            &mut self.types,
            &self.node_cache,
            &mut self.ident_cache,
        );

        for ident in nodes {
            processor.process_ident(ident)?;
        }

        Ok(())
    }
}

enum StackEntry<'state, 'schema> {
    /// Node currently being processed
    NodeEntry {
        entry: &'state NodeCacheEntry<'schema>,
    },

    /// Type currently being generated
    Type {
        ident: TypeIdent,
        group_cache: HashMap<TypeIdent, TypeIdent>,
    },

    /// Indicates the current processed group
    GroupRef {
        ident: TypeIdent,
        name: Option<String>,
    },

    /// Is set if the current processed type is mixed.
    Mixed { mixed: bool },

    /// Marker for a group
    Group,
}
