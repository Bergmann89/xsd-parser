//! Contains the [`AttributeInfo`] type information and all related types.

use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use crate::schema::xs::Use;
use crate::types::{Ident, TypeEq, Types};

use super::use_hash;

/// Type information that contains data about attribute definitions.
#[derive(Debug, Clone)]
pub struct AttributeInfo {
    /// Identifier of the attribute.
    pub ident: Ident,

    /// Type of the attribute.
    pub type_: Ident,

    /// Usage of the attribute.
    pub use_: Use,

    /// Default value of the attribute.
    pub default: Option<String>,
}

/// Type information that represents a list of [`AttributeInfo`] instances.
#[derive(Default, Debug, Clone)]
pub struct AttributesInfo(Vec<AttributeInfo>);

/* AttributeInfo */

impl AttributeInfo {
    /// Create a new [`AttributeInfo`] instance from the passed `name` and `type_`.
    #[must_use]
    pub fn new(ident: Ident, type_: Ident) -> Self {
        Self {
            ident,
            type_,
            use_: Use::Optional,
            default: None,
        }
    }

    /// Set the [`Use`] value of the attribute.
    #[must_use]
    pub fn with_use(mut self, use_: Use) -> Self {
        self.use_ = use_;

        self
    }
}

impl TypeEq for AttributeInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        let Self {
            ident,
            type_,
            use_,
            default,
        } = self;

        ident.hash(hasher);
        type_.type_hash(hasher, types);
        use_hash(use_, hasher);
        default.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let Self {
            ident,
            type_,
            use_,
            default,
        } = self;

        ident.eq(&other.ident)
            && type_.type_eq(&other.type_, types)
            && use_.eq(&other.use_)
            && default.eq(&other.default)
    }
}

/* AttributesInfo */

impl Deref for AttributesInfo {
    type Target = Vec<AttributeInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AttributesInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeEq for AttributesInfo {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        TypeEq::type_hash_slice(&self.0, hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        TypeEq::type_eq_iter(self.0.iter(), other.0.iter(), types)
    }
}
