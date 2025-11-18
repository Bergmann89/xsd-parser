use std::hash::{Hash, Hasher};

use crate::models::Ident;

use super::{Constrains, MetaTypes, TypeEq};

/// Type information that contains data about a simple type including
/// it's potential restrictions.
///
/// If a simple type definition has additional restrictions (like `xs:minExclusive`
/// or `xs:minLength`) it is represented as [`SimpleType`](super::MetaTypeVariant::SimpleType)
/// instead of a simple [`Reference`](super::MetaTypeVariant::Reference).
#[derive(Debug, Clone)]
pub struct SimpleMeta {
    /// Type that is referenced.
    pub base: Ident,

    /// `true` if this simple type is a list, `false` otherwise.
    pub is_list: bool,

    /// Constraining facets defined for this type.
    pub constrains: Constrains,
}

/// Defines how to deal with whitespaces inside a XML element.
#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum WhiteSpace {
    /// Whitespace is kept exactly as written.
    #[default]
    Preserve,

    /// Tabs, line feeds, and carriage returns are replaced with spaces.
    Replace,

    /// All whitespace sequences are collapsed to a single space, and
    /// leading/trailing whitespace is removed.
    Collapse,
}

impl SimpleMeta {
    /// Create a new [`SimpleMeta`] instance from the passed `base` identifier.
    #[must_use]
    pub fn new(base: Ident) -> Self {
        Self {
            base,
            is_list: false,
            constrains: Constrains::default(),
        }
    }
}

impl TypeEq for SimpleMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            base,
            is_list,
            constrains,
        } = self;

        base.type_hash(hasher, types);
        is_list.hash(hasher);
        constrains.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            base,
            is_list,
            constrains,
        } = self;

        base.type_eq(&other.base, types)
            && is_list.eq(&other.is_list)
            && constrains.eq(&other.constrains)
    }
}

impl TypeEq for WhiteSpace {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let _types = types;

        self.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let _types = types;

        self.eq(other)
    }
}
