use xsd_parser::{
    config::{NamespaceIdent, SerdeSupport},
    models::schema::Namespace,
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

const NS: NamespaceIdent =
    NamespaceIdent::Namespace(Namespace::new_const(b"http://www.iata.org/IATA/2007/00"));

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/complex_type_empty/schema.xsd",
        "tests/feature/complex_type_empty/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Type, Some(NS), "SuccessType")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/complex_type_empty/schema.xsd",
        "tests/feature/complex_type_empty/expected/quick_xml.rs",
        Config::test_default().with_quick_xml().with_generate([(
            IdentType::Type,
            Some(NS),
            "SuccessType",
        )]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/complex_type_empty/schema.xsd",
        "tests/feature/complex_type_empty/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::SerdeXmlRs)
            .with_generate([(IdentType::Type, Some(NS), "SuccessType")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/complex_type_empty/schema.xsd",
        "tests/feature/complex_type_empty/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::QuickXml)
            .with_generate([(IdentType::Type, Some(NS), "SuccessType")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::SuccessType;

    crate::utils::quick_xml_read_test::<SuccessType, _>(
        "tests/feature/complex_type_empty/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::SuccessType;

    crate::utils::quick_xml_write_test(
        &SuccessType {},
        "SuccessType",
        "tests/feature/complex_type_empty/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::SuccessType;

    crate::utils::serde_xml_rs_read_test::<SuccessType, _>(
        "tests/feature/complex_type_empty/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::SuccessType;

    crate::utils::serde_quick_xml_read_test::<SuccessType, _>(
        "tests/feature/complex_type_empty/example/default.xml",
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
