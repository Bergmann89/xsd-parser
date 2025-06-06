//! Transformation pipeline for processing schema definitions into Rust code.
//!
//! This module defines the sequential stages that transform raw schema input
//! into optimized, code-ready Rust type representations. Each stage is
//! encapsulated in a dedicated component responsible for a specific layer
//! of the transformation.

pub mod generator;
pub mod interpreter;
pub mod optimizer;
pub mod parser;

mod types_printer;

pub use self::generator::Generator;
pub use self::interpreter::Interpreter;
pub use self::optimizer::Optimizer;
pub use self::parser::Parser;

pub(crate) use self::types_printer::TypesPrinter;
