#![allow(dead_code, deprecated, missing_docs)]

// Compiled polymorphic_from::alloc_boxed expectations use ::alloc paths.
#[cfg(not(feature = "update-expectations"))]
extern crate alloc;

mod feature;
mod generator;
mod optimizer;
mod renderer;
mod schema;
mod utils;
