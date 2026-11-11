mod default;
mod explicit;
mod legacy;

use std::borrow::Cow;
use std::mem::take;

use proc_macro2::Ident as Ident2;
use quote::format_ident;

use crate::config::MetaType;
use crate::models::meta::MetaTypeVariant;
use crate::models::schema::MaxOccurs;
use crate::TypeIdent;

use super::Name;

pub use self::default::{NameBuilder, Naming};
pub use self::explicit::{ExplicitNameBuilder, ExplicitNaming};
pub use self::legacy::{LegacyExplicitNaming, LegacyNaming};

/// Make a type name based on the passed `ident` and `ty` and the provided `postfixes`.
pub fn make_type_name<X>(naming: &X, postfixes: &[String], ty: &MetaType, ident: &TypeIdent) -> Name
where
    X: crate::traits::Naming,
{
    if let MetaTypeVariant::Reference(ti) = &ty.variant {
        if ident.name.is_generated() && ti.type_.name.is_named() {
            let s = naming.format_type_name(ti.type_.name.as_str());

            if ti.max_occurs > MaxOccurs::Bounded(1) {
                return Name::new_generated(format!("{s}List"));
            } else if ti.min_occurs == 0 {
                return Name::new_generated(format!("{s}Opt"));
            }
        }
    }

    let postfix = postfixes
        .get(ident.type_ as usize)
        .map_or("", |s| s.as_str());

    let s = naming.format_type_name(ident.name.as_str());

    if s.ends_with(postfix) {
        ident.name.clone()
    } else {
        Name::new_generated(format!("{s}{postfix}"))
    }
}

/// Split the passed string `s` into words.
///
/// Non-alphanumeric characters separate words, and so do case changes:
/// `fooBar` and `HTTPServer` give two words each. Digits stay with the word in
/// front of them (`MP3`, `Base64Type`), a plural `s` stays with its acronym
/// (`IDs`, `MP3s`), and a single lowercase letter followed by a digit starts a
/// new word after an acronym (`IPv4` gives `IP` and `v4`).
#[must_use]
pub fn split_words(s: &str) -> Vec<String> {
    let c = s.chars().collect::<Vec<_>>();
    let mut words = Vec::new();
    let mut word = String::new();

    for (i, &ch) in c.iter().enumerate() {
        if !ch.is_alphanumeric() {
            if !word.is_empty() {
                words.push(take(&mut word));
            }

            continue;
        }

        let Some(prev) = word.chars().last() else {
            word.push(ch);

            continue;
        };

        let next = c.get(i + 1).copied();
        let after = c.get(i + 2).copied();

        let is_boundary = if ch.is_uppercase() {
            if prev.is_lowercase() {
                true
            } else if prev.is_numeric() {
                // `X509SKI` and `Content73XType` split after the digits, `MP3S` does not.
                let n = c[i..].iter().take_while(|c| c.is_uppercase()).count();
                let follow = c.get(i + n).copied();

                n > 1 || follow.is_some_and(|c| c.is_lowercase() || c.is_ascii_digit())
            } else if next.is_some_and(char::is_lowercase) {
                // `HTTPServer` splits before `Server`, `IDs` and `IPv4` keep the acronym.
                let is_plural = next == Some('s') && after.is_none_or(|c| !c.is_lowercase());
                let is_version = after.is_some_and(|c| c.is_ascii_digit());

                !is_plural && !is_version
            } else {
                false
            }
        } else if ch.is_lowercase() && prev.is_numeric() {
            // `iso3166alpha2` splits before `alpha`, `11th` and `MP3s` do not.
            c[i..].iter().take_while(|c| c.is_lowercase()).count() > 2
        } else if ch.is_lowercase() {
            // `IPv4` splits before `v4`.
            ch != 's'
                && prev.is_uppercase()
                && word.chars().count() >= 2
                && word.chars().all(|c| c.is_uppercase() || c.is_ascii_digit())
                && next.is_some_and(|c| c.is_ascii_digit())
        } else {
            false
        };

        if is_boundary {
            words.push(take(&mut word));
        }

        word.push(ch);
    }

    if !word.is_empty() {
        words.push(word);
    }

    words
}

/// Unify the passed string `s` into a standard format.
///
/// The result is `PascalCase`, with a `_` between two words where [`split_words`]
/// would not find the boundary otherwise (`Q_Name`, `Dominant_11th`).
#[must_use]
pub fn unify_string(s: &str) -> String {
    if !s.chars().any(char::is_alphanumeric) {
        return s.replace(|c: char| !c.is_alphanumeric(), "_");
    }

    let mut ret = String::with_capacity(s.len());
    for word in split_words(s) {
        let word = capitalize(&word);

        // `split_words` always splits a lowercase letter from a following uppercase one,
        // every other boundary is kept by a `_`.
        let is_split = ret.ends_with(char::is_lowercase) && word.starts_with(char::is_uppercase);
        if !ret.is_empty() && !is_split {
            ret.push('_');
        }

        ret.push_str(&word);
    }

    ret
}

