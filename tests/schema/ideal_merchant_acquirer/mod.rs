use xsd_parser::{config::NamespaceIdent, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/ideal_merchant_acquirer/schema.xsd",
        "tests/schema/ideal_merchant_acquirer/expected/default.rs",
        Config::test_default().with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::namespace(
                b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1",
            )),
            "DirectoryReq",
        )]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/ideal_merchant_acquirer/schema.xsd",
        "tests/schema/ideal_merchant_acquirer/expected/quick_xml.rs",
        Config::test_default().with_quick_xml().with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::namespace(
                b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1",
            )),
            "DirectoryReq",
        )]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/schema/ideal_merchant_acquirer/schema.xsd",
        "tests/schema/ideal_merchant_acquirer/expected/serde_xml_rs.rs",
        Config::test_default().with_serde_xml_rs().with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::namespace(
                b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1",
            )),
            "DirectoryReq",
        )]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/schema/ideal_merchant_acquirer/schema.xsd",
        "tests/schema/ideal_merchant_acquirer/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_quick_xml()
            .with_generate([(
                IdentType::Element,
                Some(NamespaceIdent::namespace(
                    b"http://www.idealdesk.com/ideal/messages/mer-acq/3.3.1",
                )),
                "DirectoryReq",
            )]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::DirectoryReq;

    crate::utils::quick_xml_read_test::<DirectoryReq, _>(
        "tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::DirectoryReq;

    crate::utils::serde_xml_rs_read_test::<DirectoryReq, _>(
        "tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::DirectoryReq;

    crate::utils::serde_quick_xml_read_test::<DirectoryReq, _>(
        "tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
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
