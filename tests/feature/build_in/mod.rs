use xsd_parser::Config;

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/build_in/schema.xsd",
        "tests/feature/build_in/expected/default.rs",
        Config::test_default(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/build_in/schema.xsd",
        "tests/feature/build_in/expected/quick_xml.rs",
        Config::test_default().with_quick_xml(),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/build_in/schema.xsd",
        "tests/feature/build_in/expected/serde_xml_rs.rs",
        Config::test_default().with_serde_xml_rs(),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/build_in/schema.xsd",
        "tests/feature/build_in/expected/serde_quick_xml.rs",
        Config::test_default().with_serde_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
