mod explicit;

use std::any::Any;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crate::traits::{NameBuilder as NameBuilderTrait, Naming as NamingTrait};

use super::Name;

pub use self::explicit::{ExplicitNameBuilder, ExplicitNaming};

/// Default name generation and formatting implementation.
///
/// This type implements the [`Naming`](NamingTrait) trait that is used for
/// naming generation and formatting.
#[derive(Default, Debug, Clone)]
pub struct Naming(Arc<AtomicUsize>);

impl NamingTrait for Naming {
    fn clone_boxed(&self) -> Box<dyn NamingTrait> {
        Box::new(self.clone())
    }

    fn builder(&self) -> Box<dyn NameBuilderTrait> {
        Box::new(NameBuilder::new(self.0.clone(), self.clone_boxed()))
    }
}

/// Default implementation for the [`NameBuilder`](NameBuilderTrait) trait.
#[must_use]
#[derive(Debug)]
pub struct NameBuilder {
    id: Arc<AtomicUsize>,
    my_id: Option<usize>,
    with_id: bool,
    generated: bool,

    name: Option<String>,
    extension: Option<String>,

    naming: Box<dyn NamingTrait>,
}

impl NameBuilder {
    /// Create a new [`NameBuilder`] instance.
    ///
    /// The passed `id` is used to generate unique ids for unnamed types.
    pub fn new(id: Arc<AtomicUsize>, naming: Box<dyn NamingTrait>) -> Self {
        Self {
            id,
            my_id: None,
            with_id: true,
            generated: false,
            name: None,
            extension: None,
            naming,
        }
    }
}

impl Clone for NameBuilder {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            my_id: self.my_id,
            with_id: self.with_id,
            generated: self.generated,
            name: self.name.clone(),
            extension: self.extension.clone(),
            naming: self.naming.clone_boxed(),
        }
    }
}

impl NameBuilderTrait for NameBuilder {
    fn finish(&self) -> Name {
        let Self {
            id,
            my_id,
            with_id,
            mut generated,
            name,
            extension,
            naming,
        } = self;

        let mut ret = String::new();
        if let Some(s) = extension {
            generated = true;
            ret.push_str(&naming.unify(s));
        }

        if let Some(s) = name {
            if ret.is_empty() {
                ret.push_str(s);
            } else {
                ret.push_str(&naming.unify(s));
            }
        }

        if ret.is_empty() {
            generated = true;
            ret.push_str("Unnamed");
        }

        if *with_id {
            generated = true;
            let id = my_id.unwrap_or_else(|| id.fetch_add(1, Ordering::Relaxed));
            ret = format!("{ret}{id}");
        }

        if generated {
            Name::new_generated(ret)
        } else {
            Name::new_named(ret)
        }
    }

    fn merge(&mut self, other: &dyn NameBuilderTrait) {
        let other: &Self = (other as &dyn Any).downcast_ref().unwrap();

        if let Some(name) = other.name.clone() {
            self.name.get_or_insert(name);
            self.with_id = other.with_id;
            self.generated = other.generated;

            if let Some(id) = other.my_id {
                self.with_id = other.with_id;
                self.my_id.get_or_insert(id);
            }

            if let Some(ext) = other.extension.clone() {
                self.extension.get_or_insert(ext);
            }
        }
    }

    fn clone_boxed(&self) -> Box<dyn NameBuilderTrait> {
        Box::new(self.clone())
    }

    fn has_name(&self) -> bool {
        self.name.is_some()
    }

    fn has_extension(&self) -> bool {
        self.extension.is_some()
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn set_with_id(&mut self, value: bool) {
        self.with_id = value;
    }

    fn set_generated(&mut self, value: bool) {
        self.generated = value;
    }

    fn add_extension(&mut self, replace: bool, extension: String) {
        let s = self.naming.unify(&extension);

        if replace {
            self.extension = Some(s);
        } else if let Some(prefix) = &self.extension {
            self.extension = Some(format!("{s}{prefix}"));
        } else {
            self.extension = Some(s);
        }
    }

    fn strip_suffix(&mut self, suffix: &str) {
        if let Some(s) = &mut self.name {
            if let Some(x) = s.strip_suffix(suffix) {
                *s = x.into();
            }
        }

        if let Some(s) = &mut self.extension {
            if let Some(x) = s.strip_suffix(suffix) {
                *s = x.into();
            }
        }
    }

    fn generate_unique_id(&mut self) {
        if self.my_id.is_none() {
            self.my_id = Some(self.id.fetch_add(1, Ordering::Release));
        }
    }

    fn prepare_type_name(&mut self) {
        self.strip_suffix("Type");
        self.strip_suffix("Content");
    }

    fn prepare_field_name(&mut self) {
        self.strip_suffix("Type");
        self.strip_suffix("Content");
    }

    fn prepare_content_type_name(&mut self) {
        self.strip_suffix("Type");
        self.strip_suffix("Content");
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Naming as _;

    use super::Naming;

    #[test]
    fn unify() {
        let naming = Naming::default();

        assert_eq!("_", naming.unify("+"));
        assert_eq!("FuuBarBaz", naming.unify("Fuu_BAR_BAZ"));
        assert_eq!("FuuBarBaz", naming.unify("fuu_bar_baz"));
        assert_eq!("FuuBarBaz", naming.unify("fuu+Bar-BAZ"));
    }
}
