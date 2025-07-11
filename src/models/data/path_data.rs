use proc_macro2::TokenStream;
use quote::quote;
use smallvec::{smallvec, SmallVec};

use crate::models::code::{IdentPath, ModulePath};

/// Represents a type ready to be resolved and rendered for a specific module.
///
/// It contains information like the actual path to the type, a list of generics
/// and a list of usings to add to the module.
#[derive(Debug, Clone)]
pub struct PathData {
    /// Actual path of the target type.
    pub path: IdentPath,

    /// Usings to add for this target type.
    pub usings: SmallVec<[String; 1]>,

    /// Generics of this target type.
    pub generics: SmallVec<[IdentPath; 1]>,
}

impl PathData {
    /// Create a new [`PathData`] instance from the passed `path`.
    #[must_use]
    pub fn from_path(path: IdentPath) -> Self {
        Self {
            path,
            usings: smallvec![],
            generics: smallvec![],
        }
    }

    /// Add a generic argument to the path data.
    #[must_use]
    pub fn with_generic(mut self, path: IdentPath) -> Self {
        self.generics.push(path);

        self
    }

    /// Add a using to the path data.
    #[must_use]
    pub fn with_using<P: Into<String>>(mut self, path: P) -> Self {
        self.usings.push(path.into());

        self
    }

    /// Resolves the target type relative to the passed module `path` and
    /// returns the rendered type as [`TokenStream`].
    #[must_use]
    pub fn resolve_relative_to(&self, path: &ModulePath) -> TokenStream {
        let mut ret = self.path.relative_to(path);

        if !self.generics.is_empty() {
            ret.extend(quote!(<));

            for x in &self.generics {
                ret.extend(x.relative_to(path));
            }

            ret.extend(quote!(>));
        }

        ret
    }
}
