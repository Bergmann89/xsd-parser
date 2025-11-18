//! Contains the [`EnumerationMeta`] type information and all related types.

use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use crate::models::{schema::xs::Use, Ident};

use super::{Base, Constrains, MetaTypes, TypeEq};

/// Type information that defines an enumeration type.
#[derive(Default, Debug, Clone)]
pub struct EnumerationMeta {
    /// Base type of this enumeration type.
    pub base: Base,

    /// Variants defined for this enumeration.
    pub variants: EnumerationMetaVariants,

    /// Constraining facets defined for this type.
    pub constrains: Constrains,
}

/// Type information that defines variants of an [`EnumerationMeta`].
#[derive(Debug, Clone)]
pub struct EnumerationMetaVariant {
    /// Identifier of the variant.
    pub ident: Ident,

    /// Use of the variant.
    pub use_: Use,

    /// Type of the variant.
    pub type_: Option<Ident>,

    /// Name of the variant to use inside the generated code.
    pub display_name: Option<String>,

    /// Documentation of the type extracted from `xs:documentation` nodes.
    pub documentation: Vec<String>,
}

/// Type information that represents a list of [`EnumerationMetaVariant`] instances.
#[derive(Default, Debug, Clone)]
pub struct EnumerationMetaVariants(pub Vec<EnumerationMetaVariant>);

/* EnumerationMeta */

impl TypeEq for EnumerationMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            base,
            variants,
            constrains,
        } = self;

        base.type_hash(hasher, types);
        variants.type_hash(hasher, types);
        constrains.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            base,
            variants,
            constrains,
        } = self;

        base.type_eq(&other.base, types)
            && variants.type_eq(&other.variants, types)
            && constrains.eq(&other.constrains)
    }
}

/* EnumerationMetaVariant */

impl EnumerationMetaVariant {
    /// Create a new [`EnumerationMetaVariant`] instance from the passed `name`.
    #[must_use]
    pub fn new(ident: Ident) -> Self {
        Self {
            ident,
            use_: Use::Optional,
            type_: None,
            display_name: None,
            documentation: Vec::new(),
        }
    }

    /// Set the type of this variant information.
    #[must_use]
    pub fn with_type(mut self, type_: Option<Ident>) -> Self {
        self.type_ = type_;

        self
    }
}

impl TypeEq for EnumerationMetaVariant {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            ident,
            use_,
            type_,
            display_name,
            documentation,
        } = self;

        ident.hash(hasher);
        use_.hash(hasher);
        type_.type_hash(hasher, types);
        display_name.hash(hasher);
        documentation.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            ident,
            use_,
            type_,
            display_name,
            documentation,
        } = self;

        ident.eq(&other.ident)
            && use_.eq(&other.use_)
            && type_.type_eq(&other.type_, types)
            && display_name.eq(&other.display_name)
            && documentation.eq(&other.documentation)
    }
}

/* EnumerationMetaVariants */

impl Deref for EnumerationMetaVariants {
    type Target = Vec<EnumerationMetaVariant>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EnumerationMetaVariants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeEq for EnumerationMetaVariants {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        TypeEq::type_hash_slice(&self.0, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        TypeEq::type_eq_iter(self.0.iter(), other.0.iter(), types)
    }
}
