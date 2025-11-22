use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote, ToTokens};
use smallvec::SmallVec;
use thiserror::Error;

use crate::models::{
    meta::MetaTypes,
    schema::{NamespaceId, SchemaId},
    Ident,
};

/// Represents an identifier path.
///
/// A identifier path is the full path of a specific identifier in the code,
/// like `std::str::FromStr`. The identified object can be a constant, a type,
/// a trait or anything else that is defined within a module.
///
/// The identifier path contains two parts:
/// - The identifier itself, which is more or less the name of the object to identify, and
/// - the path of the module the object is provided at.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct IdentPath {
    path: Option<ModulePath>,
    ident: Ident2,
    is_absolute: bool,
}

/// Represents a path of a module.
///
/// The module path is a chain ob module names separated by a double colon like
/// `std::str`. It is used to identify modules inside the code.
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ModulePath(pub SmallVec<[Ident2; 2]>);

/// Error that is raised by [`IdentPath`] if parsing the value from string failed.
#[derive(Debug, Error)]
#[error("Invalid identifier path: {0}")]
pub struct InvalidIdentPath(pub String);

/// Identifies the module the code gets rendered to.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ModuleIdent {
    /// The root module.
    Root,

    /// The module of the namespace of the current type.
    Namespace(NamespaceId),

    /// The module of the schema of the current type.
    Schema(SchemaId),

    /// The module of the namespace and the schema of the current type.
    Both(NamespaceId, SchemaId),
}

impl IdentPath {
    /// Crates a new [`IdentPath`] instance from the passed module `path` and the
    /// `ident`ifier of the object to refer to.
    #[must_use]
    pub fn from_parts<I>(path: I, ident: Ident2) -> Self
    where
        I: IntoIterator<Item = Ident2>,
    {
        Self::from_ident(ident).with_path(path)
    }

    /// Creates a new [`IdentPath`] from the passed object `ident`ifier without a
    /// module path.
    #[must_use]
    pub fn from_ident(ident: Ident2) -> Self {
        Self {
            ident,
            path: None,
            is_absolute: false,
        }
    }

    /// Changes the identifier of this identifier path to the passed `ident`.
    #[must_use]
    pub fn with_ident(mut self, ident: Ident2) -> Self {
        self.ident = ident;

        self
    }

    /// Changes the module path of this identifier path to the passed `path`.
    #[must_use]
    pub fn with_path<I>(mut self, path: I) -> Self
    where
        I: IntoIterator<Item = Ident2>,
    {
        self.path = Some(ModulePath(path.into_iter().collect()));

        self
    }

    /// Splits this identifier path into it's two main parts, the identifier
    /// and the module path.
    #[must_use]
    pub fn into_parts(self) -> (Ident2, Option<ModulePath>, bool) {
        let Self {
            ident,
            path,
            is_absolute,
        } = self;

        (ident, path, is_absolute)
    }

    /// Returns the identifier of this identifier path.
    #[must_use]
    pub fn ident(&self) -> &Ident2 {
        &self.ident
    }

    /// Returns the module path for this identifier path.
    #[must_use]
    pub fn module(&self) -> Option<&ModulePath> {
        self.path.as_ref()
    }

