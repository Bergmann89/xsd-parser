#![allow(
    unused_mut,
    missing_docs,
    unused_variables,
    clippy::len_zero,
    clippy::single_match,
    clippy::needless_pass_by_value,
    clippy::unused_self,
    clippy::unnecessary_wraps,
    clippy::redundant_else,
    clippy::redundant_field_names,
    clippy::too_many_lines,
    clippy::large_enum_variant,
    clippy::semicolon_if_nothing_returned
)]

use super::{MaxOccurs, QName};

pub type Use = AttributeUseType;

include!("./xs_generated_new.rs");
