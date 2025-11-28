//! Contains the [`Type`] type information and all related types.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use quote::format_ident;

use crate::models::code::IdentPath;
use crate::models::schema::{xs::FormChoiceType, SchemaId};

use super::{
    ComplexMeta, CustomMeta, DynamicMeta, EnumerationMeta, GroupMeta, MetaTypes, ReferenceMeta,
    SimpleMeta, TypeEq, UnionMeta,
};

/// Represents a fully interpreted type from an XML schema.
///
/// This is the core type wrapper used during schema interpretation. It contains
/// the actual type structure (`variant`), optional human-readable naming information,
/// and collected documentation from schema definitions (e.g. `xs:documentation`).
#[derive(Debug, Clone)]
pub struct MetaType {
    /// Actual data type this type represents.
    pub variant: MetaTypeVariant,

    /// Form for elements of this type.
    pub form: Option<FormChoiceType>,

    /// Id of the schema this type was defined at.
    pub schema: Option<SchemaId>,

    /// Name to use for rendering instead of the auto generated name.
    pub display_name: Option<String>,

    /// Documentation of the type extracted from `xs:documentation` nodes.
    pub documentation: Vec<String>,
}

/// Actual data variant a [`MetaType`] represents.
#[derive(Debug, Clone)]
pub enum MetaTypeVariant {
    /// Represents a union type
    Union(UnionMeta),

    /// Represents a build-in type
    BuildIn(BuildInMeta),

    /// Represents a user defined type
    Custom(CustomMeta),

    /// References an other type
    Reference(ReferenceMeta),

    /// Represents an enumeration
    Enumeration(EnumerationMeta),

    /// Represents an dynamic element
    Dynamic(DynamicMeta),

    /// Represents a specific set of elements
    All(GroupMeta),

    /// Represents a choice of different elements
    Choice(GroupMeta),

    /// Represents a sequence of different elements
    Sequence(GroupMeta),

    /// Represents a complex type
    ComplexType(ComplexMeta),

    /// Represents a simple type with additional restrictions
    SimpleType(SimpleMeta),
}

/// Union that defined the build in types of the rust language or
/// custom defined types.
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BuildInMeta {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,

    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,

    F32,
    F64,

    Bool,
    String,
}

/* Type */

macro_rules! impl_from {
    ($var:ident, $ty:ty) => {
        impl From<$ty> for MetaType {
            fn from(value: $ty) -> Self {
                MetaType::new(MetaTypeVariant::$var(value))
            }
        }
    };
}

impl_from!(Union, UnionMeta);
impl_from!(BuildIn, BuildInMeta);
impl_from!(Custom, CustomMeta);
impl_from!(Reference, ReferenceMeta);
impl_from!(Enumeration, EnumerationMeta);
impl_from!(Dynamic, DynamicMeta);
impl_from!(ComplexType, ComplexMeta);

impl MetaType {
    /// Create a new [`MetaType`] instance from the passed `variant`.
    #[must_use]
    pub fn new(variant: MetaTypeVariant) -> Self {
        Self {
            variant,
            form: None,
            schema: None,
            display_name: None,
            documentation: Vec::new(),
        }
    }

    /// Returns the form element of this type should have.
    #[must_use]
    pub fn form(&self) -> FormChoiceType {
        self.form.unwrap_or(FormChoiceType::Unqualified)
    }

    /// Returns `true` if this type is emptiable, `false` otherwise.
    ///
    /// Emptiable means that the type may not have any element.
    #[must_use]
    pub fn is_emptiable(&self, types: &MetaTypes) -> bool {
        self.variant.is_emptiable(types)
    }

    /// Returns `true` if this type is mixed, `false` otherwise.
    ///
    /// Mixed means, that the type also accepts text intermixed with it's elements.
    #[must_use]
    pub fn is_mixed(&self, types: &MetaTypes) -> bool {
        self.variant.is_mixed(types)
    }
}

impl Deref for MetaType {
    type Target = MetaTypeVariant;

    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

impl DerefMut for MetaType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variant
    }
}

impl TypeEq for MetaType {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
        #[allow(clippy::enum_glob_use)]
        use MetaTypeVariant::*;

        self.display_name.hash(hasher);

