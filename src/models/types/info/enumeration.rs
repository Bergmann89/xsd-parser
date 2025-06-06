//! Contains the [`EnumerationInfo`] type information and all related types.

use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use crate::models::{
    schema::xs::Use,
    types::{TypeEq, Types},
    Ident,
};

use super::{use_hash, Base};

/// Type information that defines an enumeration type.
#[derive(Default, Debug, Clone)]
pub struct EnumerationInfo {
    /// Base type of this enumeration type.
    pub base: Base,

    /// Variants defined for this enumeration.
    pub variants: VariantsInfo,
}

/// Type information that defines variants of an [`EnumerationInfo`].
#[derive(Debug, Clone)]
pub struct VariantInfo {
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

/// Type information that represents a list of [`VariantInfo`] instances.
#[derive(Default, Debug, Clone)]
pub struct VariantsInfo(pub Vec<VariantInfo>);

/* EnumerationInfo */

impl TypeEq for EnumerationInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self { base, variants } = self;

        base.type_hash(hasher, types);
        variants.type_hash(hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self { base, variants } = self;

        base.type_eq(&other.base, types) && variants.type_eq(&other.variants, types)
    }
}

/* VariantInfo */

impl VariantInfo {
    /// Create a new [`VariantInfo`] instance from the passed `name`.
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

impl TypeEq for VariantInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self {
            ident,
            use_,
            type_,
            display_name,
            documentation,
        } = self;

        ident.hash(hasher);
        use_hash(use_, hasher);
        type_.type_hash(hasher, types);
        display_name.hash(hasher);
        documentation.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
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

/* VariantsInfo */

impl Deref for VariantsInfo {
    type Target = Vec<VariantInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VariantsInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeEq for VariantsInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        TypeEq::type_hash_slice(&self.0, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        TypeEq::type_eq_iter(self.0.iter(), other.0.iter(), types)
    }
}
