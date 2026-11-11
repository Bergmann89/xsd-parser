use std::sync::{atomic::AtomicUsize, Arc};

use inflector::Inflector;
use proc_macro2::Ident as Ident2;

use crate::config::MetaType;
use crate::traits::{NameBuilder as NameBuilderTrait, Naming as NamingTrait};
use crate::TypeIdent;

use super::{ExplicitNameBuilder, ExplicitNaming, Name, NameBuilder};

/// The [`Naming`](super::Naming) of `xsd-parser` up to version 1.7.
///
/// Use this to keep the generated names of older versions. It splits words
/// with [`Inflector`], which separates acronyms from their digits and plural
/// `s` (`MP3s` becomes `mp_3s`, `UserIDs` becomes `user_i_ds`).
#[derive(Default, Debug, Clone)]
pub struct LegacyNaming(Arc<AtomicUsize>);

impl NamingTrait for LegacyNaming {
    fn clone_boxed(&self) -> Box<dyn NamingTrait> {
        Box::new(self.clone())
    }

    fn builder(&self) -> Box<dyn NameBuilderTrait> {
        Box::new(NameBuilder::new(self.0.clone(), Box::new(self.clone())))
    }

    fn unify(&self, s: &str) -> String {
        unify_string(s)
    }

    fn make_type_name(&self, postfixes: &[String], ty: &MetaType, ident: &TypeIdent) -> Name {
        super::make_type_name(self, postfixes, ty, ident)
    }

    fn make_unknown_variant(&self, id: usize) -> Ident2 {
        super::format_unknown_variant(id)
    }

    fn format_module_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s).to_snake_case())
    }

    fn format_type_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s))
    }

    fn format_field_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s).to_snake_case())
    }

    fn format_variant_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s))
    }

    fn format_constant_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s).to_screaming_snake_case())
    }
}

/// The [`ExplicitNaming`] of `xsd-parser` up to version 1.7.
///
/// See [`LegacyNaming`] for the difference to the current naming.
#[derive(Default, Debug, Clone)]
pub struct LegacyExplicitNaming(ExplicitNaming);

impl LegacyExplicitNaming {
    /// Create a new [`LegacyExplicitNaming`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// See [`ExplicitNaming::unify_names`].
    #[must_use]
    pub fn unify_names(self, value: bool) -> Self {
        Self(self.0.unify_names(value))
    }

    /// See [`ExplicitNaming::with_element_field_postfix`].
    #[must_use]
    pub fn with_element_field_postfix<S: Into<String>>(self, postfix: S) -> Self {
        Self(self.0.with_element_field_postfix(postfix))
    }

    /// See [`ExplicitNaming::with_attribute_field_postfix`].
    #[must_use]
    pub fn with_attribute_field_postfix<S: Into<String>>(self, postfix: S) -> Self {
        Self(self.0.with_attribute_field_postfix(postfix))
    }
}

impl NamingTrait for LegacyExplicitNaming {
    fn clone_boxed(&self) -> Box<dyn NamingTrait> {
        Box::new(self.clone())
    }

    fn builder(&self) -> Box<dyn NameBuilderTrait> {
        Box::new(ExplicitNameBuilder::new(
            self.0.id.clone(),
            self.clone_boxed(),
        ))
    }

    fn unify(&self, s: &str) -> String {
        if self.0.unify_names {
            unify_string(s)
        } else {
            s.replace(|c: char| !c.is_alphanumeric(), "_")
        }
    }

    fn make_type_name(&self, postfixes: &[String], ty: &MetaType, ident: &TypeIdent) -> Name {
        let _ty = ty;
        let postfix = postfixes.get(ident.type_ as usize).map(String::as_str);

        let s = self.format_type_name(ident.name.as_str());

        Name::new_generated(if let Some(postfix) = postfix {
            format!("{s}{postfix}")
        } else {
            s
        })
    }

    fn make_unknown_variant(&self, id: usize) -> Ident2 {
        super::format_unknown_variant(id)
    }

    fn format_module_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s).to_snake_case())
    }

    fn format_type_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s))
    }

    fn format_field_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s).to_snake_case())
    }

    fn format_variant_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s))
    }

    fn format_constant_name(&self, s: &str) -> String {
        super::format_ident(self.unify(s).to_screaming_snake_case())
    }

    fn format_element_field_name(&self, s: &str) -> String {
        if let Some(postfix) = &self.0.element_field_postfix {
            self.format_field_name(&format!("{s}{postfix}"))
        } else {
            self.format_field_name(s)
        }
    }

    fn format_attribute_field_name(&self, s: &str) -> String {
        if let Some(postfix) = &self.0.attribute_field_postfix {
            self.format_field_name(&format!("{s}{postfix}"))
        } else {
            self.format_field_name(s)
        }
    }
}

/// The `unify_string` of `xsd-parser` up to version 1.7.
fn unify_string(s: &str) -> String {
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

#[cfg(test)]
mod tests {
    use crate::traits::Naming as _;

    use super::{LegacyExplicitNaming, LegacyNaming};

    #[test]
    fn legacy_naming_keeps_old_names() {
        let naming = LegacyNaming::default();

        assert_eq!("FuuBarBaz", naming.unify("fuu+Bar-BAZ"));
        assert_eq!("UserIDs", naming.format_type_name("UserIDs"));
        assert_eq!("user_i_ds", naming.format_field_name("UserIDs"));
        assert_eq!("mp_3s", naming.format_field_name("MP3s"));
        assert_eq!("content_103", naming.format_field_name("Content103"));
        assert_eq!("Dominant11Th", naming.format_variant_name("dominant-11th"));
    }

    #[test]
    fn legacy_explicit_naming_keeps_old_names() {
        let naming = LegacyExplicitNaming::default().with_attribute_field_postfix("_attrib");

        assert_eq!("user_i_ds", naming.format_field_name("UserIDs"));
        assert_eq!("au_attrib", naming.format_attribute_field_name("aU"));
        assert_eq!(
            "user_i_ds",
            LegacyExplicitNaming::default()
                .unify_names(false)
                .format_field_name("UserIDs")
        );
    }
}
