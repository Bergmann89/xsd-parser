mod default;
mod explicit;

use std::borrow::Cow;

use inflector::Inflector;
use proc_macro2::Ident as Ident2;
use quote::format_ident;

use super::Name;

pub use self::default::{NameBuilder, Naming};
pub use self::explicit::{ExplicitNameBuilder, ExplicitNaming};

/// Unify the passed string `s` into a standard format.
#[must_use]
pub fn unify_string(s: &str) -> String {
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

/// Format an unknown variant identifier based on the passed `id`.
#[must_use]
pub fn format_unknown_variant(id: usize) -> Ident2 {
    format_ident!("Unknown{id}")
}

/// Format the passed string `s` into a valid Rust identifier by adding an
/// underscore if it starts with a numeric character or is a Rust keyword.
#[must_use]
pub fn format_ident<'a, S>(s: S) -> String
where
    S: Into<Cow<'a, str>>,
{
    let mut s = s.into();
    if let Ok(idx) = KEYWORDS.binary_search_by(|(key, _)| key.cmp(&&*s)) {
        s = Cow::Borrowed(KEYWORDS[idx].1);
    }

    if s.starts_with(char::is_numeric) {
        s = Cow::Owned(format!("_{s}"));
    }

    s.replace(|c: char| !c.is_alphanumeric(), "_")
}

/// List of keywords that needs to be replaced by something else.
/// This list needs to be sorted, because we use it in [`core::slice::binary_search_by`]
const KEYWORDS: &[(&str, &str)] = &[
    ("Self", "Self_"),
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
    use super::KEYWORDS;

    #[test]
    fn verify_keyword_order() {
        for i in 1..KEYWORDS.len() {
            assert!(dbg!(KEYWORDS[i - 1].0) < dbg!(KEYWORDS[i].0));
        }
    }
}
