//! Contains the [`Type`] type information and all related types.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use super::{
    ComplexInfo, CustomType, DynamicInfo, EnumerationInfo, GroupInfo, ReferenceInfo, Types,
    UnionInfo,
};

/// The form of a simple type.
pub type SimpleType = TypeDescriptor<SimpleTypeVariant>;

/// The form of a complex type.
pub type ComplexType = TypeDescriptor<ComplexTypeVariant>;

/// Either a [`SimpleType`] or a [`ComplexType`].
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Type {
    /// A simple type.
    SimpleType(SimpleType),
    /// A complex type.
    ComplexType(ComplexType),
}

impl Type {
    /// Returns a reference to the display name of this type.
    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        match self {
            Type::SimpleType(TypeDescriptor { display_name, .. })
            | Type::ComplexType(TypeDescriptor { display_name, .. }) => display_name.as_deref(),
        }
    }

    /// Returns a mutable reference to the display name of this type.
    pub fn display_name_mut(&mut self) -> &mut Option<String> {
        match self {
            Type::SimpleType(TypeDescriptor { display_name, .. })
            | Type::ComplexType(TypeDescriptor { display_name, .. }) => display_name,
        }
    }

    /// Returns a reference to the variant of this type.
    #[must_use]
    pub fn variant(&self) -> TypeVariant<&SimpleTypeVariant, &ComplexTypeVariant> {
        match self {
            Type::SimpleType(type_descriptor) => TypeVariant::SimpleType(&type_descriptor.variant),
            Type::ComplexType(type_descriptor) => {
                TypeVariant::ComplexType(&type_descriptor.variant)
            }
        }
    }

    /// Returns a mutable reference to the variant of this type.
    pub fn variant_mut(&mut self) -> TypeVariant<&mut SimpleTypeVariant, &mut ComplexTypeVariant> {
        match self {
            Type::SimpleType(type_descriptor) => {
                TypeVariant::SimpleType(&mut type_descriptor.variant)
            }
            Type::ComplexType(type_descriptor) => {
                TypeVariant::ComplexType(&mut type_descriptor.variant)
            }
        }
    }

    /// Returns the simple type if this type is a simple type.
    #[must_use]
    pub fn simple_type_ref(&self) -> Option<&SimpleType> {
        match self {
            Type::SimpleType(type_descriptor) => Some(type_descriptor),
            _ => None,
        }
    }

    /// Returns the complex type if this type is a complex type.
    #[must_use]
    pub fn complex_type_ref(&self) -> Option<&ComplexType> {
        match self {
            Type::ComplexType(type_descriptor) => Some(type_descriptor),
            _ => None,
        }
    }

    /// Returns the reference info if this type is a reference.
    #[must_use]
    pub fn reference_type(&self) -> Option<&ReferenceInfo> {
        match self {
            Type::SimpleType(TypeDescriptor {
                variant: SimpleTypeVariant::Reference(ri),
                ..
            })
            | Type::ComplexType(TypeDescriptor {
                variant: ComplexTypeVariant::Reference(ri),
                ..
            }) => Some(ri),
            _ => None,
        }
    }
}

/// Represents a type that was read and interpreted from an XML schema.
#[derive(Debug, Clone)]
pub struct TypeDescriptor<T> {
    /// Name to use for rendering instead of the auto generated name.
    pub display_name: Option<String>,

    /// Actual data type this type represents.
    pub variant: T,
}

/// Represents either a simple or complex type variant, with generics for dereferenced variants.
#[derive(Debug, Clone)]
pub enum TypeVariant<S, C> {
    /// Represents a simple type variant.
    SimpleType(S),
    /// Represents a complex type variant.
    ComplexType(C),
}

impl TypeVariant<SimpleTypeVariant, ComplexTypeVariant> {
    /// Converts the variant into a [`ComplexTypeVariant`].
    #[must_use]
    pub fn into_complex_type_variant(self) -> ComplexTypeVariant {
        match self {
            TypeVariant::SimpleType(variant) => ComplexTypeVariant::SimpleType(variant),
            TypeVariant::ComplexType(variant) => variant,
        }
    }
}

impl<S: Deref<Target = SimpleTypeVariant>, C: Deref<Target = ComplexTypeVariant>>
    TypeVariant<S, C>
{
    /// Clones the variant in a generic way.
    pub fn cloned(&self) -> TypeVariant<SimpleTypeVariant, ComplexTypeVariant> {
        match self {
            TypeVariant::SimpleType(variant) => TypeVariant::SimpleType((**variant).clone()),
            TypeVariant::ComplexType(variant) => TypeVariant::ComplexType((**variant).clone()),
        }
    }
}

/// Actual data type a [`Type`] represents.
#[derive(Debug, Clone)]
pub enum SimpleTypeVariant {
    /// Represents a union type
    Union(UnionInfo),

    /// Represents a build-in type
    BuildIn(BuildInInfo),

    /// References an other type
    Reference(ReferenceInfo),

    /// Represents an enumeration
    Enumeration(EnumerationInfo),
}

/// Actual data type a [`Type`] represents.
#[derive(Debug, Clone)]
pub enum ComplexTypeVariant {
    /// References an other type
    Reference(ReferenceInfo),

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

    /// Represents a simple type
    SimpleType(SimpleTypeVariant),
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
    ($var:ident, $t:ty, $ty:ty) => {
        impl ::std::convert::From<$ty> for TypeDescriptor<$t> {
            fn from(value: $ty) -> Self {
                TypeDescriptor::new(<$t>::$var(value))
            }
        }
    };
}

