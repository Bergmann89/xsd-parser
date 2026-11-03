//! Regression test: substitution group members without a type are aliases of
//! the type of their head. In `TypedefMode::Typedef` this produced conflicting
//! trait implementations (E0119) in xsd-parser 1.5.2 (without quick-xml too).

use xsd_parser::{config::TypedefMode, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_typedef_mode(TypedefMode::Typedef)
        .with_generate([(IdentType::Element, "tns:Root")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/substitution_group_typeless_members/schema.xsd",
        "tests/feature/substitution_group_typeless_members/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/substitution_group_typeless_members/schema.xsd",
        "tests/feature/substitution_group_typeless_members/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
