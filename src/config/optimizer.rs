use std::path::PathBuf;

use bitflags::bitflags;

/// Configuration for the type information optimizer.
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Additional flags to control the optimizer.
    pub flags: OptimizerFlags,

    /// Wether to enable the debug output and where to write it to.
    pub debug_output: Option<PathBuf>,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            debug_output: None,
            flags: OptimizerFlags::REMOVE_EMPTY_ENUM_VARIANTS
                | OptimizerFlags::REMOVE_EMPTY_ENUMS
                | OptimizerFlags::REMOVE_DUPLICATE_UNION_VARIANTS
                | OptimizerFlags::REMOVE_EMPTY_UNIONS,
        }
    }
}

bitflags! {
    /// Flags to control the [`Optimizer`](crate::Optimizer).
    #[derive(Debug, Clone)]
    pub struct OptimizerFlags: u32 {
        /// Whether to remove empty enum variants or not.
        ///
        /// See [`remove_empty_enum_variants`](crate::Optimizer::remove_empty_enum_variants) for details.
        const REMOVE_EMPTY_ENUM_VARIANTS = 1 << 0;

        /// Whether to remove empty enums or not.
        ///
        /// See [`remove_empty_enums`](crate::Optimizer::remove_empty_enums) for details.
        const REMOVE_EMPTY_ENUMS = 1 << 1;

        /// Whether to remove duplicate union variants or not.
        ///
        /// See [`remove_duplicate_union_variants`](crate::Optimizer::remove_duplicate_union_variants) for details.
        const REMOVE_DUPLICATE_UNION_VARIANTS = 1 << 2;

        /// Whether to remove empty unions or not.
        ///
        /// See [`remove_empty_unions`](crate::Optimizer::remove_empty_unions) for details.
        const REMOVE_EMPTY_UNIONS = 1 << 3;

        /// Whether to use the unrestricted base type of a type or not.
        ///
        /// See [`use_unrestricted_base_type`](crate::Optimizer::use_unrestricted_base_type) for details.
        const USE_UNRESTRICTED_BASE_TYPE = 1 << 4;

        /// Whether to convert dynamic types to choices or not.
        ///
        /// See [`convert_dynamic_to_choice`](crate::Optimizer::convert_dynamic_to_choice) for details.
        const CONVERT_DYNAMIC_TO_CHOICE = 1 << 5;

        /// Whether to flatten the content of complex types or not.
        ///
        /// See [`flatten_complex_types`](crate::Optimizer::flatten_complex_types) for details.
        const FLATTEN_COMPLEX_TYPES = 1 << 6;

        /// Whether to flatten unions or not.
        ///
        /// See [`flatten_unions`](crate::Optimizer::flatten_unions) for details.
        const FLATTEN_UNIONS = 1 << 7;

        /// Whether to merge enumerations and unions or not.
        ///
        /// See [`merge_enum_unions`](crate::Optimizer::merge_enum_unions) for details.
        const MERGE_ENUM_UNIONS = 1 << 8;

        /// Whether to resolve type definitions or not.
        ///
        /// See [`resolve_typedefs`](crate::Optimizer::resolve_typedefs) for details.
        const RESOLVE_TYPEDEFS = 1 << 9;

        /// Whether to remove duplicate types or not.
        ///
        /// See [`remove_duplicates`](crate::Optimizer::remove_duplicates) for details.
        const REMOVE_DUPLICATES = 1 << 10;

        /// Group that contains all necessary optimization that should be applied
        /// if code with [`serde`] support should be rendered.
        const SERDE =  Self::FLATTEN_COMPLEX_TYPES.bits()
            | Self::FLATTEN_UNIONS.bits()
            | Self::MERGE_ENUM_UNIONS.bits();

        /// Wether to merge the cardinality of a complex choice type or not.
        ///
        /// See [`merge_choice_cardinalities`](crate::Optimizer::merge_choice_cardinalities) for details.
        const MERGE_CHOICE_CARDINALITIES = 1 << 11;

        /// Wether to simplify complex mixed types or not.
        ///
        /// See [`simplify_mixed_types`](crate::Optimizer::simplify_mixed_types) for details.
        const SIMPLIFY_MIXED_TYPES = 1 << 12;
    }
}
