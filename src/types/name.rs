//! Contains the [`Name`] helper type and all related types.

use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::mem::take;

use inflector::Inflector;

/// Type that represents a name of a XSD element
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Name {
    /// The name was explicitly set to the given value.
    Named(Cow<'static, str>),

    /// The name is unknown and should be generated out of the provided information.
    Unnamed {
        /// Unique id for this name.
        id: usize,

        /// Extension that may be added to the name.
        ext: Option<Cow<'static, str>>,
    },
}

impl Name {
    /// Create a new [`Name::Named`] using the passed `name`.
    #[must_use]
    pub const fn named(name: &'static str) -> Self {
        Self::Named(Cow::Borrowed(name))
    }

    /// Create a new [`Name::Named`] using the passed `name`.
    #[must_use]
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self::Named(Cow::Owned(name.into()))
    }

    /// Remove the provided `suffix` from the [`Name::Named`] or the [`Name::Unnamed::ext`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// # use xsd_parser::types::Name;
    /// let name = Name::new("test-fuu");
    /// let expected = Name::new("test");
    /// assert_eq!(name.remove_suffix("-fuu"), expected);
    ///
    /// let name = Name::Unnamed { id: 123, ext: Some(Cow::Borrowed("test-fuu")) };
    /// let expected = Name::Unnamed { id: 123, ext: Some(Cow::Owned("test".into())) };;
    /// assert_eq!(name.remove_suffix("-fuu"), expected);
    /// ```
    #[must_use]
    pub fn remove_suffix(&self, suffix: &str) -> Self {
        match self {
            Self::Named(s) => {
                if let Some(s) = s.strip_suffix(suffix) {
                    Self::new(s)
                } else {
                    Self::Named(s.clone())
                }
            }
            Self::Unnamed { id, ext: Some(ext) } => {
                if let Some(ext) = ext.strip_suffix(suffix) {
                    Self::Unnamed {
                        id: *id,
                        ext: Some(Cow::Owned(ext.to_owned())),
                    }
                } else {
                    Self::Unnamed {
                        id: *id,
                        ext: Some(ext.clone()),
                    }
                }
            }
            x => x.clone(),
        }
    }

    /// Create a type name from this [`Name`] object.
    ///
    /// This method can be used to generate a rust type name from this name object.
    /// The resulting name is written in pascal case and may or may not contain
    /// additional information depending on the passed arguments.
    ///
    /// # Arguments
    /// - `with_id` Wether to add the unique id of the name to the resulting name or not
    /// - `name` Optional name that should be added to the resulting name
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// # use xsd_parser::types::Name;
    ///
    /// let name = Name::new("test");
    /// assert_eq!(Name::new("Test"), name.to_type_name(false, None));
    /// assert_eq!(Name::new("Test"), name.to_type_name(true, None));
    /// assert_eq!(Name::new("Test"), name.to_type_name(false, Some("extra")));
    /// assert_eq!(Name::new("Test"), name.to_type_name(true, Some("extra")));
    ///
    /// let name = Name::Unnamed { id: 123, ext: None };
    /// assert_eq!(Name::new("Unnamed123"), name.to_type_name(false, None));
    /// assert_eq!(Name::new("Extra"), name.to_type_name(false, Some("extra")));
    /// assert_eq!(Name::new("Unnamed123"), name.to_type_name(true, None));
    /// assert_eq!(Name::new("Extra123"), name.to_type_name(true, Some("extra")));
    ///
    /// let name = Name::Unnamed { id: 123, ext: Some(Cow::Borrowed("ext")) };
    /// assert_eq!(Name::new("ExtUnnamed123"), name.to_type_name(false, None));
    /// assert_eq!(Name::new("ExtExtra"), name.to_type_name(false, Some("extra")));
    /// assert_eq!(Name::new("ExtUnnamed123"), name.to_type_name(true, None));
    /// assert_eq!(Name::new("ExtExtra123"), name.to_type_name(true, Some("extra")));
    /// ```
    #[must_use]
    pub fn to_type_name(&self, with_id: bool, name: Option<&str>) -> Self {
        match (self, name) {
            (Self::Named(s), _) => Self::Named(Cow::Owned(s.to_pascal_case())),
            (Self::Unnamed { id, ext: Some(ext) }, Some(name)) if with_id => {
                Self::Named(Cow::Owned(format!(
                    "{}{}{id}",
                    ext.to_pascal_case(),
                    name.to_pascal_case()
                )))
            }
            (Self::Unnamed { ext: Some(ext), .. }, Some(name)) => Self::Named(Cow::Owned(format!(
                "{}{}",
                ext.to_pascal_case(),
                name.to_pascal_case()
            ))),
            (Self::Unnamed { id, ext: None, .. }, Some(name)) if with_id => {
                Self::Named(Cow::Owned(format!("{}{id}", name.to_pascal_case())))
            }
            (Self::Unnamed { ext: None, .. }, Some(name)) => {
                Self::Named(Cow::Owned(name.to_pascal_case()))
            }
            (
                Self::Unnamed {
                    id, ext: Some(ext), ..
                },
                None,
            ) => Self::Named(Cow::Owned(format!("{}Unnamed{id}", ext.to_pascal_case()))),
            (Self::Unnamed { id, ext: None, .. }, None) => {
                Self::Named(Cow::Owned(format!("Unnamed{id}")))
            }
        }
    }

    /// Returns `true` if this is a [`Name::Named`], `false` otherwise.
    #[must_use]
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named(_))
    }

    /// Returns `true` if this is a [`Name::Unnamed`], `false` otherwise.
    #[must_use]
    pub fn is_unnamed(&self) -> bool {
        matches!(self, Self::Unnamed { .. })
    }

    /// Returns `true` if this is a [`Name::Unnamed`] with extensions, `false` otherwise.
    #[must_use]
    pub fn has_extension(&self) -> bool {
        matches!(self, Self::Unnamed { ext: Some(_), .. })
    }

    /// Returns the value of [`Name::Named`] as `Some(&str)`, or `None` if it's an [`Name::Unnamed`].
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Named(s) => Some(s),
            Self::Unnamed { .. } => None,
        }
    }

    /// Adds extensions to this name.
    ///
    /// # Arguments
    /// - `replace` replace any existing extension with the new one
    /// - `iter` iterator of extensions to apply
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// # use xsd_parser::types::Name;
    ///
    /// let name = Name::new("test");
    /// assert_eq!(Name::new("extTest"), name.extend(false, Some("ext")));
    ///
    /// let name = Name::Unnamed { id: 123, ext: Some(Cow::Borrowed("ext")) };
    /// assert_eq!(Name::Unnamed { id: 123, ext: Some(Cow::Owned("fuu".into())) }, name.clone().extend(true, Some("fuu")));
    /// assert_eq!(Name::Unnamed { id: 123, ext: Some(Cow::Owned("fuuExt".into())) }, name.extend(false, Some("fuu")));
    /// ``````
    #[must_use]
    pub fn extend<I>(mut self, mut replace: bool, iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Display,
    {
        for s in iter {
            match &mut self {
                Self::Named(name) => *name = Cow::Owned(format!("{s}{}", name.to_pascal_case())),
                Self::Unnamed { ext: Some(ext), .. } if !take(&mut replace) => {
                    *ext = Cow::Owned(format!("{s}{}", ext.to_pascal_case()));
                }
                Self::Unnamed { ext, .. } => *ext = Some(Cow::Owned(s.to_string())),
            }
        }

        self
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Named(x) => write!(f, "{x}"),
            Self::Unnamed { id, ext: None } => write!(f, "Unnamed{id}"),
            Self::Unnamed { id, ext: Some(ext) } => write!(f, "{ext}Unnamed{id}"),
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
