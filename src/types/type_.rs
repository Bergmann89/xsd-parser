//! Contains the [`Type`] type information and all related types.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

use super::{
    ComplexInfo, CustomType, DynamicInfo, EnumerationInfo, GroupInfo, ReferenceInfo, Types,
    UnionInfo,
};

/// Represents a type that was read and interpreted from an XML schema.
#[derive(Debug, Clone)]
pub enum Type {
    /// Represents a union type
    Union(UnionInfo),

    /// Represents a build-in type
    BuildIn(BuildInInfo),

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

/// Trait to check if two types are equal to each other or not.
///
/// This trait will automatically resolve type definitions to its target
/// type before it compares the two instances. This means a type is considered
/// as equal, if all type identifiers point to the same type and all normal
/// values are equal.
pub trait TypeEq: Sized {
    /// Feeds this value into the given [`Hasher`].
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types);

    /// Feeds a slice of this value into the given [`Hasher`].
    fn type_hash_slice<H: Hasher>(slice: &[Self], hasher: &mut H, types: &Types) {
        hasher.write_usize(slice.len());
        for item in slice {
            item.type_hash(hasher, types);
        }
    }

    /// Check if this instance is equal to the `other` instance using the passed
    /// `types` to resolve identifiers.
    fn type_eq(&self, other: &Self, types: &Types) -> bool;

    /// Check if the two passed iterators contain type equal elements.
    fn type_eq_iter<'a, X, Y>(x: X, y: Y, types: &Types) -> bool
    where
        Self: 'a,
        X: IntoIterator<Item = &'a Self>,
        Y: IntoIterator<Item = &'a Self>,
    {
        let mut x = x.into_iter();
        let mut y = y.into_iter();

        loop {
            match (x.next(), y.next()) {
                (None, None) => return true,
                (Some(x), Some(y)) => {
                    if !x.type_eq(y, types) {
                        return false;
                    }
                }
                (_, _) => return false,
            }
        }
    }
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

    Custom(CustomType),
}

/* Type */

macro_rules! impl_from {
    ($var:ident, $ty:ty) => {
        impl From<$ty> for Type {
            fn from(value: $ty) -> Self {
                Self::$var(value)
            }
        }
    };
}

impl_from!(Reference, ReferenceInfo);
impl_from!(BuildIn, BuildInInfo);
impl_from!(Enumeration, EnumerationInfo);

impl TypeEq for Type {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        #[allow(clippy::enum_glob_use)]
        use Type::*;

        match self {
            Union(x) => x.type_hash(hasher, types),
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
        use Type::*;

        match (self, other) {
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

            Self::Custom(x) => x.name(),
        }
    }
}

impl Display for BuildInInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_str())
    }
}

/* TypeEq */

impl<T> TypeEq for Option<T>
where
    T: TypeEq,
{
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        if let Some(inner) = self {
            hasher.write_u8(1);
            inner.type_hash(hasher, types);
        } else {
            hasher.write_u8(0);
        }
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        match (self, other) {
            (Some(x), Some(y)) => x.type_eq(y, types),
            (None, None) => true,
            (_, _) => false,
        }
    }
}
