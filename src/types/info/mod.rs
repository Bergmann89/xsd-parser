//! Contains different type information types.

pub mod attribute;
pub mod complex;
pub mod custom;
pub mod dynamic;
pub mod element;
pub mod enumeration;
pub mod reference;
pub mod union;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::Hasher;

pub use attribute::{AttributeInfo, AttributesInfo};
pub use complex::{AnyAttributeInfo, AnyInfo, ComplexInfo, GroupInfo};
pub use custom::{CustomDefaultImpl, CustomInfo};
pub use dynamic::DynamicInfo;
pub use element::{ElementInfo, ElementMode, ElementsInfo};
pub use enumeration::{EnumerationInfo, VariantInfo};
pub use reference::ReferenceInfo;
pub use union::{UnionInfo, UnionTypeInfo, UnionTypesInfo};

use crate::schema::xs::Use;

use super::{Ident, TypeEq, Types};

/// Describes the base type information of a specific type information.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum Base {
    /// The type information has no base type.
    #[default]
    None,

    /// The type information extends the provided base type.
    Extension(Ident),

    /// The type information restricts the provided base type.
    Restriction(Ident),
}

impl Base {
    /// Get the identifier of the base type if it is available.
    #[must_use]
    pub fn as_ident(&self) -> Option<&Ident> {
        match self {
            Self::None => None,
            Self::Extension(x) => Some(x),
            Self::Restriction(x) => Some(x),
        }
    }

    /// Extracts the identifier of the base type if it is available.
    #[must_use]
    pub fn into_ident(self) -> Option<Ident> {
        match self {
            Self::None => None,
            Self::Extension(x) => Some(x),
            Self::Restriction(x) => Some(x),
        }
    }
}

impl Display for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::None => write!(f, "None"),
            Self::Extension(x) => write!(f, "Extension({x})"),
            Self::Restriction(x) => write!(f, "Restriction({x})"),
        }
    }
}

impl TypeEq for Base {
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &Types) {
        match self {
            Self::None => hasher.write_u8(0),
            Self::Extension(x) => {
                hasher.write_u8(1);
                x.type_hash(hasher, types);
            }
            Self::Restriction(x) => {
                hasher.write_u8(2);
                x.type_hash(hasher, types);
            }
        }
    }

    fn type_eq(&self, other: &Self, types: &Types) -> bool {
        #[allow(clippy::enum_glob_use)]
        use Base::*;

        match (self, other) {
            (None, None) => true,
            (Extension(x), Extension(y)) => x.type_eq(y, types),
            (Restriction(x), Restriction(y)) => x.type_eq(y, types),
            (_, _) => false,
        }
    }
}

fn use_hash<H: Hasher>(use_: &Use, hasher: &mut H) {
    match use_ {
        Use::Prohibited => hasher.write_u8(0),
        Use::Optional => hasher.write_u8(1),
        Use::Required => hasher.write_u8(2),
    }
}
