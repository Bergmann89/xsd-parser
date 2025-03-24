use std::borrow::Cow;

use inflector::Inflector;
use proc_macro2::Ident as Ident2;
use quote::format_ident;
use thiserror::Error;

use crate::schema::NamespaceId;
use crate::types::{Ident, Name, Type, TypeVariant, Types};

/// Error that is raised by [`format_module`].
#[derive(Debug, Error)]
#[error("Unknown namespace: {0:?}")]
pub struct UnknownNamespace(pub NamespaceId);

/// Formats the passed `name` as field name.
///
/// Will format the passed `name` as field name if no `display_name` was passed as well.
/// If a `display_name` was passed, this one will be formatted as field name instead.
pub fn format_field_name(name: &Name, display_name: Option<&str>) -> Cow<'static, str> {
    if let Some(display_name) = display_name {
        return Cow::Owned(display_name.to_snake_case());
    }

    let ident = name
        .to_type_name(false, None)
        .as_str()
        .unwrap()
        .to_snake_case();

    match KEYWORDS.binary_search_by(|(key, _)| key.cmp(&ident.as_str())) {
        Ok(idx) => Cow::Borrowed(KEYWORDS[idx].1),
        Err(_) => {
            if ident.starts_with(char::is_numeric) {
                Cow::Owned(format!("_{ident}"))
            } else {
                Cow::Owned(ident)
            }
        }
    }
}

/// Returns the result of [`format_field_name`] as [`Ident2`] instead of a string.
#[must_use]
pub fn format_field_ident(name: &Name, display_name: Option<&str>) -> Ident2 {
    let ident = format_field_name(name, display_name);

    format_ident!("{ident}")
}

/// Formats the passed `name` as module name.
#[must_use]
pub fn format_module_ident(name: &Name) -> Ident2 {
    format_field_ident(name, None)
}

/// Formats the passed `name` as type name.
///
/// Will format the passed `name` as type name if no `display_name` was passed as well.
/// If a `display_name` was passed, this one will be formatted as type name instead.
#[must_use]
pub fn format_type_name(name: &Name, display_name: Option<&str>) -> String {
    if let Some(display_name) = display_name {
        return display_name.to_pascal_case();
    }

    let name = name
        .to_type_name(false, None)
        .as_str()
        .unwrap()
        .to_pascal_case();

    if name.starts_with(char::is_numeric) {
        format!("_{name}")
    } else {
        name
    }
}

/// Returns the result of [`format_type_ident`] as [`Ident2`] instead of a string.
#[must_use]
pub fn format_type_ident(name: &Name, display_name: Option<&str>) -> Ident2 {
    let ident = format_type_name(name, display_name);

    format_ident!("{ident}")
}

/// Formats the passed `name` as variant name.
///
/// Will format the passed `name` as variant name if no `display_name` was passed as well.
/// If a `display_name` was passed, this one will be formatted as variant name instead.
#[must_use]
pub fn format_variant_ident(name: &Name, display_name: Option<&str>) -> Ident2 {
    format_type_ident(name, display_name)
}

/// Returns name of the module that is identified by the passed namespace id `ns`.
///
/// If the module does not have a name, `None` is returned.
///
/// # Errors
///
/// Returns [`UnknownNamespace`] if the passed namespace is not known.
pub fn format_module(
    types: &Types,
    ns: Option<NamespaceId>,
) -> Result<Option<Ident2>, UnknownNamespace> {
    let Some(ns) = ns else {
        return Ok(None);
    };

    let module = types.modules.get(&ns).ok_or(UnknownNamespace(ns))?;
    let Some(name) = &module.name else {
        return Ok(None);
    };

    Ok(Some(format_module_ident(name)))
}

/// Generates a type name using the provided `postfixes` for the identifier
/// `ident` of the given type `ty`.
#[must_use]
pub fn make_type_name(postfixes: &[String], ty: &Type, ident: &Ident) -> Name {
    match (&ty.variant, &ident.name) {
        (TypeVariant::Reference(ti), Name::Unnamed { .. }) if ti.type_.name.is_named() => {
            if ti.min_occurs > 1 {
                return Name::new(format!("{}List", ti.type_.name));
            } else if ti.min_occurs == 0 {
                return Name::new(format!("{}Opt", ti.type_.name));
            }
        }
        (_, _) => (),
    };

    let postfix = postfixes
        .get(ident.type_ as usize)
        .map_or("", |s| s.as_str());

    match &ident.name {
        Name::Named(s) if s.ends_with(postfix) => Name::Named(s.clone()),
        Name::Named(s) => Name::Named(Cow::Owned(format!("{}{postfix}", Name::unify(s)))),
        name => name.to_type_name(false, None),
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
