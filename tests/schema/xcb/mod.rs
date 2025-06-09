use xsd_parser::{
    config::{GeneratorFlags, OptimizerFlags},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/xcb/schema.xsd",
        "tests/schema/xcb/expected/default.rs",
        Config::test_default()
            .with_optimizer_flags(OptimizerFlags::FLATTEN_COMPLEX_TYPES)
            .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
            .with_generate([(IdentType::Element, "xcb")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/xcb/schema.xsd",
        "tests/schema/xcb/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_optimizer_flags(OptimizerFlags::FLATTEN_COMPLEX_TYPES)
            .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
            .with_generate([(IdentType::Element, "xcb")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/schema/xcb/schema.xsd",
        "tests/schema/xcb/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_xml_rs()
            .with_optimizer_flags(OptimizerFlags::FLATTEN_COMPLEX_TYPES)
            .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
            .with_generate([(IdentType::Element, "xcb")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/schema/xcb/schema.xsd",
        "tests/schema/xcb/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_quick_xml()
            .with_optimizer_flags(OptimizerFlags::FLATTEN_COMPLEX_TYPES)
            .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
            .with_generate([(IdentType::Element, "xcb")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use crate::utils::quick_xml_read_test;
    use quick_xml::Xcb;

    quick_xml_read_test::<Xcb, _>("tests/schema/xcb/example/render.xml");
    quick_xml_read_test::<Xcb, _>("tests/schema/xcb/example/res.xml");
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
