use std::hash::{Hash, Hasher};
use std::ops::{Bound, Range};

use crate::models::Ident;

use super::{MetaTypes, TypeEq};

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

    /// Range the value should be in.
    pub range: Range<Bound<String>>,

    /// Number of total digits the value maximal should have.
    pub total_digits: Option<usize>,

    /// Number of fraction digits the value maximal should have.
    pub fraction_digits: Option<usize>,

    /// Regex pattern the value should fulfill.
    pub pattern: Option<String>,

    /// The minimum length the value should have.
    pub min_length: Option<usize>,

    /// The maximum length the value should have.
    pub max_length: Option<usize>,

    /// Defines the whitespace handling.
    pub whitespace: WhiteSpace,
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

impl TypeEq for SimpleMeta {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        let Self {
            base,
            range,
            total_digits,
            fraction_digits,
            pattern,
            min_length,
            max_length,
            whitespace,
        } = self;

        base.type_hash(hasher, types);
        range.hash(hasher);
        total_digits.hash(hasher);
        fraction_digits.hash(hasher);
        pattern.hash(hasher);
        min_length.hash(hasher);
        max_length.hash(hasher);
        whitespace.hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        let Self {
            base,
            range,
            total_digits,
            fraction_digits,
            pattern,
            min_length,
            max_length,
            whitespace,
        } = self;

        base.type_eq(&other.base, types)
            && range.eq(&other.range)
            && total_digits.eq(&other.total_digits)
            && fraction_digits.eq(&other.fraction_digits)
            && pattern.eq(&other.pattern)
            && min_length.eq(&other.min_length)
            && max_length.eq(&other.max_length)
            && whitespace.eq(&other.whitespace)
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
