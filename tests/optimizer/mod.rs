use xsd_parser::{config::OptimizerFlags, types::IdentType, Config};

use crate::utils::{optimizer_test, optimizer_test_with_config, ConfigEx};

#[test]
fn remove_empty_enum_variants() {
    optimizer_test(
        "tests/optimizer/enum_empty_variant.xsd",
        "tests/optimizer/expected0/remove_empty_enum_variants.rs",
        "tests/optimizer/expected1/remove_empty_enum_variants.rs",
        [(IdentType::Type, "tns:MyEnum")],
        OptimizerFlags::REMOVE_EMPTY_ENUM_VARIANTS,
    )
}

#[test]
fn remove_empty_enums() {
    optimizer_test(
        "tests/optimizer/enum_empty.xsd",
        "tests/optimizer/expected0/remove_empty_enums.rs",
        "tests/optimizer/expected1/remove_empty_enums.rs",
        [(IdentType::Type, "tns:MyEnum")],
        OptimizerFlags::REMOVE_EMPTY_ENUM_VARIANTS | OptimizerFlags::REMOVE_EMPTY_ENUMS,
    )
}

#[test]
fn remove_duplicate_union_variants() {
    optimizer_test_with_config(
        "tests/optimizer/union_duplicate.xsd",
        "tests/optimizer/expected0/remove_duplicate_union_variants.rs",
        "tests/optimizer/expected1/remove_duplicate_union_variants.rs",
        [(IdentType::Type, "tns:MyUnion")],
        OptimizerFlags::REMOVE_DUPLICATE_UNION_VARIANTS,
        Config::test_default().without_optimizer_flags(OptimizerFlags::all()),
    )
}

#[test]
fn remove_empty_unions() {
    optimizer_test(
        "tests/optimizer/union_empty.xsd",
        "tests/optimizer/expected0/remove_empty_unions.rs",
        "tests/optimizer/expected1/remove_empty_unions.rs",
        [(IdentType::Type, "tns:MyUnion")],
        OptimizerFlags::REMOVE_EMPTY_UNIONS,
    )
}

#[test]
fn use_unrestricted_base_type() {
    optimizer_test(
        "tests/optimizer/complex_restricted.xsd",
        "tests/optimizer/expected0/use_unrestricted_base_type.rs",
        "tests/optimizer/expected1/use_unrestricted_base_type.rs",
        [
            (IdentType::Type, "tns:FirstRestrictedType"),
            (IdentType::Type, "tns:SecondRestrictedType"),
            (IdentType::Type, "tns:ThirdRestrictedType"),
        ],
        OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE,
    )
}

#[test]
fn convert_dynamic_to_choice() {
    optimizer_test(
        "tests/optimizer/abstract.xsd",
        "tests/optimizer/expected0/convert_dynamic_to_choice.rs",
        "tests/optimizer/expected1/convert_dynamic_to_choice.rs",
        [(IdentType::Element, "tns:Abstract")],
        OptimizerFlags::CONVERT_DYNAMIC_TO_CHOICE,
    )
}

#[test]
fn flatten_complex_types() {
    optimizer_test(
        "tests/optimizer/complex_flatten.xsd",
        "tests/optimizer/expected0/flatten_complex_types.rs",
        "tests/optimizer/expected1/flatten_complex_types.rs",
        [(IdentType::Type, "tns:MyComplexType")],
        OptimizerFlags::FLATTEN_COMPLEX_TYPES,
    )
}

#[test]
fn flatten_unions() {
    optimizer_test(
        "tests/optimizer/union_flatten.xsd",
        "tests/optimizer/expected0/flatten_unions.rs",
        "tests/optimizer/expected1/flatten_unions.rs",
        [(IdentType::Type, "tns:MyUnion")],
        OptimizerFlags::FLATTEN_UNIONS,
    )
}

#[test]
fn merge_enum_unions() {
    optimizer_test(
        "tests/optimizer/union_flatten.xsd",
        "tests/optimizer/expected0/merge_enum_unions.rs",
        "tests/optimizer/expected1/merge_enum_unions.rs",
        [(IdentType::Type, "tns:MyUnion")],
        OptimizerFlags::MERGE_ENUM_UNIONS,
    )
}

#[test]
fn resolve_typedefs() {
    optimizer_test_with_config(
        "tests/optimizer/complex_flatten.xsd",
        "tests/optimizer/expected0/resolve_typedefs.rs",
        "tests/optimizer/expected1/resolve_typedefs.rs",
        [(IdentType::Type, "tns:MyComplexType")],
        OptimizerFlags::RESOLVE_TYPEDEFS,
        Config::test_default().without_optimizer_flags(OptimizerFlags::RESOLVE_TYPEDEFS),
    )
}

#[test]
fn remove_duplicates() {
    optimizer_test(
        "tests/optimizer/duplicate.xsd",
        "tests/optimizer/expected0/remove_duplicates.rs",
        "tests/optimizer/expected1/remove_duplicates.rs",
        [
            (IdentType::Type, "tns:First"),
            (IdentType::Type, "tns:Second"),
        ],
        OptimizerFlags::REMOVE_DUPLICATES,
    );
}
