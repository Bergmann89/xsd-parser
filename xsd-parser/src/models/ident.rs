//! Defines the [`TypeIdent`] and [`NodeIdent`] type and all related helper types.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::Hash;

use crate::models::schema::{NamespaceId, SchemaId};

use super::Name;

/// Type that is used to identify types in the schema definition.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypeIdent {
    /// Namespace the type is defined in
    pub ns: NamespaceId,

    /// Id of the schema file the type pointed to by this identifier was defined
    /// at.
    ///
    /// This is needed to support the case when identifier are duplicated across
    /// schema files, or redefined by `xs:redefine` or `xs:override`.
    pub schema: SchemaId,

    /// Name of the type.
    pub name: Name,

    /// Type of the identifier (because pure names are not unique in XSD).
    pub type_: TypeIdentType,
}

/// Defines the type of the [`TypeIdent`].
pub type TypeIdentType = IdentType;

/// Type that is used to identify properties (elements, attributes, enumerations, ...)
/// in the schema definition.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PropertyIdent {
    /// Namespace the type is defined in
    pub ns: NamespaceId,

    /// Name of the type.
    pub name: Name,
}

/// Identifier used to identify elements in the
/// [`ElementsMeta`](crate::models::meta::ElementsMeta) structure.
pub type ElementIdent = PropertyIdent;

/// Identifier used to identify attributes in the
/// [`AttributesMeta`](crate::models::meta::AttributesMeta) structure.
pub type AttributeIdent = PropertyIdent;

/// Identifier used to identify enumerations in the
/// [`EnumerationMetaVariants`](crate::models::meta::EnumerationMetaVariants)
/// structure.
pub type EnumerationIdent = PropertyIdent;

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

    /// Used to define the content for [`Nillable`](xsd_parser_types::xml::Nillable) types.
    NillableContent = 8,

    /// One concrete element in a substitution group.
    DynamicElement = 9,
}

/* TypeIdent */

#[allow(missing_docs)]
impl TypeIdent {
    pub const UNKNOWN: Self = Self::type_("UNKNOWN");

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

impl TypeIdent {
    /// Create a new [`TypeIdent`] instance with [`IdentType::Type`] and the passed `name`.
    #[must_use]
    pub const fn new(name: Name) -> Self {
        Self {
            ns: NamespaceId::UNKNOWN,
            schema: SchemaId::UNKNOWN,
            name,
            type_: IdentType::Type,
        }
    }

    /// Create a new [`TypeIdent`] instance with [`IdentType::Type`] and the passed `name`.
    #[must_use]
    pub const fn type_(name: &'static str) -> Self {
        Self {
            ns: NamespaceId::UNKNOWN,
            schema: SchemaId::UNKNOWN,
            name: Name::named(name),
            type_: IdentType::Type,
        }
    }

    /// Create a new [`TypeIdent`] instance with [`IdentType::BuildIn`] and the passed `name`.
    #[must_use]
    pub const fn build_in(name: &'static str) -> Self {
        Self {
            ns: NamespaceId::ANONYMOUS,
            schema: SchemaId::UNKNOWN,
            name: Name::named(name),
            type_: IdentType::BuildIn,
        }
    }

    /// Create a new [`TypeIdent`] instance with [`IdentType::Element`] and the passed `name`.
    #[must_use]
    pub const fn element(name: &'static str) -> Self {
        Self {
            ns: NamespaceId::UNKNOWN,
            schema: SchemaId::UNKNOWN,
            name: Name::named(name),
            type_: IdentType::Element,
        }
    }

    /// Change the [`NamespaceId`] of this type identifier.
    #[must_use]
    pub fn with_ns(mut self, ns: NamespaceId) -> Self {
        self.ns = ns;

        self
    }

    /// Change the [`SchemaId`] of this type identifier.
    #[must_use]
    pub fn with_schema(mut self, schema: SchemaId) -> Self {
        self.schema = schema;

        self
    }

    /// Change the [`Name`] of this type identifier.
    #[must_use]
    pub fn with_name(mut self, name: Name) -> Self {
        self.name = name;

        self
    }

    /// Change the [`TypeIdentType`] of this type identifier.
    #[must_use]
    pub fn with_type(mut self, type_: TypeIdentType) -> Self {
        self.type_ = type_;

        self
    }

