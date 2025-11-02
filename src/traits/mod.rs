//! The `traits` module defines different public traits that are not specific to
//! a special module and are used generally in `xsd-parser`

mod as_any;
mod naming;
mod vec_helper;
mod with_namespace;

pub use as_any::AsAny;
pub use naming::{NameBuilder, NameBuilderExt, NameFallback, Naming};
pub use vec_helper::{VecHelper, WithIdent};
pub use with_namespace::WithNamespace;
