use xsd_parser::{generator::SerdeSupport, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/shiporder/schema.xsd",
        "tests/feature/shiporder/expected/default.rs",
        Config::test_default().with_generate([(IdentType::Element, "shiporder")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/shiporder/schema.xsd",
        "tests/feature/shiporder/expected/quick_xml.rs",
        Config::test_default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "shiporder")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/shiporder/schema.xsd",
        "tests/feature/shiporder/expected/serde_xml_rs.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::SerdeXmlRs)
            .with_generate([(IdentType::Element, "shiporder")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/shiporder/schema.xsd",
        "tests/feature/shiporder/expected/serde_quick_xml.rs",
        Config::test_default()
            .with_serde_support(SerdeSupport::QuickXml)
            .with_generate([(IdentType::Element, "shiporder")]),
    );
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! verify_obj {
    ($obj:expr) => {{
        let obj = $obj;
        let mut items = obj.item.iter();

        assert_eq!(obj.orderid, "889923");
        assert_eq!(obj.orderperson, "John Smith");

        assert_eq!(obj.shipto.name, "Ola Nordmann");
        assert_eq!(obj.shipto.address, "Langgt 23");
        assert_eq!(obj.shipto.city, "4000 Stavanger");
        assert_eq!(obj.shipto.country, "Norway");

        let item = items.next().unwrap();
        assert_eq!(item.title, "Empire Burlesque");
        assert!(matches!(item.note.as_deref(), Some("Special Edition")));
        assert_eq!(item.quantity, 1);
        assert_eq!(item.price, 10.90);

        let item = items.next().unwrap();
        assert_eq!(item.title, "Hide your heart");
        assert!(matches!(item.note.as_deref(), None));
        assert_eq!(item.quantity, 1);
        assert_eq!(item.price, 9.90);
    }};
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Shiporder;

    let obj = crate::utils::quick_xml_read_test::<Shiporder, _>(
        "tests/feature/shiporder/example/default.xml",
    );

    verify_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Shiporder;

    let obj = crate::utils::serde_xml_rs_read_test::<Shiporder, _>(
        "tests/feature/shiporder/example/default.xml",
    );

    verify_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Shiporder;

    let obj = crate::utils::serde_quick_xml_read_test::<Shiporder, _>(
        "tests/feature/shiporder/example/default.xml",
    );

    verify_obj!(obj);
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
