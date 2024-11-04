use xsd_parser::{generator::SerdeSupport, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/union/schema.xsd",
        "tests/schema/union/expected/default.rs",
        Config::default().with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/union/schema.xsd",
        "tests/schema/union/expected/quick_xml.rs",
        Config::default()
            .with_quick_xml()
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/schema/union/schema.xsd",
        "tests/schema/union/expected/serde_xml_rs.rs",
        Config::default()
            .with_serde(SerdeSupport::SerdeXmlRs)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/schema/union/schema.xsd",
        "tests/schema/union/expected/serde_quick_xml.rs",
        Config::default()
            .with_serde(SerdeSupport::QuickXml)
            .with_generate([(IdentType::Element, "tns:Foo")]),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{Foo, UnionType};

    let obj = crate::utils::quick_xml_read_test::<Foo, _>("tests/schema/union/example/default.xml");

    assert!(matches!(obj.union_, UnionType::String(s) if s == "string"));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::{Foo, UnionType};

    let obj = Foo {
        union_: UnionType::String("string".into()),
    };

    crate::utils::quick_xml_write_test(&obj, "tns:Foo", "tests/schema/union/example/default.xml");
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
    #![allow(dead_code, unused_imports)]

    include!("expected/serde_xml_rs.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(dead_code, unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
