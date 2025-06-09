use std::ops::Deref;

use crate::config::{BoxFlags, GeneratorFlags, TypedefMode};
use crate::models::{code::IdentPath, meta::MetaTypes};

/// Meta data of the generator process.
///
/// Contains different information and data that is useful during the code
/// generation process.
#[derive(Debug)]
pub struct MetaData<'types> {
    /// Reference to the types the code should be generated for.
    pub types: &'types MetaTypes,

    /// Flags that controls the behavior of the generator.
    pub flags: GeneratorFlags,

    /// List of postfixed to add to the name of the generated types.
    ///
    /// This corresponds to the variants of [`IdentType`](crate::models::IdentType).
    pub postfixes: [String; 8],

    /// Tells the generator how to deal with boxed elements.
    pub box_flags: BoxFlags,

    /// Tells the generator how to deal with type definitions.
    pub typedef_mode: TypedefMode,

    /// Type to use to store unstructured `xs:any` elements.
    pub any_type: Option<IdentPath>,

    /// Type to use to store unstructured `xs:anyAttribute` attributes.
    pub any_attribute_type: Option<IdentPath>,
}

impl MetaData<'_> {
    /// Whether the passed `flags` intersect with the generator flags set in
    /// the configuration, or not.
    #[must_use]
    pub fn check_generator_flags(&self, flags: GeneratorFlags) -> bool {
        self.flags.intersects(flags)
    }
}

impl Deref for MetaData<'_> {
    type Target = MetaTypes;

    fn deref(&self) -> &Self::Target {
        self.types
    }
}
