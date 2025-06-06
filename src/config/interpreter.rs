use std::path::PathBuf;

use bitflags::bitflags;

use crate::models::types::Type;

use super::IdentTriple;

/// Configuration for the schema interpreter.
#[derive(Debug, Clone)]
pub struct InterpreterConfig {
    /// List of user defined types to add to the interpreter before the schemas
    /// are actually interpreted.
    ///
    /// See [`with_type`](crate::Interpreter::with_type) for more details.
    pub types: Vec<(IdentTriple, Type)>,

    /// Additional flags to control the interpreter.
    pub flags: InterpreterFlags,

    /// Wether to enable the debug output and where to write it to.
    pub debug_output: Option<PathBuf>,
}

impl Default for InterpreterConfig {
    fn default() -> Self {
        Self {
            types: vec![],
            debug_output: None,
            flags: InterpreterFlags::BUILDIN_TYPES
                | InterpreterFlags::DEFAULT_TYPEDEFS
                | InterpreterFlags::WITH_XS_ANY_TYPE,
        }
    }
}

bitflags! {
    /// Flags to control the [`Interpreter`](crate::Interpreter).
    #[derive(Debug, Clone)]
    pub struct InterpreterFlags: u32 {
        /// Whether to add the build-in types to the interpreter or not.
        ///
        /// See [`with_buildin_types`](crate::Interpreter::with_buildin_types) for details.
        const BUILDIN_TYPES = 1 << 0;

        /// Whether to add the default types definitions to the interpreter or not.
        ///
        /// See [`with_default_typedefs`](crate::Interpreter::with_default_typedefs) for details.
        const DEFAULT_TYPEDEFS = 1 << 1;

        /// Whether to add a default type definitions for `xs:anyType` or not.
        ///
        /// See [`with_xs_any_type`](crate::Interpreter::with_xs_any_type) for details.
        const WITH_XS_ANY_TYPE = 1 << 2;

        /// Whether to use `num::BigInt` and `num::BigUint` instead of build-in integer types.
        ///
        /// See [`with_num_big_int`](crate::Interpreter::with_num_big_int) for details.
        const WITH_NUM_BIG_INT = 1 << 3;
    }
}
