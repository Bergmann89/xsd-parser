//! The `traits` module defines different public traits that are not specific to
//! a special module and are used generally in `xsd-parser`

mod naming;
mod vec_helper;

pub use naming::{NameBuilder, NameBuilderExt, NameFallback, Naming};
pub use vec_helper::{VecHelper, WithIdent};
