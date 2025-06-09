use xsd_parser::{
    config::{GeneratorFlags, OptimizerFlags, ParserFlags, SerdeSupport},
    Config,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .without_parser_flags(ParserFlags::RESOLVE_INCLUDES)
        .with_optimizer_flags(OptimizerFlags::all() - OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE)
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::USE_MODULES)
}

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/xml_catalogs/schema.xsd",
        "tests/schema/xml_catalogs/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/xml_catalogs/schema.xsd",
        "tests/schema/xml_catalogs/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/schema/xml_catalogs/schema.xsd",
        "tests/schema/xml_catalogs/expected/serde_xml_rs.rs",
        config().with_serde_support(SerdeSupport::SerdeXmlRs),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/schema/xml_catalogs/schema.xsd",
        "tests/schema/xml_catalogs/expected/serde_quick_xml.rs",
        config().with_serde_support(SerdeSupport::QuickXml),
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
    include!("expected/serde_xml_rs.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
