//! Contains the [`Name`] helper type and all related types.

use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};

use inflector::Inflector;

/// Type that represents a name of a XSD element.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Name {
    /// The name was explicitly set to the given value.
    Named(Cow<'static, str>),

    /// The name was generated.
    Generated(Cow<'static, str>),
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

    /// Formats this name as type name.
    #[must_use]
    pub fn to_type_name(&self) -> String {
        Self::format_type_name(self.as_str())
    }

    /// Formats this name as field name.
    #[must_use]
    pub fn to_field_name(&self) -> String {
        Self::format_field_name(self.as_str())
    }

    /// Unifies the passed string `s`.
    #[must_use]
    pub fn unify(s: &str) -> String {
        let mut done = true;
        let s = s.replace(
            |c: char| {
                let replace = !c.is_alphanumeric();
                if c != '_' && !replace {
                    done = false;
                }

                c != '_' && replace
            },
            "_",
        );

        if done {
            s
        } else {
            s.to_screaming_snake_case().to_pascal_case()
        }
    }

    /// Formats the passed string `s` as type name.
    #[must_use]
    pub fn format_type_name(s: &str) -> String {
        let name = Name::unify(s);

        if name.starts_with(char::is_numeric) {
            format!("_{name}")
        } else {
            name
        }
    }

    /// Formats the passed string `s` as field name.
    #[must_use]
    pub fn format_field_name(s: &str) -> String {
        let mut name = Name::unify(s).to_snake_case();

        if let Ok(idx) = KEYWORDS.binary_search_by(|(key, _)| key.cmp(&&*name)) {
            name = KEYWORDS[idx].1.into();
        }

        if name.starts_with(char::is_numeric) {
            name = format!("_{name}");
        }

        name
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

const KEYWORDS: &[(&str, &str)] = &[
    ("abstract", "abstract_"),
    ("as", "as_"),
    ("become", "become_"),
    ("box", "box_"),
    ("break", "break_"),
    ("const", "const_"),
    ("continue", "continue_"),
    ("crate", "crate_"),
    ("do", "do_"),
    ("else", "else_"),
    ("enum", "enum_"),
    ("extern", "extern_"),
    ("false", "false_"),
    ("final", "final_"),
    ("fn", "fn_"),
    ("for", "for_"),
    ("if", "if_"),
    ("impl", "impl_"),
    ("in", "in_"),
    ("let", "let_"),
    ("loop", "loop_"),
    ("macro", "macro_"),
    ("match", "match_"),
    ("mod", "mod_"),
    ("move", "move_"),
    ("mut", "mut_"),
    ("override", "override_"),
    ("priv", "priv_"),
    ("pub", "pub_"),
    ("ref", "ref_"),
    ("return", "return_"),
    ("self", "self_"),
    ("Self", "Self_"),
    ("static", "static_"),
    ("struct", "struct_"),
    ("super", "super_"),
    ("trait", "trait_"),
    ("true", "true_"),
    ("try", "try_"),
    ("type", "type_"),
    ("typeof", "typeof_"),
    ("union", "union_"),
    ("unsafe", "unsafe_"),
    ("unsized", "unsized_"),
    ("use", "use_"),
    ("virtual", "virtual_"),
    ("where", "where_"),
    ("while", "while_"),
    ("yield", "yield_"),
];

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    fn unify() {
        assert_eq!("_", Name::unify("+"));
        assert_eq!("FuuBarBaz", Name::unify("Fuu_BAR_BAZ"));
        assert_eq!("FuuBarBaz", Name::unify("fuu_bar_baz"));
        assert_eq!("FuuBarBaz", Name::unify("fuu+Bar-BAZ"));
    }
}
