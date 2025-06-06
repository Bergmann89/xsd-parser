//! Contains the [`Type`] type information and all related types.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use super::{
    ComplexInfo, CustomInfo, DynamicInfo, EnumerationInfo, GroupInfo, ReferenceInfo, TypeEq, Types,
    UnionInfo,
};

/// Represents a type that was read and interpreted from an XML schema.
#[derive(Debug, Clone)]
pub struct Type {
    /// Actual data type this type represents.
    pub variant: TypeVariant,

    /// Name to use for rendering instead of the auto generated name.
    pub display_name: Option<String>,

    /// Documentation of the type extracted from `xs:documentation` nodes.
    pub documentation: Vec<String>,
}

/// Actual data type a [`Type`] represents.
#[derive(Debug, Clone)]
pub enum TypeVariant {
    /// Represents a union type
    Union(UnionInfo),

    /// Represents a build-in type
    BuildIn(BuildInInfo),

    /// Represents a user defined type
    Custom(CustomInfo),

    /// References an other type
    Reference(ReferenceInfo),

    /// Represents an enumeration
    Enumeration(EnumerationInfo),

    /// Represents an dynamic element
    Dynamic(DynamicInfo),

    /// Represents a specific set of elements
    All(GroupInfo),

    /// Represents a choice of different elements
    Choice(GroupInfo),

    /// Represents a sequence of different elements
    Sequence(GroupInfo),

    /// Represents a complex type
    ComplexType(ComplexInfo),
}

/// Union that defined the build in types of the rust language or
/// custom defined types.
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BuildInInfo {
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
        impl From<$ty> for Type {
            fn from(value: $ty) -> Self {
                Type::new(TypeVariant::$var(value))
            }
        }
    };
}

impl_from!(Union, UnionInfo);
impl_from!(BuildIn, BuildInInfo);
impl_from!(Custom, CustomInfo);
impl_from!(Reference, ReferenceInfo);
impl_from!(Enumeration, EnumerationInfo);
impl_from!(Dynamic, DynamicInfo);
impl_from!(ComplexType, ComplexInfo);

impl Type {
    /// Create a new [`Type`] instance from the passed `variant`.
    #[must_use]
    pub fn new(variant: TypeVariant) -> Self {
        Self {
            variant,
            display_name: None,
            documentation: Vec::new(),
        }
    }
}

impl Deref for Type {
    type Target = TypeVariant;

    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

impl DerefMut for Type {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variant
    }
}

impl TypeEq for Type {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        #[allow(clippy::enum_glob_use)]
        use TypeVariant::*;

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
        }
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        #[allow(clippy::enum_glob_use)]
        use TypeVariant::*;

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
            (_, _) => false,
        }
    }
}

/* BuildInInfo */

impl BuildInInfo {
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
}

impl Display for BuildInInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}
