use xsd_parser::{
    config::NamespaceIdent, pipeline::renderer::NamespaceSerialization, Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default().with_generate([(
        IdentType::Element,
        Some(NamespaceIdent::namespace(b"http://example.com")),
        "Foo",
    )])
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/namespaces_unqualified/schema.xsd",
        "tests/feature/namespaces_unqualified/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/namespaces_unqualified/example/default.xml",
    );

    assert_eq!(obj.bar, "hello");
    assert_eq!(obj.baz, 42);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::Foo;

    let obj = Foo {
        bar: "hello".into(),
        baz: 42,
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/namespaces_unqualified/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_prefixed() {
    use quick_xml::Foo;

    let obj = Foo {
        bar: "hello".into(),
        baz: 42,
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/namespaces_unqualified/example/prefixed.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* quick_xml_local */

#[test]
fn generate_quick_xml_local() {
    generate_test(
        "tests/feature/namespaces_unqualified/schema.xsd",
        "tests/feature/namespaces_unqualified/expected/quick_xml_local.rs",
        config()
            .with_quick_xml_deserialize()
            .with_quick_xml_serialize_config(NamespaceSerialization::Local, None),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_local() {
    use quick_xml_local::Foo;

    let obj = Foo {
        bar: "hello".into(),
        baz: 42,
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/namespaces_unqualified/example/default.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_local_prefixed() {
    use quick_xml_local::Foo;

    let obj = Foo {
        bar: "hello".into(),
        baz: 42,
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "tns:Foo",
        "tests/feature/namespaces_unqualified/example/prefixed.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_local {
    #![allow(unused_imports)]

    include!("expected/quick_xml_local.rs");
}
