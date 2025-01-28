use xsd_parser::{generator::TypedefMode, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn auto() {
    generate_test(
        "tests/generator/typedef_mode/schema.xsd",
        "tests/generator/typedef_mode/expected/auto.rs",
        Config::test_default()
            .with_typedef_mode(TypedefMode::Auto)
            .with_generate([
                (IdentType::Type, "tns:MyType"),
                (IdentType::Type, "tns:MyList"),
            ]),
    );
}

#[test]
fn typedef() {
    generate_test(
        "tests/generator/typedef_mode/schema.xsd",
        "tests/generator/typedef_mode/expected/typedef.rs",
        Config::test_default()
            .with_typedef_mode(TypedefMode::Typedef)
            .with_generate([
                (IdentType::Type, "tns:MyType"),
                (IdentType::Type, "tns:MyList"),
            ]),
    );
}

#[test]
fn new_type() {
    generate_test(
        "tests/generator/typedef_mode/schema.xsd",
        "tests/generator/typedef_mode/expected/new_type.rs",
        Config::test_default()
            .with_typedef_mode(TypedefMode::NewType)
            .with_generate([
                (IdentType::Type, "tns:MyType"),
                (IdentType::Type, "tns:MyList"),
            ]),
    );
}
