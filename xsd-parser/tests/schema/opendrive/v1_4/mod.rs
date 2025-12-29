use xsd_parser::Config;

use crate::utils::{generate_test_smoke, ConfigEx};

fn config() -> Config {
    Config::test_default()
}

/* default */

#[test]
fn generate_default_smoke() {
    // Smoke test to check if its possible to generate the code without errors.
    generate_test_smoke("tests/schema/opendrive/v1_4/OpenDRIVE_1.4H.xsd", config());
}
