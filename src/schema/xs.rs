#![allow(
    unused_mut,
    unused_variables,
    clippy::redundant_field_names,
    clippy::large_enum_variant
)]

use super::{MaxOccurs, QName};

pub type Use = AttributeUseType;

include!("./xs_generated_new.rs");
