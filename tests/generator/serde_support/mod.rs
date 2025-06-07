use xsd_parser::{config::SerdeSupport, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn none() {
    generate_test(
        "tests/generator/serde_support/schema.xsd",
        "tests/generator/serde_support/expected/none.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::None)
            .with_generate([
                (IdentType::Type, "tns:MyAll"),
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[test]
fn quick_xml() {
    generate_test(
        "tests/generator/serde_support/schema.xsd",
        "tests/generator/serde_support/expected/quick_xml.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::QuickXml)
            .with_generate([
                (IdentType::Type, "tns:MyAll"),
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}

#[test]
fn serde_xml_rs() {
    generate_test(
        "tests/generator/serde_support/schema.xsd",
        "tests/generator/serde_support/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::SerdeXmlRs)
            .with_generate([
                (IdentType::Type, "tns:MyAll"),
                (IdentType::Type, "tns:MyChoice"),
                (IdentType::Type, "tns:MySequence"),
            ]),
    );
}
