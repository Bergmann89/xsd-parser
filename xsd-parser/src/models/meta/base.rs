//! Contains the [`Base`] type information and all related types.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::Hasher;

use crate::models::TypeIdent;

use super::{MetaTypes, TypeEq};

/// Describes the base type information of a specific type information.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum Base {
    /// The type information has no base type.
    #[default]
    None,

    /// The type information extends the provided base type.
    Extension(TypeIdent),

    /// The type information restricts the provided base type.
    Restriction(TypeIdent),
}

impl Base {
    /// Get the identifier of the base type if it is available.
    #[must_use]
    pub fn as_ident(&self) -> Option<&TypeIdent> {
        match self {
            Self::None => None,
            Self::Extension(x) => Some(x),
            Self::Restriction(x) => Some(x),
        }
    }

    /// Extracts the identifier of the base type if it is available.
    #[must_use]
    pub fn into_ident(self) -> Option<TypeIdent> {
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
    fn type_hash<H: Hasher>(&self, hasher: &mut H, types: &MetaTypes) {
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

    fn type_eq(&self, other: &Self, types: &MetaTypes) -> bool {
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
