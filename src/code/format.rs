use proc_macro2::Ident as Ident2;
use quote::format_ident;

use crate::types::Name;

/// Formats the passed `name` as module name.
#[must_use]
pub fn format_module_ident(name: &Name) -> Ident2 {
    format_field_ident(name, None)
}

/// Formats the passed `name` as field name.
///
/// Will format the passed `name` as field name if no `display_name` was passed as well.
/// If a `display_name` was passed, this one will be formatted as field name instead.
#[must_use]
pub fn format_field_ident(name: &Name, display_name: Option<&str>) -> Ident2 {
    let ident = Name::format_field_name(display_name.unwrap_or(name.as_str()));

    format_ident!("{ident}")
}

/// Formats the passed `name` as type name.
///
/// Will format the passed `name` as type name if no `display_name` was passed as well.
/// If a `display_name` was passed, this one will be formatted as type name instead.
#[must_use]
pub fn format_type_ident(name: &Name, display_name: Option<&str>) -> Ident2 {
    let ident = Name::format_type_name(display_name.unwrap_or(name.as_str()));

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
