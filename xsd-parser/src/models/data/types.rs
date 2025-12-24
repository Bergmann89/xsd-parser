use std::ops::Deref;

use indexmap::IndexMap;

use crate::models::IdentMap;
use crate::pipeline::generator::MetaData as GeneratorMetaData;
use crate::Ident;

use super::DataType;

/// Holds all generated Rust data types along with associated metadata.
///
/// This structure is produced by the [`Generator`](crate::Generator) after processing
/// the intermediate [`MetaTypes`](crate::models::meta::MetaTypes). It serves as the
/// final intermediate representation used in the rendering stage to output Rust code.
///
/// The `items` map contains type-safe, idiomatic Rust representations for each schema
/// element, type, or attribute group encountered.
///
/// The `meta` field holds generator-specific configuration and state, such as flags,
/// postfix rules, and user-defined overrides, which influence the structure and naming
/// of generated code.
#[derive(Debug)]
pub struct DataTypes<'types> {
    /// Meta types and information from the generator process
    pub meta: GeneratorMetaData<'types>,

    /// Map of the different types.
    pub items: IdentMap<IndexMap<Ident, DataType<'types>>>,
}

impl<'types> DataTypes<'types> {
    pub(crate) fn new(meta: GeneratorMetaData<'types>) -> Self {
        let items = Default::default();

        Self { meta, items }
    }
}

impl<'types> Deref for DataTypes<'types> {
    type Target = GeneratorMetaData<'types>;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}
