use xsd_parser::{Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn deserializer() {
    generate_test(
        "tests/generator/quick_xml/schema.xsd",
        "tests/generator/quick_xml/expected/deserializer.rs",
        Config::test_default()
            .with_quick_xml_deserialize_config(false)
            .with_generate([
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[test]
fn deserializer_boxed() {
    generate_test(
        "tests/generator/quick_xml/schema.xsd",
        "tests/generator/quick_xml/expected/deserializer_boxed.rs",
        Config::test_default()
            .with_quick_xml_deserialize_config(true)
            .with_generate([
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod deserializer {
    #![allow(unreachable_pub)]

    include!("expected/deserializer.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod deserializer_boxed {
    #![allow(unreachable_pub)]

    include!("expected/deserializer_boxed.rs");
}
