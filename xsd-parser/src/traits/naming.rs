use std::any::Any;
use std::fmt::{Debug, Display};
use std::mem::take;

use inflector::Inflector;
use proc_macro2::Ident as Ident2;
use quote::format_ident;

use crate::models::{
    meta::{MetaType, MetaTypeVariant, MetaTypes},
    schema::{MaxOccurs, NamespaceId},
    Ident, Name,
};

/// This trait defined how names are generated and formatted in `xsd-parser`.
///
/// Use the [`Interpreter::with_naming`](crate::Interpreter::with_naming) method
/// to use a customized implementation for this trait.
pub trait Naming: Debug {
    /// Clone this object into a new `Box`.
    fn clone_boxed(&self) -> Box<dyn Naming>;

    /// Create a new name builder instance.
    fn builder(&self) -> Box<dyn NameBuilder>;

    /// Unify the given string.
    ///
    /// Before actual name generation or formatting it is sometimes useful
    /// to have a pre-formatting for names, to have a unified schema for the
    /// names in general.
    ///
    /// The default implementation uses pascal case to unify all different kind
    /// of names.
    fn unify(&self, s: &str) -> String {
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

    /// Format the passed string `s` as type name.
    ///
    /// The default implementation uses pascal case here.
    fn format_type_name(&self, s: &str) -> String {
        let mut name = self.unify(s);

        if let Ok(idx) = KEYWORDS.binary_search_by(|(key, _)| key.cmp(&&*name)) {
            name = KEYWORDS[idx].1.into();
        }

        if name.starts_with(char::is_numeric) {
            name = format!("_{name}");
        }

        name
    }

    /// Format the passed string `s` as field name.
    ///
    /// The default implementation uses snake case here.
    fn format_field_name(&self, s: &str) -> String {
        let mut name = self.unify(s).to_snake_case();

        if let Ok(idx) = KEYWORDS.binary_search_by(|(key, _)| key.cmp(&&*name)) {
            name = KEYWORDS[idx].1.into();
        }

        if name.starts_with(char::is_numeric) {
            name = format!("_{name}");
        }

        name
    }

    /// Format the passed string `s` as variant name.
    ///
    /// The default implementation uses [`format_type_name`](Naming::format_type_name) here.
    fn format_variant_name(&self, s: &str) -> String {
        self.format_type_name(s)
    }

    /// Format the passed string `s` as module name.
    ///
    /// The default implementation uses [`format_field_name`](Naming::format_field_name) here.
    fn format_module_name(&self, s: &str) -> String {
        self.format_field_name(s)
    }

    /// Create a suitable identifier for the passed type name `name` respecting
    /// user defined names stored in `display_name`.
    ///
    /// The default implementation uses the `display_name` or `name` as fallback
    /// and formats it using [`format_type_name`](Naming::format_type_name).
    fn format_type_ident(&self, name: &Name, display_name: Option<&str>) -> Ident2 {
        let ident = self.format_type_name(display_name.unwrap_or(name.as_str()));

        format_ident!("{ident}")
    }

    /// Create a suitable identifier for the passed field name `name` respecting
    /// user defined names stored in `display_name`.
    ///
    /// The default implementation uses the `display_name` or `name` as fallback
    /// and formats it using [`format_field_name`](Naming::format_field_name).
    fn format_field_ident(&self, name: &Name, display_name: Option<&str>) -> Ident2 {
        let ident = self.format_field_name(display_name.unwrap_or(name.as_str()));

        format_ident!("{ident}")
    }

    /// Create a suitable identifier for the passed variant name `name` respecting
    /// user defined names stored in `display_name`.
    ///
    /// The default implementation uses [`format_type_ident`](Naming::format_type_ident) here.
    fn format_variant_ident(&self, name: &Name, display_name: Option<&str>) -> Ident2 {
        self.format_type_ident(name, display_name)
    }

    /// Create a suitable identifier for the passed module name `name`.
    ///
    /// The default implementation uses [`format_module_ident`](Naming::format_module_ident)
    /// with `display_name` set to `None` here.
    fn format_module_ident(&self, name: &Name) -> Ident2 {
        self.format_field_ident(name, None)
    }

    /// Generate a identifier for the module identified by `ns`.
    fn format_module(&self, types: &MetaTypes, ns: Option<NamespaceId>) -> Option<Ident2> {
        let ns = ns?;
        let module = types.modules.get(&ns)?;
        let name = module.name.as_ref()?;

        Some(self.format_module_ident(name))
    }

    /// Generate a name for the passed type `ty` identified by `ident` respecting the
    /// configured type postfixes.
    fn make_type_name(&self, postfixes: &[String], ty: &MetaType, ident: &Ident) -> Name {
        if let MetaTypeVariant::Reference(ti) = &ty.variant {
            if ident.name.is_generated() && ti.type_.name.is_named() {
                let s = self.format_type_name(ti.type_.name.as_str());

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

        let s = self.format_type_name(ident.name.as_str());

        if s.ends_with(postfix) {
            ident.name.clone()
        } else {
            Name::new_generated(format!("{s}{postfix}"))
        }
    }

    /// Create an unknown enum variant using the provided id.
    ///
    /// The default implementation generated `Unknown{id}` here.
    fn make_unknown_variant(&self, id: usize) -> Ident2 {
        format_ident!("Unknown{id}")
    }
}

/// This trait defines a builder for building type names.
///
/// The general idea of the builder is the following:
/// - A type name needs a name to be valid. The name is set by
///   [`set_name`](NameBuilder::set_name).
/// - The name can be extended by multiple prefixes using
///   [`add_extension`](NameBuilder::add_extension).
/// - Sometimes is it not possible to create a unique name. To get one
///   the builder should add a unique id to the name (this is controlled by
///   [`set_with_id`](NameBuilder::set_with_id)).
/// - The output of the name builder is a [`Name`]. `Name`s can be fixed or
///   generated. The name builder should decide automatically which variant of
///   the name has to be generated, if not explicitly specified by the user
///   (using [`set_generated`](NameBuilder::set_generated)).
pub trait NameBuilder: Debug + Any {
    /// Finish the current name building and create a [`Name`] from the known
    /// information.
    fn finish(&self) -> Name;

    /// Merge the data of the `other` name builder into the current name builder.
    /// The passed name builder is of the same type then the current one.
    fn merge(&mut self, other: &dyn NameBuilder);

    /// Create a clone of the current builder and return it as box.
    fn clone_boxed(&self) -> Box<dyn NameBuilder>;

    /// Returns `true` if this builder has a name set, `false` otherwise.
    fn has_name(&self) -> bool;

    /// Returns `true` if this builder has at least on extension set, `false` otherwise.
    fn has_extension(&self) -> bool;

    /// Set the base name of this builder.
    fn set_name(&mut self, name: String);

    /// Instruct the builder to add a unique id to the generated name or not.
    fn set_with_id(&mut self, value: bool);

    /// Instruct the builder to generated a [`Name::Generated`] if `true` is passed,
    /// or a [`Name::Named`] if `false` is passed.
    fn set_generated(&mut self, value: bool);

    /// Add a new `extension` to the builder. If `replace` is set to true, any previous
    /// extension is dropped.
    fn add_extension(&mut self, replace: bool, extension: String);

    /// Remove the specified `suffix` from the name and the extensions.
    fn strip_suffix(&mut self, suffix: &str);

    /// Force the builder to generate a unique id, that is later used to generate
    /// the name.
    ///
    /// Normally the id should be generated as last step (in the
    /// [`finish`](NameBuilder::finish) method), but sometimes it is useful to
    /// force the builder to generate the id to shared it between different copies
    /// of the builder.
    fn generate_unique_id(&mut self);
}

impl NameBuilder for Box<dyn NameBuilder> {
    #[inline]
    fn finish(&self) -> Name {
        (**self).finish()
    }

    #[inline]
    fn merge(&mut self, other: &dyn NameBuilder) {
        (**self).merge(other);
    }

    #[inline]
    fn clone_boxed(&self) -> Box<dyn NameBuilder> {
        (**self).clone_boxed()
    }

    #[inline]
    fn has_name(&self) -> bool {
        (**self).has_name()
    }

    #[inline]
    fn has_extension(&self) -> bool {
        (**self).has_extension()
    }

    #[inline]
    fn set_name(&mut self, name: String) {
        (**self).set_name(name);
    }

    #[inline]
    fn set_with_id(&mut self, value: bool) {
        (**self).set_with_id(value);
    }

    #[inline]
    fn set_generated(&mut self, value: bool) {
        (**self).set_generated(value);
    }

    #[inline]
    fn add_extension(&mut self, replace: bool, extension: String) {
        (**self).add_extension(replace, extension);
    }

    #[inline]
    fn strip_suffix(&mut self, suffix: &str) {
        (**self).strip_suffix(suffix);
    }

    #[inline]
    fn generate_unique_id(&mut self) {
        (**self).generate_unique_id();
    }
}

/// Helper trait that provides additional builder patterns to the [`NameBuilder`].
pub trait NameBuilderExt: Sized {
    /// Force the builder to generate a unique id.
    #[must_use]
    fn generate_id(self) -> Self;

    /// Tell the builder to add (`true`) or not to add (`false`) the unique id
    /// to the generated name.
    #[must_use]
    fn with_id(self, value: bool) -> Self;

    /// Add extensions to the builder using the passed iterator `iter`. If `replace`
    /// is set to `true` any existing extension is dropped before the new ones are
    /// applied.
    #[must_use]
    fn extend<I>(self, replace: bool, iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Display;

    /// Remove the specified `suffix` from the builder.
    #[must_use]
    fn remove_suffix(self, suffix: &str) -> Self;

    /// Instruct the builder to create a unique name from the passed `value`.
    ///
    /// This means, that the [`with_id`](NameBuilderExt::with_id) is automatically
    /// set to `false`.
    #[must_use]
    fn unique_name<T>(self, value: T) -> Self
    where
        T: Display;

    /// Instruct the builder to create a name that is shared between different parts
    /// of the code from the passed `value`.
    ///
    /// This means, that the [`with_id`](NameBuilderExt::with_id) is automatically
    /// set to `true` and the generated name has a unique id to be identified.
    #[must_use]
    fn shared_name<T>(self, value: T) -> Self
    where
        T: Display;

    /// If the builder does currently not have a name, the passed `fallback` is applied.
    #[must_use]
    fn or<T>(self, fallback: T) -> Self
    where
        T: NameFallback;

    /// If the builder does currently not have a name, the passed `fallback` is applied.
    #[must_use]
    fn or_else<F, T>(self, fallback: F) -> Self
    where
        F: FnOnce() -> T,
        T: NameFallback;
}

impl<X> NameBuilderExt for X
where
    X: NameBuilder + Sized,
{
    fn generate_id(mut self) -> Self {
        self.generate_unique_id();

        self
    }

    fn with_id(mut self, value: bool) -> Self {
        self.set_with_id(value);

        self
    }

    fn extend<I>(mut self, mut replace: bool, iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Display,
    {
        for ext in iter {
            let ext = ext.to_string();

            self.add_extension(take(&mut replace), ext);
        }

        self
    }

    fn remove_suffix(mut self, suffix: &str) -> Self {
        self.strip_suffix(suffix);

        self
    }

    fn unique_name<T>(mut self, value: T) -> Self
    where
        T: Display,
    {
        self.set_name(value.to_string());
        self.set_with_id(false);

        self
    }

    fn shared_name<T>(mut self, value: T) -> Self
    where
        T: Display,
    {
        self.set_name(value.to_string());
        self.set_with_id(true);

        self
    }

    fn or<T>(self, fallback: T) -> Self
    where
        T: NameFallback,
    {
        self.or_else(|| fallback)
    }

    fn or_else<F, T>(mut self, fallback: F) -> Self
    where
        F: FnOnce() -> T,
        T: NameFallback,
    {
        if !self.has_name() {
            fallback().apply(&mut self);
        }

        self
    }
}

/// Helper trait used in [`NameBuilderExt::or`] and [`NameBuilderExt::or_else`] to define
/// what can be used as fallback for a name.
pub trait NameFallback {
    /// Apply the fallback to the passed `builder`.
    fn apply(self, builder: &mut dyn NameBuilder);
}

impl NameFallback for &dyn NameBuilder {
    fn apply(self, builder: &mut dyn NameBuilder) {
        builder.merge(self);
    }
}

impl NameFallback for Box<dyn NameBuilder> {
    fn apply(self, builder: &mut dyn NameBuilder) {
        builder.merge(&*self);
    }
}

impl NameFallback for Name {
    #[inline]
    fn apply(self, builder: &mut dyn NameBuilder) {
        (&self).apply(builder);
    }
}

impl NameFallback for &Name {
    #[inline]
    fn apply(self, builder: &mut dyn NameBuilder) {
        builder.set_name(self.as_str().to_owned());
        builder.set_generated(self.is_generated());
        builder.set_with_id(false);
    }
}

impl NameFallback for Option<&Name> {
    #[inline]
    fn apply(self, builder: &mut dyn NameBuilder) {
        if let Some(x) = self {
            x.apply(builder);
        }
    }
}

impl NameFallback for Option<Name> {
    #[inline]
    fn apply(self, builder: &mut dyn NameBuilder) {
        self.as_ref().apply(builder);
    }
}

impl NameFallback for &Option<Name> {
    fn apply(self, builder: &mut dyn NameBuilder) {
        self.as_ref().apply(builder);
    }
}

impl NameFallback for &String {
    fn apply(self, builder: &mut dyn NameBuilder) {
        builder.set_name(self.to_owned());
        builder.set_with_id(false);
    }
}

impl NameFallback for Option<&String> {
    fn apply(self, builder: &mut dyn NameBuilder) {
        if let Some(x) = self {
            x.apply(builder);
        }
    }
}

impl NameFallback for Option<String> {
    fn apply(self, builder: &mut dyn NameBuilder) {
        self.as_ref().apply(builder);
    }
}

impl NameFallback for &Option<String> {
    fn apply(self, builder: &mut dyn NameBuilder) {
        self.as_ref().apply(builder);
    }
}

impl NameFallback for &str {
    fn apply(self, builder: &mut dyn NameBuilder) {
        builder.set_name(self.to_owned());
        builder.set_with_id(false);
    }
}

impl NameFallback for Option<&str> {
    fn apply(self, builder: &mut dyn NameBuilder) {
        if let Some(x) = self {
            x.apply(builder);
        }
    }
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