/// Format the passed unified string `s` as `PascalCase` type name.
#[must_use]
pub fn to_type_case(s: &str) -> String {
    if !s.chars().any(char::is_alphanumeric) {
        return s.to_owned();
    }

    let mut ret = String::with_capacity(s.len());
    for part in s.split('_') {
        // Keep the `_` between two digits (`Gs1_128`).
        if ret.ends_with(|c: char| c.is_ascii_digit())
            && part.starts_with(|c: char| c.is_ascii_digit())
        {
            ret.push('_');
        }

        ret.push_str(part);
    }

    ret
}

/// Format the passed string `s` as `snake_case` field or module name.
#[must_use]
pub fn to_field_case(s: &str) -> String {
    split_words(s)
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

/// Format the passed string `s` as `SCREAMING_SNAKE_CASE` constant name.
#[must_use]
pub fn to_constant_case(s: &str) -> String {
    to_field_case(s).to_uppercase()
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    chars.next().map_or_else(String::new, |first| {
        first
            .to_uppercase()
            .chain(chars.flat_map(char::to_lowercase))
            .collect()
    })
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
    use crate::traits::Naming as _;

    use super::{split_words, unify_string, ExplicitNaming, Naming, KEYWORDS};

    #[test]
    fn split_words_keeps_acronyms_together() {
        for (s, words) in [
            ("fooBar", &["foo", "Bar"][..]),
            ("HTTPServer", &["HTTP", "Server"]),
            ("UserIDs", &["User", "IDs"]),
            ("URLsForIDs", &["URLs", "For", "IDs"]),
            ("MP3s", &["MP3s"]),
            ("IPv4s", &["IP", "v4s"]),
            ("X509SKI", &["X509", "SKI"]),
            ("Content73XType", &["Content73", "X", "Type"]),
            ("iso3166alpha2", &["iso3166", "alpha2"]),
            ("dominant-11th", &["dominant", "11th"]),
            ("fuu+Bar-BAZ", &["fuu", "Bar", "BAZ"]),
        ] {
            assert_eq!(split_words(s), words, "{s}");
        }
    }

    #[test]
    fn names_are_formatted_from_words() {
        let naming = Naming::default();

        for (s, type_name, field_name, constant_name) in [
            ("MP3s", "Mp3s", "mp3s", "MP3S"),
            ("IPV4s", "Ipv4s", "ipv4s", "IPV4S"),
            ("IPv4s", "IpV4s", "ip_v4s", "IP_V4S"),
            ("UserIDs", "UserIds", "user_ids", "USER_IDS"),
            ("IDStatus", "IdStatus", "id_status", "ID_STATUS"),
            ("X509SKI", "X509Ski", "x509_ski", "X509_SKI"),
            ("GS1_128", "Gs1_128", "gs1_128", "GS1_128"),
            (
                "dominant-11th",
                "Dominant11th",
                "dominant_11th",
                "DOMINANT_11TH",
            ),
            ("1.2", "_1_2", "_1_2", "_1_2"),
            ("aU", "AU", "a_u", "A_U"),
            ("QName", "QName", "q_name", "Q_NAME"),
            ("Content103", "Content103", "content103", "CONTENT103"),
        ] {
            assert_eq!(naming.format_type_name(s), type_name, "{s}");
            assert_eq!(naming.format_variant_name(s), type_name, "{s}");
            assert_eq!(naming.format_field_name(s), field_name, "{s}");
            assert_eq!(naming.format_module_name(s), field_name, "{s}");
            assert_eq!(naming.format_constant_name(s), constant_name, "{s}");
        }
    }

    #[test]
    fn unify_is_stable() {
        for s in [
            "aU",
            "dominant-11th",
            "GS1_128",
            "IPv4s",
            "UserIDs",
            "Content73XType",
        ] {
            let unified = unify_string(s);

            assert_eq!(unify_string(&unified), unified, "{s}");
        }
    }

    #[test]
    fn explicit_naming_without_unify_keeps_type_names() {
        let naming = ExplicitNaming::default().unify_names(false);

        assert_eq!(naming.format_type_name("foo_BAR"), "foo_BAR");
        assert_eq!(naming.format_field_name("UserIDs"), "user_ids");
    }

    #[test]
    fn verify_keyword_order() {
        for i in 1..KEYWORDS.len() {
            assert!(dbg!(KEYWORDS[i - 1].0) < dbg!(KEYWORDS[i].0));
        }
    }
}
