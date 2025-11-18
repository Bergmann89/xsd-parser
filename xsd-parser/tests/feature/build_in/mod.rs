use xsd_parser::{config::SerdeXmlRsVersion, Config};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/build_in/schema.xsd",
        "tests/feature/build_in/expected/default.rs",
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
        "tests/feature/build_in/schema.xsd",
        "tests/feature/build_in/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* serde_xml_rs */

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/build_in/schema.xsd",
        "tests/feature/build_in/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    #![allow(unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

/* serde_quick_xml */

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/build_in/schema.xsd",
        "tests/feature/build_in/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
