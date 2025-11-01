use std::ops::Deref;

use crate::models::{code::IdentPath, meta::MetaType};

use super::{
    BuildInData, ComplexData, ConfigValue, CustomData, DynamicData, EnumerationData, ReferenceData,
    SimpleData, UnionData,
};

/// Represents a fully evaluated and enriched type definition used for rendering.
///
/// A `DataType` wraps a [`MetaType`] and augments it with generator-evaluated
/// context required by the rendering phase. This avoids duplicating logic in
/// multiple renderers by providing a preprocessed, render-ready structure.
///
/// Each `DataType` includes both a reference to the original meta type and
/// a [`DataTypeVariant`] which contains the specific generator output based
/// on the kind of the underlying type.
#[derive(Debug)]
pub struct DataType<'types> {
    /// Underlying meta type this data type contains additional information for.
    pub meta: &'types MetaType,

    /// Defines traits to derive the type from.
    pub derive: ConfigValue<Vec<IdentPath>>,

    /// The variant of the data type.
    pub variant: DataTypeVariant<'types>,
}

/// Represents the specific form of a [`DataType`] as evaluated by the generator.
///
/// Each variant corresponds to a variant of [`MetaTypeVariant`](crate::models::meta::MetaTypeVariant)
/// and contains generator-enriched data specific to that type kind, e.g. a Rust struct or enum.
///
/// This abstraction simplifies renderer logic by encapsulating all decisions
/// needed to render idiomatic Rust code for each type.
#[derive(Debug)]
pub enum DataTypeVariant<'types> {
    /// Corresponds to [`MetaTypeVariant::BuildIn`](crate::models::meta::MetaTypeVariant::BuildIn).
    BuildIn(BuildInData<'types>),

    /// Corresponds to [`MetaTypeVariant::Custom`](crate::models::meta::MetaTypeVariant::Custom).
    Custom(CustomData<'types>),

    /// Corresponds to [`MetaTypeVariant::Union`](crate::models::meta::MetaTypeVariant::Union).
    Union(UnionData<'types>),

    /// Corresponds to [`MetaTypeVariant::Dynamic`](crate::models::meta::MetaTypeVariant::Dynamic).
    Dynamic(DynamicData<'types>),

    /// Corresponds to [`MetaTypeVariant::Reference`](crate::models::meta::MetaTypeVariant::Reference).
    Reference(ReferenceData<'types>),

    /// Corresponds to [`MetaTypeVariant::Enumeration`](crate::models::meta::MetaTypeVariant::Enumeration).
    Enumeration(EnumerationData<'types>),

    /// Corresponds to [`MetaTypeVariant::All`](crate::models::meta::MetaTypeVariant::All),
    /// [`MetaTypeVariant::Choice`](crate::models::meta::MetaTypeVariant::Choice),
    /// [`MetaTypeVariant::Sequence`](crate::models::meta::MetaTypeVariant::Sequence) or
    /// [`MetaTypeVariant::ComplexType`](crate::models::meta::MetaTypeVariant::ComplexType).
    Complex(ComplexData<'types>),

    /// Corresponds to [`MetaTypeVariant::SimpleType`](crate::models::meta::MetaTypeVariant::SimpleType).
    Simple(SimpleData<'types>),
}

impl Deref for DataType<'_> {
    type Target = MetaType;

    fn deref(&self) -> &Self::Target {
        self.meta
    }
}