    /// Returns `true` if the passed identifier matches the current one.
    ///
    /// This ignores the values stored in [`ns`](TypeIdent::ns) and
    /// [`schema`](TypeIdent::schema) if they are set to `UNKNOWN`.
    #[must_use]
    pub fn matches(&self, other: &Self) -> bool {
        let Self {
            ns,
            schema,
            name,
            type_,
        } = self;

        (ns.is_unknown() || other.ns.is_unknown() || ns.eq(&other.ns))
            && (schema.is_unknown() || other.schema.is_unknown() || schema.eq(&other.schema))
            && name.eq(&other.name)
            && type_.eq(&other.type_)
    }

    /// Convert this type identifier into a [`PropertyIdent`] by dropping the schema
    /// and type information.
    #[must_use]
    pub fn to_property_ident(&self) -> PropertyIdent {
        let Self {
            ns,
            schema: _,
            name,
            type_: _,
        } = self;

        PropertyIdent {
            ns: *ns,
            name: name.clone(),
        }
    }

    /// Returns `true` if this is build-in type of the rust language, `false` otherwise.
    #[must_use]
    pub fn is_build_in(&self) -> bool {
        TypeIdent::BUILD_IN.contains(self)
    }

    /// Returns `true` if this identifier is fully qualified, `false` otherwise.
    ///
    /// The identifier is fully qualified, if neither the [`ns`](TypeIdent::ns),
    /// nor the [`schema`](TypeIdent::schema) is set to `UNKNOWN`.
    #[must_use]
    pub fn is_fully_qualified(&self) -> bool {
        !self.ns.is_unknown() && !self.schema.is_unknown()
    }
}

impl Display for TypeIdent {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_ident(
            f,
            Some(self.ns),
            Some(self.schema),
            &self.name,
            Some(self.type_),
        )
    }
}

/* PropertyIdent */

impl PropertyIdent {
    /// Create a new [`PropertyIdent`] instance with the passed `name`.
    #[inline]
    #[must_use]
    pub const fn new(name: Name) -> Self {
        Self {
            ns: NamespaceId::UNKNOWN,
            name,
        }
    }

    /// Create a new [`PropertyIdent`] instance with the passed `name`.
    #[inline]
    #[must_use]
    pub const fn named(name: &'static str) -> Self {
        Self {
            ns: NamespaceId::UNKNOWN,
            name: Name::named(name),
        }
    }

    /// Change the [`NamespaceId`] of this identifier.
    #[inline]
    #[must_use]
    pub fn with_ns(mut self, ns: NamespaceId) -> Self {
        self.ns = ns;

        self
    }
}

impl Display for PropertyIdent {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fmt_ident(f, Some(self.ns), None, &self.name, None)
    }
}

fn fmt_ident(
    f: &mut Formatter<'_>,
    ns: Option<NamespaceId>,
    schema: Option<SchemaId>,
    name: &Name,
    type_: Option<IdentType>,
) -> FmtResult {
    match type_ {
        None => write!(f, "Ident(")?,
        Some(IdentType::Type) => write!(f, "Type(")?,
        Some(IdentType::Group) => write!(f, "Group(")?,
        Some(IdentType::BuildIn) => write!(f, "BuildIn(")?,
        Some(IdentType::Element) => write!(f, "Element(")?,
        Some(IdentType::ElementType) => write!(f, "ElementType(")?,
        Some(IdentType::Attribute) => write!(f, "Attribute(")?,
        Some(IdentType::AttributeGroup) => write!(f, "AttributeGroup(")?,
        Some(IdentType::Enumeration) => write!(f, "Enumeration(")?,
        Some(IdentType::NillableContent) => write!(f, "NillableContent(")?,
        Some(IdentType::DynamicElement) => write!(f, "DynamicElement(")?,
    }

    if f.sign_minus() {
        write!(f, "{name})")?;
    } else {
        if let Some(SchemaId(schema)) = schema {
            write!(f, "schema={schema}, ")?;
        }

        if let Some(ns) = ns {
            write!(f, "ns={}, name={name})", ns.0)?;
        } else {
            write!(f, "ns=default, name={name})")?;
        }
    }

    Ok(())
}