    /// Returns `true` if the path is absolute, `false` otherwise.
    #[must_use]
    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    /// Creates a [`TokenStream`] that is relative to the passed `dst` module path.
    ///
    /// This uses the `super` keyword to create a relative path from the passed `dst` module path
    /// and this identifier path. The relative path is returned as token stream.
    #[must_use]
    pub fn relative_to(&self, dst: &ModulePath) -> TokenStream {
        let ident = &self.ident;

        let Some(src) = &self.path else {
            return quote!(#ident);
        };

        let mut ret = TokenStream::new();
        if self.is_absolute {
            for p in src.0.iter() {
                ret.extend(quote!(::#p));
            }

            return quote!(#ret::#ident);
        }

        let mut src = src.0.iter().fuse();
        let mut dst = dst.0.iter().fuse();

        macro_rules! push {
            ($x:expr) => {{
                let x = $x;
                if ret.is_empty() {
                    ret.extend(x)
                } else {
                    ret.extend(quote!(::#x))
                }
            }};
        }

        loop {
            match (src.next(), dst.next()) {
                (Some(a), Some(b)) if a == b => {}
                (Some(a), Some(_)) => {
                    push!(quote!(super));
                    while dst.next().is_some() {
                        push!(quote!(super));
                    }

                    push!(quote!(#a));
                    for a in src {
                        push!(quote!(#a));
                    }

                    push!(quote!(#ident));

                    return ret;
                }
                (Some(a), None) => push!(quote!(#a)),
                (None, Some(_)) => push!(quote!(super)),
                (None, None) => {
                    push!(quote!(#ident));
                    return ret;
                }
            }
        }
    }
}

impl TryFrom<&str> for IdentPath {
    type Error = InvalidIdentPath;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for IdentPath {
    type Error = InvalidIdentPath;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl FromStr for IdentPath {
    type Err = InvalidIdentPath;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ident = None;
        let mut path = ModulePath::default();
        let mut is_absolute = false;

        for part in s.split("::") {
            let part = part.trim();
            if part.is_empty() {
                if path.is_empty() && ident.is_none() {
                    is_absolute = true;
                }

                continue;
            }

            if let Some(ident) = ident.take() {
                path.0.push(ident);
            }

            ident = Some(format_ident!("{part}"));
        }

        Ok(Self {
            ident: ident.ok_or_else(|| InvalidIdentPath(s.into()))?,
            path: Some(path),
            is_absolute,
        })
    }
}

impl Display for IdentPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(path) = &self.path {
            for module in &path.0 {
                write!(f, "{module}::")?;
            }
        }

        write!(f, "{}", self.ident)
    }
}

impl ToTokens for IdentPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.is_absolute {
            tokens.extend(quote!(::));
        }

        if let Some(path) = &self.path {
            for module in &path.0 {
                tokens.extend(quote!(#module::));
            }
        }

        let ident = self.ident();

        tokens.extend(quote!(#ident));
    }
}

impl ModulePath {
    /// Create a new [`ModulePath`] instance from the passed namespace id `ns` and the
    /// `types` information.
    ///
    /// This tries to look up the passed namespace id in the types information and create
    /// a module path for it.
    #[must_use]
    pub fn from_ident(types: &MetaTypes, ident: ModuleIdent) -> Self {
        let (namespace, schema) = match ident {
            ModuleIdent::Root => (None, None),
            ModuleIdent::Namespace(n) => (Some(n), None),
            ModuleIdent::Schema(s) => (None, Some(s)),
            ModuleIdent::Both(n, s) => (Some(n), Some(s)),
        };

        let namespace = namespace
            .and_then(|id| types.modules.get(&id))
            .and_then(|module| module.name.as_ref())
            .map(|name| types.naming.format_module_ident(name));
        let schema = schema
            .and_then(|id| types.schemas.get(&id))
            .and_then(|schema| schema.name.as_ref())
            .map(|name| types.naming.format_module_ident(name));

        Self(namespace.into_iter().chain(schema).collect())
    }

    /// Add a module to the module
    #[must_use]
    pub fn join(mut self, other: Ident2) -> Self {
        self.0.push(other);

        self
    }
}

impl Deref for ModulePath {
    type Target = SmallVec<[Ident2; 2]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ModulePath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/* ModuleIdent */

impl ModuleIdent {
    pub(crate) fn new(
        types: &MetaTypes,
        ident: &Ident,
        with_namespace: bool,
        with_schema: bool,
    ) -> Self {
        let schema_count = ident
            .ns
            .as_ref()
            .and_then(|ns| types.modules.get(ns))
            .map(|m| m.schema_count)
            .unwrap_or_default();
        let with_schema = with_schema && schema_count > 1;

        let namespace = with_namespace.then_some(ident.ns).flatten();
        let schema = with_schema
            .then(|| types.get_type(ident))
            .flatten()
            .and_then(|ty| ty.schema);

        match (namespace, schema) {
            (None, None) => ModuleIdent::Root,
            (Some(n), None) => ModuleIdent::Namespace(n),
            (None, Some(s)) => ModuleIdent::Schema(s),
            (Some(n), Some(s)) => ModuleIdent::Both(n, s),
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::{format_ident, quote};

    use super::{IdentPath, ModulePath};

    #[test]
    #[rustfmt::skip]
    fn type_path() {
        let string = IdentPath::from_ident(format_ident!("String"));
        let my_type = IdentPath::from_parts(
            [format_ident!("my_module")],
            format_ident!("MyType"),
        );
        let serializer = IdentPath::from_parts(
            [
                format_ident!("my_module"),
                format_ident!("quick_xml_serialize"),
            ],
            format_ident!("MyTypeSerializer"),
        );
        let deserializer = IdentPath::from_parts(
            [
                format_ident!("my_module"),
                format_ident!("quick_xml_deserialize"),
            ],
            format_ident!("MyTypeDeserializer"),
        );

        let empty_path = ModulePath::default();
        let module_path = ModulePath::default().join(format_ident!("my_module"));
        let other_module_path = ModulePath::default().join(format_ident!("other_module"));
        let serializer_path = module_path.clone().join(format_ident!("quick_xml_serialize"));
        let deserializer_path = module_path.clone().join(format_ident!("quick_xml_deserialize"));

        macro_rules! test {
            ($actual:expr, $( $expected:tt )*) => {{
                let a = $actual.to_string();
                let b = quote!($( $expected )*).to_string();

                assert_eq!(a, b);
            }};
        }

        /* With modules */

        test!(string.relative_to(&empty_path), String);
        test!(string.relative_to(&module_path), String);
        test!(string.relative_to(&other_module_path), String);
        test!(string.relative_to(&serializer_path), String);
        test!(string.relative_to(&deserializer_path), String);

        test!(my_type.relative_to(&empty_path), my_module::MyType);
        test!(my_type.relative_to(&module_path), MyType);
        test!(my_type.relative_to(&other_module_path), super::my_module::MyType);
        test!(my_type.relative_to(&serializer_path), super::MyType);
        test!(my_type.relative_to(&deserializer_path), super::MyType);

        test!(serializer.relative_to(&empty_path), my_module::quick_xml_serialize::MyTypeSerializer);
        test!(serializer.relative_to(&module_path), quick_xml_serialize::MyTypeSerializer);
        test!(serializer.relative_to(&other_module_path), super::my_module::quick_xml_serialize::MyTypeSerializer);
        test!(serializer.relative_to(&serializer_path), MyTypeSerializer);
        test!(serializer.relative_to(&deserializer_path), super::quick_xml_serialize::MyTypeSerializer);

        test!(deserializer.relative_to(&empty_path), my_module::quick_xml_deserialize::MyTypeDeserializer);
        test!(deserializer.relative_to(&module_path), quick_xml_deserialize::MyTypeDeserializer);
        test!(deserializer.relative_to(&other_module_path), super::my_module::quick_xml_deserialize::MyTypeDeserializer);
        test!(deserializer.relative_to(&serializer_path), super::quick_xml_deserialize::MyTypeDeserializer);
        test!(deserializer.relative_to(&deserializer_path), MyTypeDeserializer);
    }
}
