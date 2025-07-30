use xsd_parser::Config;

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/include_target_namespace/schema.xsd",
        "tests/feature/include_target_namespace/expected/default.rs",
        Config::test_default(),
    );
}
