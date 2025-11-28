use xsd_parser::{config::SerdeXmlRsVersion, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(IdentType::Element, "tns:Foo")])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/sequence/schema.xsd",
        "tests/feature/sequence/expected/default.rs",
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
        "tests/feature/sequence/schema.xsd",
        "tests/feature/sequence/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj =
        crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/sequence/example/default.xml");

    assert_eq!(obj.min, 1);
    assert_eq!(obj.max, 2);
}

#[tokio::test]
#[cfg(not(feature = "update-expectations"))]
async fn read_quick_xml_async() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test_async::<Foo, _>(
        "tests/feature/sequence/example/default.xml",
    )
    .await;

    assert_eq!(obj.min, 1);
    assert_eq!(obj.max, 2);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::Foo;

    let obj = Foo { min: 1, max: 2 };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/sequence/example/default.xml",
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
        "tests/feature/sequence/schema.xsd",
        "tests/feature/sequence/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Foo;

    let obj = crate::utils::serde_xml_rs_read_test::<Foo, _>(
        "tests/feature/sequence/example/default.xml",
    );

    assert_eq!(obj.min, 1);
    assert_eq!(obj.max, 2);
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
        "tests/feature/sequence/schema.xsd",
        "tests/feature/sequence/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Foo;

    let obj = crate::utils::serde_quick_xml_read_test::<Foo, _>(
        "tests/feature/sequence/example/default.xml",
    );

    assert_eq!(obj.min, 1);
    assert_eq!(obj.max, 2);
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
