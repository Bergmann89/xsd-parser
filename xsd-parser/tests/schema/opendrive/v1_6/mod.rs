use xsd_parser::Config;

use crate::utils::generate_test_smoke;
use xsd_parser::config::{GeneratorFlags, OptimizerFlags};

fn config() -> Config {
    Config::default()
        .with_optimizer_flags(OptimizerFlags::FLATTEN_COMPLEX_TYPES)
        .with_generator_flags(
            GeneratorFlags::all()
                - GeneratorFlags::USE_SCHEMA_MODULES
                - GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS
                - GeneratorFlags::NILLABLE_TYPE_SUPPORT
                - GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS,
        )
        .with_any_type_support()
}

/* default */

#[test]
fn generate_default_smoke() {
    // Smoke test to check if its possible to generate the code without errors.
    generate_test_smoke(
        "tests/schema/opendrive/v1_6/schema/opendrive_16_core.xsd",
        config(),
    );
}
