use xsd_parser::{config::GeneratorFlags, generator::SerdeSupport, types::IdentType, Config};

use crate::utils::{generate_test, ConfigEx};

fn serde_config() -> Config {
    let mut config = Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([(IdentType::Element, "tns:Foo")]);

    config.generator.any_type = Some("xsd_parser::xml::AnyElement".into());
    config.generator.any_attribute_type = Some("xsd_parser::xml::AnyAttributes".into());

    config
}

fn config() -> Config {
    let mut config = serde_config();

    config
}

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/any/schema.xsd",
        "tests/feature/any/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/any/schema.xsd",
        "tests/feature/any/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/feature/any/schema.xsd",
        "tests/feature/any/expected/serde_xml_rs.rs",
        serde_config().with_serde_support(SerdeSupport::SerdeXmlRs),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/feature/any/schema.xsd",
        "tests/feature/any/expected/serde_quick_xml.rs",
        serde_config().with_serde_support(SerdeSupport::QuickXml),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    /*
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>("tests/feature/any/example/default.xml");

    assert_eq!(obj.name, "abcd");
    */
    unimplemented!()
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    /*
    use quick_xml::Foo;

    let obj = Foo {
        any_attributes: Default::default(),
        name: "abcd".into(),
        any_elements: Default::default(),
    };

    crate::utils::quick_xml_write_test(&obj, "tns:Foo", "tests/feature/any/example/serialize.xml");
    */
    unimplemented!()
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::Foo;

    let obj =
        crate::utils::serde_xml_rs_read_test::<Foo, _>("tests/feature/any/example/default.xml");

    assert_eq!(obj.name, "abcd");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::Foo;

    let obj =
        crate::utils::serde_quick_xml_read_test::<Foo, _>("tests/feature/any/example/default.xml");

    assert_eq!(obj.name, "abcd");
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
