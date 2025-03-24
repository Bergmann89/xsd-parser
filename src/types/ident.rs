//! Contains the [`Ident`] helper type and all related types.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

use crate::schema::NamespaceId;

use super::{Name, TypeEq, Types};

/// Represents a type identifier.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ident {
    /// Namespace the type is defined in
    pub ns: Option<NamespaceId>,

    /// Name of the type.
    pub name: Name,

    /// Type of the identifier (because pure names are not unique in XSD).
    pub type_: IdentType,
}

/// Type of the identifier.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IdentType {
    /// Used for `xs:simpleType` or `xs:complexType`.
    Type = 0,

    /// Used for `xs:group`.
    Group = 1,

    /// Used for `xs:element`.
    Element = 2,

    /// Used for inline types of `xs:element`.
    ElementType = 3,

    /// Used for `xs:attribute`.
    Attribute = 4,

    /// Used for `xs:attributeGroup`.
    AttributeGroup = 5,

    /// Used for build-in types.
    BuildIn = 6,

    /// Used for `xs:enumeration`.
    Enumeration = 7,
}

#[allow(missing_docs)]
impl Ident {
    pub const U8: Self = Self::build_in("u8");
    pub const U16: Self = Self::build_in("u16");
    pub const U32: Self = Self::build_in("u32");
    pub const U64: Self = Self::build_in("u64");
    pub const U128: Self = Self::build_in("u128");
    pub const USIZE: Self = Self::build_in("usize");

    pub const I8: Self = Self::build_in("i8");
    pub const I16: Self = Self::build_in("i16");
    pub const I32: Self = Self::build_in("i32");
    pub const I64: Self = Self::build_in("i64");
    pub const I128: Self = Self::build_in("i128");
    pub const ISIZE: Self = Self::build_in("isize");

    pub const F32: Self = Self::build_in("f32");
    pub const F64: Self = Self::build_in("f64");

    pub const BOOL: Self = Self::build_in("bool");
    pub const STRING: Self = Self::build_in("String");

    pub const ANY_TYPE: Self = Self::type_("anyType");

    pub const BUILD_IN: &[Self] = &[
        Self::U8,
        Self::U16,
        Self::U32,
        Self::U64,
        Self::U128,
        Self::USIZE,
        Self::I8,
        Self::I16,
        Self::I32,
        Self::I64,
        Self::I128,
        Self::ISIZE,
        Self::F32,
        Self::F64,
        Self::BOOL,
        Self::STRING,
    ];
}

impl Ident {
    /// Create an [`Type`](IdentType::Type) [`Ident`]ifier with the given `name`.
    #[must_use]
    pub fn new(name: Name) -> Self {
        Self {
            ns: None,
            name,
            type_: IdentType::Type,
        }
    }

    /// Create an [`Type`](IdentType::Type) [`Ident`]ifier with the given `name`.
    #[must_use]
    pub const fn type_(name: &'static str) -> Self {
        Self {
            ns: None,
            name: Name::named(name),
            type_: IdentType::Type,
        }
    }

    /// Create an [`BuildIn`](IdentType::BuildIn) [`Ident`]ifier with the given `name`.
    #[must_use]
    pub const fn build_in(name: &'static str) -> Self {
        Self {
            ns: None,
            name: Name::named(name),
            type_: IdentType::BuildIn,
        }
    }

    /// Create an [`Element`](IdentType::Element) [`Ident`]ifier with the given `name`.
    #[must_use]
    pub const fn element(name: &'static str) -> Self {
        Self {
            ns: None,
            name: Name::named(name),
            type_: IdentType::Element,
        }
    }

    /// Create an [`Ident`]ifier suitable for field names with the given `name`.
    #[must_use]
    pub const fn name(name: &'static str) -> Self {
        // We do not have a separate `IdentType` for fields, so we just use `IdentType::Type`
        Self::type_(name)
    }

    /// Set the namespace of the identifier.
    #[must_use]
    pub fn with_ns(mut self, ns: Option<NamespaceId>) -> Self {
        self.ns = ns;

        self
    }

    /// Set the type of the identifier.
    #[must_use]
    pub fn with_type(mut self, type_: IdentType) -> Self {
        self.type_ = type_;

        self
    }

    /// Returns `true` if this is build-in type of the rust language, `false` otherwise.
    #[must_use]
    pub fn is_build_in(&self) -> bool {
        Ident::BUILD_IN.contains(self)
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let Self { ns, name, type_ } = self;

        match type_ {
            IdentType::Type => write!(f, "Type(")?,
            IdentType::Group => write!(f, "Group(")?,
            IdentType::BuildIn => write!(f, "BuildIn(")?,
            IdentType::Element => write!(f, "Element(")?,
            IdentType::ElementType => write!(f, "ElementType(")?,
            IdentType::Attribute => write!(f, "Attribute(")?,
            IdentType::AttributeGroup => write!(f, "AttributeGroup(")?,
            IdentType::Enumeration => write!(f, "Enumeration(")?,
        }

        if f.sign_minus() {
            write!(f, "{name})")?;
        } else if let Some(ns) = ns {
            write!(f, "ns={}, name={name})", ns.0)?;
        } else {
            write!(f, "ns=default, name={name})")?;
        }

        Ok(())
    }
}

impl TypeEq for Ident {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        types.get_resolved_ident(self).unwrap_or(self).hash(hasher);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        let a = types.get_resolved_ident(self).unwrap_or(self);
        let b = types.get_resolved_ident(other).unwrap_or(other);

        a == b
    }
}