macro_rules! impl_from_simple_type {
    ($var:ident, $t:ty, $ty:ty) => {
        impl ::std::convert::From<$ty> for Type {
            fn from(value: $ty) -> Self {
                Type::SimpleType(TypeDescriptor::<$t>::from(value))
            }
        }
    };
}

macro_rules! impl_from_complex_type {
    ($var:ident, $t:ty, $ty:ty) => {
        impl ::std::convert::From<$ty> for Type {
            fn from(value: $ty) -> Self {
                Type::ComplexType(TypeDescriptor::<$t>::from(value))
            }
        }
    };
}

impl_from!(Reference, SimpleTypeVariant, ReferenceInfo);
impl_from!(Reference, ComplexTypeVariant, ReferenceInfo);
impl_from!(BuildIn, SimpleTypeVariant, BuildInInfo);
impl_from!(Enumeration, SimpleTypeVariant, EnumerationInfo);
impl_from!(Dynamic, ComplexTypeVariant, DynamicInfo);
impl_from!(ComplexType, ComplexTypeVariant, ComplexInfo);
impl_from_simple_type!(BuildIn, SimpleTypeVariant, BuildInInfo);
impl_from_simple_type!(Enumeration, SimpleTypeVariant, EnumerationInfo);
impl_from_complex_type!(Dynamic, ComplexTypeVariant, DynamicInfo);
impl_from_complex_type!(ComplexType, ComplexTypeVariant, ComplexInfo);

impl From<SimpleType> for Type {
    fn from(value: SimpleType) -> Self {
        Type::SimpleType(value)
    }
}

impl From<ComplexType> for Type {
    fn from(value: ComplexType) -> Self {
        Type::ComplexType(value)
    }
}

impl<T> TypeDescriptor<T> {
    /// Create a new [`Type`] instance from the passed `variant`.
    #[must_use]
    pub fn new(variant: T) -> Self {
        Self {
            variant,
            display_name: None,
        }
    }
}

impl<T> Deref for TypeDescriptor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.variant
    }
}

impl<T> DerefMut for TypeDescriptor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variant
    }
}

impl TypeEq for SimpleType {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        #[allow(clippy::enum_glob_use)]
        use SimpleTypeVariant::*;

        self.display_name.hash(hasher);

        match &self.variant {
            Union(x) => x.type_hash(hasher, types),
            BuildIn(x) => x.hash(hasher),
            Reference(x) => x.type_hash(hasher, types),
            Enumeration(x) => x.type_hash(hasher, types),
        }
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        if self.display_name != other.display_name {
            return false;
        }

        self.variant.type_eq(&other.variant, types)
    }
}

impl TypeEq for SimpleTypeVariant {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        #[allow(clippy::enum_glob_use)]
        use SimpleTypeVariant::*;

        match &self {
            Union(x) => x.type_hash(hasher, types),
            BuildIn(x) => x.hash(hasher),
            Reference(x) => x.type_hash(hasher, types),
            Enumeration(x) => x.type_hash(hasher, types),
        }
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        #[allow(clippy::enum_glob_use)]
        use SimpleTypeVariant::*;

        match (&self, &other) {
            (Union(x), Union(y)) => x.type_eq(y, types),
            (BuildIn(x), BuildIn(y)) => x == y,
            (Reference(x), Reference(y)) => x.type_eq(y, types),
            (Enumeration(x), Enumeration(y)) => x.type_eq(y, types),
            (_, _) => false,
        }
    }
}

impl TypeEq for ComplexType {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        self.display_name.hash(hasher);

        self.variant.type_hash(hasher, types);
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        if self.display_name != other.display_name {
            return false;
        }

        self.variant.type_eq(&other.variant, types)
    }
}

impl TypeEq for ComplexTypeVariant {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        #[allow(clippy::enum_glob_use)]
        use ComplexTypeVariant::*;

        match &self {
            Reference(x) => x.type_hash(hasher, types),
            Dynamic(x) => x.type_hash(hasher, types),
            All(x) => x.type_hash(hasher, types),
            Choice(x) => x.type_hash(hasher, types),
            Sequence(x) => x.type_hash(hasher, types),
            ComplexType(x) => x.type_hash(hasher, types),
            SimpleType(x) => x.type_hash(hasher, types),
        }
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        #[allow(clippy::enum_glob_use)]
        use ComplexTypeVariant::*;

        match (&self, &other) {
            (Reference(x), Reference(y)) => x.type_eq(y, types),
            (Dynamic(x), Dynamic(y)) => x.type_eq(y, types),
            (All(x), All(y)) => x.type_eq(y, types),
            (Choice(x), Choice(y)) => x.type_eq(y, types),
            (Sequence(x), Sequence(y)) => x.type_eq(y, types),
            (ComplexType(x), ComplexType(y)) => x.type_eq(y, types),
            (_, _) => false,
        }
    }
}

impl TypeEq for Type {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        match self {
            Type::SimpleType(type_descriptor) => type_descriptor.type_hash(hasher, types),
            Type::ComplexType(type_descriptor) => type_descriptor.type_hash(hasher, types),
        }
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        match (self, other) {
            (Type::SimpleType(x), Type::SimpleType(y)) => x.type_eq(y, types),
            (Type::ComplexType(x), Type::ComplexType(y)) => x.type_eq(y, types),
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
