use std::ops::Deref;

use indexmap::IndexMap;

use crate::models::Ident;
use crate::pipeline::generator::MetaData as GeneratorMetaData;

use super::DataType;

/// This structure contains information about the generated data types.
///
/// It is created by the [`Generator`](crate::Generator) by using the information
/// from the [`MetaTypes`](crate::models::meta::MetaTypes) structure. The information
/// of this structure can be rendered to actual code using the [`Renderers`](crate::Renderers).
#[derive(Debug)]
pub struct DataTypes<'types> {
    /// Meta types
    pub meta: GeneratorMetaData<'types>,

    /// Map of the different types.
    pub items: IndexMap<Ident, DataType<'types>>,
}

impl<'types> DataTypes<'types> {
    pub(crate) fn new(meta: GeneratorMetaData<'types>) -> Self {
        let items = IndexMap::default();

        Self { meta, items }
    }
}

impl<'types> Deref for DataTypes<'types> {
    type Target = GeneratorMetaData<'types>;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}
