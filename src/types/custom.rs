//! Contains the [`CustomType`] type information and all related types.

use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};

use proc_macro2::TokenStream;

/// Type information for a custom defined type.
pub struct CustomType {
    name: &'static str,
    default: Option<Box<dyn CustomDefaultImpl>>,
}

impl CustomType {
    /// Create a new custom type information with the passed `name`.
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            default: None,
        }
    }

    /// Get the name of the custom defined type.
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Try to get the default value (as code) for the given string.
    ///
    /// This is used to translate default values specified in the XSD schema,
    /// to suitable rust code.
    #[must_use]
    pub fn default(&self, s: &str) -> Option<TokenStream> {
        self.default.as_ref()?.exec(s)
    }

    /// Set the handler for the default values for this custom defined type.
    #[must_use]
    pub fn with_default<X: CustomDefaultImpl>(mut self, x: X) -> Self {
        self.default = Some(Box::new(x));

        self
    }
}

impl Clone for CustomType {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            default: self
                .default
                .as_ref()
                .map(|x| CustomDefaultImpl::clone(&**x)),
        }
    }
}

impl Debug for CustomType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("CustomType")
            .field("name", &self.name)
            .field("default", &self.default.is_some())
            .finish()
    }
}

impl Eq for CustomType {}

impl PartialEq for CustomType {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl Hash for CustomType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// Trait that converts the default value of a element specified in the XML
/// schema to actual default code.
///
/// You can add a custom default implementation to your custom type using
/// [`CustomType::with_default`].
pub trait CustomDefaultImpl: Send + Sync + 'static {
    /// Try to convert the passed string `s` that contains the default value from
    /// the XML schema to actual default code. If the value could not be converted
    /// to code `None` is returned.
    fn exec(&self, s: &str) -> Option<TokenStream>;

    /// Clone this instance and return it as a box.
    fn clone(&self) -> Box<dyn CustomDefaultImpl>;
}

impl<X> CustomDefaultImpl for X
where
    X: Fn(&str) -> Option<TokenStream> + Clone + Send + Sync + 'static,
{
    fn exec(&self, s: &str) -> Option<TokenStream> {
        (*self)(s)
    }

    fn clone(&self) -> Box<dyn CustomDefaultImpl> {
        Box::new(self.clone())
    }
}
