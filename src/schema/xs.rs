#![allow(
    missing_docs,
    unused_imports,
    clippy::large_enum_variant,
    clippy::redundant_field_names,
    clippy::single_match,
    clippy::too_many_lines,
    clippy::wildcard_imports
)]

use super::{MaxOccurs, QName};

pub type Use = AttributeUseType;

include!("./xs_generated.rs");
