//! Contains the [`Name`] helper type and all related types.

use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Type that represents a name of a XSD element.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Name {
    /// The name was explicitly set to the given value.
    Named(Cow<'static, str>),

    /// The name was generated.
    Generated(Cow<'static, str>),
}

#[allow(missing_docs)]
impl Name {
    pub const ANY: Self = Self::named("any");
    pub const ANY_TYPE: Self = Self::named("anyType");
    pub const ANY_SIMPLE_TYPE: Self = Self::named("anySimpleType");

    pub const TYPE: Self = Self::named("type");
    pub const ANY_ATTRIBUTE: Self = Self::named("any_attribute");
}

impl Name {
    /// Create a new [`Name::Named`] using the passed `name`.
    #[must_use]
    pub const fn named(name: &'static str) -> Self {
        Self::Named(Cow::Borrowed(name))
    }

    /// Create a new [`Name::Generated`] using the passed `name`.
    #[must_use]
    pub const fn generated(name: &'static str) -> Self {
        Self::Generated(Cow::Borrowed(name))
    }

    /// Create a new [`Name::Named`] using the passed `name`.
    #[must_use]
    pub fn new_named<T>(name: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::Named(name.into())
    }

    /// Create a new [`Name::Generated`] using the passed `name`.
    #[must_use]
    pub fn new_generated<T>(name: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::Generated(name.into())
    }

    /// Returns `true` if this is a [`Name::Named`], `false` otherwise.
    #[must_use]
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    /// Returns `true` if this is a [`Name::Generated`], `false` otherwise.
    #[must_use]
    pub fn is_generated(&self) -> bool {
        matches!(self, Self::Generated(_))
    }

    /// Returns the value of [`Name::Named`] or [`Name::Generated`].
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Named(s) => s,
            Self::Generated(s) => s,
        }
    }

    /// Returns the value of [`Name::Named`] or `None`.
    #[must_use]
    pub fn as_named_str(&self) -> Option<&str> {
        match self {
            Self::Named(s) => Some(s),
            Self::Generated(_) => None,
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsMut<String> for Name {
    fn as_mut(&mut self) -> &mut String {
        let x = match self {
            Self::Named(x) => {
                *x = Cow::Owned((**x).to_owned());

                x
            }
            Self::Generated(x) => {
                *x = Cow::Owned((**x).to_owned());

                x
            }
        };

        let Cow::Owned(x) = x else {
            unreachable!();
        };

        x
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Named(x) => write!(f, "{x}"),
            Self::Generated(x) => write!(f, "{x}"),
        }
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self::Named(Cow::Owned(value))
    }
}

impl From<&'static str> for Name {
    fn from(value: &'static str) -> Self {
        Self::Named(Cow::Borrowed(value))
    }
}

impl From<Name> for String {
    fn from(value: Name) -> Self {
        value.as_str().to_owned()
    }
}

impl From<Name> for Cow<'static, str> {
    fn from(value: Name) -> Self {
        match value {
            Name::Named(s) => s,
            Name::Generated(s) => s,
        }
    }
}