        match &self.variant {
            Union(x) => x.type_hash(hasher, types),
            Custom(x) => x.hash(hasher),
            BuildIn(x) => x.hash(hasher),
            Reference(x) => x.type_hash(hasher, types),
            Enumeration(x) => x.type_hash(hasher, types),
            Dynamic(x) => x.type_hash(hasher, types),
            All(x) => x.type_hash(hasher, types),
            Choice(x) => x.type_hash(hasher, types),
            Sequence(x) => x.type_hash(hasher, types),
            ComplexType(x) => x.type_hash(hasher, types),
            SimpleType(x) => x.type_hash(hasher, types),
        }
    }

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
        #[allow(clippy::enum_glob_use)]
        use MetaTypeVariant::*;

        if self.display_name != other.display_name {
            return false;
        }

        match (&self.variant, &other.variant) {
            (Union(x), Union(y)) => x.type_eq(y, types),
            (BuildIn(x), BuildIn(y)) => x == y,
            (Reference(x), Reference(y)) => x.type_eq(y, types),
            (Enumeration(x), Enumeration(y)) => x.type_eq(y, types),
            (Dynamic(x), Dynamic(y)) => x.type_eq(y, types),
            (All(x), All(y)) => x.type_eq(y, types),
            (Choice(x), Choice(y)) => x.type_eq(y, types),
            (Sequence(x), Sequence(y)) => x.type_eq(y, types),
            (ComplexType(x), ComplexType(y)) => x.type_eq(y, types),
            (SimpleType(x), SimpleType(y)) => x.type_eq(y, types),
            (_, _) => false,
        }
    }
}

/* MetaTypeVariant */

impl MetaTypeVariant {
    /// Returns `true` if this type is emptiable, `false` otherwise.
    ///
    /// Emptiable means that the type may not have any element.
    #[must_use]
    pub fn is_emptiable(&self, types: &MetaTypes) -> bool {
        match self {
            Self::Union(_) | Self::BuildIn(_) | Self::Enumeration(_) | Self::SimpleType(_) => true,
            Self::Custom(_) | Self::Dynamic(_) => false,
            Self::Reference(x) => x.is_emptiable(types),
            Self::All(x) | Self::Choice(x) | Self::Sequence(x) => x.is_emptiable(types),
            Self::ComplexType(x) => x.is_emptiable(types),
        }
    }

    /// Returns `true` if this type is mixed, `false` otherwise.
    ///
    /// Mixed means, that the type also accepts text intermixed with it's elements.
    #[must_use]
    pub fn is_mixed(&self, types: &MetaTypes) -> bool {
        match self {
            Self::Union(_)
            | Self::BuildIn(_)
            | Self::Enumeration(_)
            | Self::SimpleType(_)
            | Self::Custom(_)
            | Self::Dynamic(_) => false,
            Self::Reference(x) => x.is_mixed(types),
            Self::All(x) | Self::Choice(x) | Self::Sequence(x) => x.is_mixed,
            Self::ComplexType(x) => x.is_mixed,
        }
    }
}

/* BuildInMeta */

impl BuildInMeta {
    /// Get the name of the build-in type as `&str`.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::Usize => "usize",

            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::Isize => "isize",

            Self::F32 => "f32",
            Self::F64 => "f64",

            Self::Bool => "bool",
            Self::String => "String",
        }
    }

    /// Return the absolute path of the build in type as &str.
    #[must_use]
    pub fn as_absolute_path(&self) -> &'static str {
        match self {
            Self::U8 => "::core::primitive::u8",
            Self::U16 => "::core::primitive::u16",
            Self::U32 => "::core::primitive::u32",
            Self::U64 => "::core::primitive::u64",
            Self::U128 => "::core::primitive::u128",
            Self::Usize => "::core::primitive::usize",

            Self::I8 => "::core::primitive::i8",
            Self::I16 => "::core::primitive::i16",
            Self::I32 => "::core::primitive::i32",
            Self::I64 => "::core::primitive::i64",
            Self::I128 => "::core::primitive::i128",
            Self::Isize => "::core::primitive::isize",

            Self::F32 => "::core::primitive::f32",
            Self::F64 => "::core::primitive::f64",

            Self::Bool => "::core::primitive::bool",
            Self::String => "::std::string::String",
        }
    }

    /// Return the relative [`IdentPath`] of the build-in type.
    #[must_use]
    pub fn ident_path(&self) -> IdentPath {
        IdentPath::from_ident(format_ident!("{self}"))
    }

    /// Return the absolute [`IdentPath`] of the build-in type.
    #[must_use]
    pub fn absolute_ident_path(&self) -> IdentPath {
        use std::str::FromStr;

        IdentPath::from_str(self.as_absolute_path()).unwrap()
    }
}

impl Display for BuildInMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}
