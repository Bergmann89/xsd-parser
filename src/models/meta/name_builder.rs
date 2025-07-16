use std::fmt::Display;
use std::mem::take;
use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc};

use crate::models::Name;

/// Builder type to construct a [`Name`].
#[must_use]
#[derive(Debug, Clone)]
pub struct NameBuilder {
    id: Arc<AtomicUsize>,
    my_id: Option<usize>,
    with_id: bool,
    generated: bool,

    name: Option<String>,
    extension: Option<String>,
}

impl NameBuilder {
    /// Create a new [`NameBuilder`] instance.
    ///
    /// The passed `id` is used to generate unique ids for unnamed types.
    pub fn new(id: Arc<AtomicUsize>) -> Self {
        Self {
            id,
            my_id: None,
            with_id: true,
            generated: false,
            name: None,
            extension: None,
        }
    }

    /// Finish the builder and create the [`Name`] instance.
    #[must_use]
    pub fn finish(self) -> Name {
        let Self {
            id,
            my_id,
            with_id,
            mut generated,
            name,
            extension,
        } = self;

        let mut ret = String::new();
        if let Some(s) = extension {
            generated = true;
            ret.push_str(&Name::unify(&s));
        }

        if let Some(s) = name {
            if ret.is_empty() {
                ret.push_str(&s);
            } else {
                ret.push_str(&Name::unify(&s));
            }
        }

        if ret.is_empty() {
            generated = true;
            ret.push_str("Unnamed");
        }

        if with_id {
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

    /// Wether to add a unique id to the generated name or not.
    pub fn with_id(mut self, value: bool) -> Self {
        self.with_id = value;

        self
    }

    /// Generate the id for the name.
    ///
    /// This can be useful if you want to clone the builder to generate multiple names
    /// with the same id. For example for a field name and corresponding field type.
    pub fn generate_id(mut self) -> Self {
        if self.my_id.is_none() {
            self.my_id = Some(self.id.fetch_add(1, Ordering::Release));
        }

        self
    }

    /// Set a unique name.
    ///
    /// This will automatically set `with_id` to `false`.
    pub fn unique_name<T>(mut self, value: T) -> Self
    where
        T: Display,
    {
        self.name = Some(value.to_string());
        self.with_id = false;

        self
    }

    /// Set a shared name.
    ///
    /// This will automatically set `with_id` to `true`.
    pub fn shared_name<T>(mut self, value: T) -> Self
    where
        T: Display,
    {
        self.name = Some(value.to_string());
        self.with_id = true;

        self
    }

    /// Uses the name that is already stored in the builder, or the passed
    /// `fallback` value if the name was not set yet.
    pub fn or<T>(self, fallback: T) -> Self
    where
        T: NameFallback,
    {
        self.or_else(|| fallback)
    }

    /// Uses the name that is already stored in the builder, or the value that
    /// is returned by the passed `fallback` closure if the name was not set yet.
    pub fn or_else<F, T>(mut self, fallback: F) -> Self
    where
        F: FnOnce() -> T,
        T: NameFallback,
    {
        if self.name.is_none() {
            fallback().apply(&mut self);
        }

        self
    }

    /// Add a extension to the name.
    ///
    /// Extensions are added as to the generated name as prefix.
    pub fn extend<I>(mut self, mut replace: bool, iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Display,
    {
        for s in iter {
            let s = s.to_string();
            let s = Name::unify(&s);

            if take(&mut replace) {
                self.extension = Some(s);
            } else if let Some(prefix) = &self.extension {
                self.extension = Some(format!("{s}{prefix}"));
            } else {
                self.extension = Some(s);
            }
        }

        self
    }

    /// Remove the specified `suffix` from the name and the extension.
    pub fn remove_suffix(mut self, suffix: &str) -> Self {
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

        self
    }

    /// Returns `true` if a extension was specified, `false` otherwise.
    #[inline]
    #[must_use]
    pub fn has_extension(&self) -> bool {
        self.extension.is_some()
    }
}

/// Helper trait to define fallback values passed to [`NameBuilder::or`] or
/// [`NameBuilder::or_else`]
pub trait NameFallback {
    /// Create the fallback value.
    fn apply(self, builder: &mut NameBuilder);
}

impl NameFallback for NameBuilder {
    #[inline]
    fn apply(self, builder: &mut NameBuilder) {
        if let Some(name) = self.name {
            builder.name.get_or_insert(name);
            builder.with_id = self.with_id;
            builder.generated = self.generated;

            if let Some(id) = self.my_id {
                builder.with_id = self.with_id;
                builder.my_id.get_or_insert(id);
            }

            if let Some(ext) = self.extension {
                builder.extension.get_or_insert(ext);
            }
        }
    }
}

impl NameFallback for Name {
    #[inline]
    fn apply(self, builder: &mut NameBuilder) {
        (&self).apply(builder);
    }
}

impl NameFallback for &Name {
    #[inline]
    fn apply(self, builder: &mut NameBuilder) {
        builder.name = Some(self.as_str().to_owned());
        builder.generated = self.is_generated();
        builder.with_id = false;
    }
}

impl NameFallback for Option<&Name> {
    #[inline]
    fn apply(self, builder: &mut NameBuilder) {
        if let Some(x) = self {
            x.apply(builder);
        }
    }
}

impl NameFallback for Option<Name> {
    #[inline]
    fn apply(self, builder: &mut NameBuilder) {
        self.as_ref().apply(builder);
    }
}

impl NameFallback for &Option<Name> {
    fn apply(self, builder: &mut NameBuilder) {
        self.as_ref().apply(builder);
    }
}

impl NameFallback for &String {
    fn apply(self, builder: &mut NameBuilder) {
        builder.name = Some(self.to_owned());
        builder.with_id = false;
    }
}

impl NameFallback for Option<&String> {
    fn apply(self, builder: &mut NameBuilder) {
        if let Some(x) = self {
            x.apply(builder);
        }
    }
}

impl NameFallback for Option<String> {
    fn apply(self, builder: &mut NameBuilder) {
        self.as_ref().apply(builder);
    }
}

impl NameFallback for &Option<String> {
    fn apply(self, builder: &mut NameBuilder) {
        self.as_ref().apply(builder);
    }
}

impl NameFallback for &str {
    fn apply(self, builder: &mut NameBuilder) {
        builder.name = Some(self.to_owned());
        builder.with_id = false;
    }
}

impl NameFallback for Option<&str> {
    fn apply(self, builder: &mut NameBuilder) {
        if let Some(x) = self {
            x.apply(builder);
        }
    }
}
