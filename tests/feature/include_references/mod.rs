use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/include_references/schema.xsd",
        "tests/feature/include_references/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Type, None, "myComplexType")]),
    );
}
