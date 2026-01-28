use std::any::Any;
use std::sync::{atomic::AtomicUsize, Arc};

use crate::config::MetaType;
use crate::traits::{NameBuilder as NameBuilderTrait, Naming as NamingTrait};
use crate::TypeIdent;

use super::{Name, NameBuilder};

/// A more explicit version of [`Naming`](super::Naming) that does not add or
/// remove any suffixes or prefixes from the generated name expect the one that
/// were configured by [`Generator::with_type_postfix`](crate::pipeline::Generator::with_type_postfix).
///
/// This may result in repeated string inside the generated names (like
/// `FooTypeType`), but will reduce naming conflicts in most cases.
#[derive(Default, Debug, Clone)]
pub struct ExplicitNaming {
    id: Arc<AtomicUsize>,
    element_field_postfix: Option<String>,
    attribute_field_postfix: Option<String>,
}

impl ExplicitNaming {
    /// Create a new [`ExplicitNaming`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the postfix to add to generated element field names.
    ///
    /// This can be useful to avoid naming conflicts between element and attribute fields.
    #[must_use]
    pub fn with_element_field_postfix<S: Into<String>>(mut self, postfix: S) -> Self {
        self.element_field_postfix = Some(postfix.into());

        self
    }

    /// Set the postfix to add to generated attribute field names.
    ///
    /// This can be useful to avoid naming conflicts between element and attribute fields.
    #[must_use]
    pub fn with_attribute_field_postfix<S: Into<String>>(mut self, postfix: S) -> Self {
        self.attribute_field_postfix = Some(postfix.into());

        self
    }
}

impl NamingTrait for ExplicitNaming {
    fn clone_boxed(&self) -> Box<dyn NamingTrait> {
        Box::new(self.clone())
    }

    fn builder(&self) -> Box<dyn NameBuilderTrait> {
        Box::new(ExplicitNameBuilder::new(
            self.id.clone(),
            self.clone_boxed(),
        ))
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

    fn format_element_field_name(&self, s: &str) -> String {
        if let Some(postfix) = &self.element_field_postfix {
            self.format_field_name(&format!("{s}{postfix}"))
        } else {
            self.format_field_name(s)
        }
    }

    fn format_attribute_field_name(&self, s: &str) -> String {
        if let Some(postfix) = &self.attribute_field_postfix {
            self.format_field_name(&format!("{s}{postfix}"))
        } else {
            self.format_field_name(s)
        }
    }
}

/// A more explicit version of [`NameBuilder`] that does not add or remove any
/// suffixes or prefixes from the generated name. This may lead to duplicated
/// strings inside the generated name (like `FuuTypeType`), but is will reduce
/// naming conflicts in most cases.
#[must_use]
#[derive(Debug, Clone)]
pub struct ExplicitNameBuilder(NameBuilder);

impl ExplicitNameBuilder {
    /// Create a new [`ExplicitNameBuilder`] instance.
    ///
    /// The passed `id` is used to generate unique ids for unnamed types.
    pub fn new(id: Arc<AtomicUsize>, naming: Box<dyn NamingTrait>) -> Self {
        Self(NameBuilder::new(id, naming))
    }
}

impl NameBuilderTrait for ExplicitNameBuilder {
    #[inline]
    fn finish(&self) -> Name {
        self.0.finish()
    }

    #[inline]
    fn merge(&mut self, other: &dyn NameBuilderTrait) {
        let other: &Self = (other as &dyn Any).downcast_ref().unwrap();

        self.0.merge(&other.0);
    }

    #[inline]
    fn clone_boxed(&self) -> Box<dyn NameBuilderTrait> {
        Box::new(self.clone())
    }

    #[inline]
    fn has_name(&self) -> bool {
        self.0.has_name()
    }

    #[inline]
    fn has_extension(&self) -> bool {
        self.0.has_extension()
    }

    #[inline]
    fn set_name(&mut self, name: String) {
        self.0.set_name(name);
    }

    #[inline]
    fn set_with_id(&mut self, value: bool) {
        self.0.set_with_id(value);
    }

    #[inline]
    fn set_generated(&mut self, value: bool) {
        self.0.set_generated(value);
    }

    #[inline]
    fn add_extension(&mut self, replace: bool, extension: String) {
        self.0.add_extension(replace, extension);
    }

    #[inline]
    fn strip_suffix(&mut self, suffix: &str) {
        let _suffix = suffix;
    }

    fn generate_unique_id(&mut self) {
        self.0.generate_unique_id();
    }

    fn prepare_type_name(&mut self) {}

    fn prepare_field_name(&mut self) {}

    fn prepare_content_type_name(&mut self) {}
}
