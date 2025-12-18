use xsd_parser::{
    config::{NamespaceIdent, ParserFlags},
    pipeline::renderer::NamespaceSerialization,
    Config, IdentType,
};
use xsd_parser_types::misc::Namespace;

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_quick_xml_deserialize()
        .with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::Namespace(Namespace::new_const(b"Foo"))),
            "Foo",
        )])
}

/* quick_xml_local_alt */

#[test]
fn generate_quick_xml_local_alt() {
    generate_test(
        "tests/feature/namespaces_qualified/foo.xsd",
        "tests/feature/namespaces_qualified/expected/quick_xml_local_alt.rs",
        config().with_quick_xml_serialize_config(NamespaceSerialization::Local, None),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_local_alt() {
    use quick_xml_local_alt::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/namespaces_qualified/example/local_alt.xml",
    );

    assert_eq!(obj.inner_1.a, "Bar String");
    assert_eq!(obj.inner_2.b, "Baz String");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_local_alt() {
    use quick_xml_local_alt::{Foo, Inner1Type, Inner2Type};

    let obj = Foo {
        inner_1: Inner1Type {
            a: "Bar String".into(),
        },
        inner_2: Inner2Type {
            b: "Baz String".into(),
        },
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/namespaces_qualified/example/local_alt.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_local_alt {
    #![allow(unused_imports)]

    include!("expected/quick_xml_local_alt.rs");
}

/* quick_xml_local_no_alt */

#[test]
fn generate_quick_xml_local_no_alt() {
    generate_test(
        "tests/feature/namespaces_qualified/foo.xsd",
        "tests/feature/namespaces_qualified/expected/quick_xml_local_no_alt.rs",
        config()
            .without_parser_flags(ParserFlags::ALTERNATIVE_PREFIXES)
            .with_quick_xml_serialize_config(NamespaceSerialization::Local, None),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_local_no_alt() {
    use quick_xml_local_no_alt::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/namespaces_qualified/example/local_no_alt.xml",
    );

    assert_eq!(obj.inner_1.a, "Bar String");
    assert_eq!(obj.inner_2.b, "Baz String");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_local_no_alt() {
    use quick_xml_local_no_alt::{Foo, Inner1Type, Inner2Type};

    let obj = Foo {
        inner_1: Inner1Type {
            a: "Bar String".into(),
        },
        inner_2: Inner2Type {
            b: "Baz String".into(),
        },
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/namespaces_qualified/example/local_no_alt.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_local_no_alt {
    #![allow(unused_imports)]

    include!("expected/quick_xml_local_no_alt.rs");
}

/* quick_xml_global_alt */

#[test]
fn generate_quick_xml_global_alt() {
    generate_test(
        "tests/feature/namespaces_qualified/foo.xsd",
        "tests/feature/namespaces_qualified/expected/quick_xml_global_alt.rs",
        config().with_quick_xml_serialize_config(NamespaceSerialization::Global, None),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_global_alt() {
    use quick_xml_global_alt::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/namespaces_qualified/example/global_alt.xml",
    );

    assert_eq!(obj.inner_1.a, "Bar String");
    assert_eq!(obj.inner_2.b, "Baz String");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_global_alt() {
    use quick_xml_global_alt::{Foo, Inner1Type, Inner2Type};

    let obj = Foo {
        inner_1: Inner1Type {
            a: "Bar String".into(),
        },
        inner_2: Inner2Type {
            b: "Baz String".into(),
        },
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/namespaces_qualified/example/global_alt.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_global_alt {
    #![allow(unused_imports)]

    include!("expected/quick_xml_global_alt.rs");
}

/* quick_xml_global_no_alt */

#[test]
fn generate_quick_xml_global_no_alt() {
    generate_test(
        "tests/feature/namespaces_qualified/foo.xsd",
        "tests/feature/namespaces_qualified/expected/quick_xml_global_no_alt.rs",
        config()
            .without_parser_flags(ParserFlags::ALTERNATIVE_PREFIXES)
            .with_quick_xml_serialize_config(NamespaceSerialization::Global, None),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_global_no_alt() {
    use quick_xml_global_no_alt::Foo;

    let obj = crate::utils::quick_xml_read_test::<Foo, _>(
        "tests/feature/namespaces_qualified/example/global_no_alt.xml",
    );

    assert_eq!(obj.inner_1.a, "Bar String");
    assert_eq!(obj.inner_2.b, "Baz String");
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_global_no_alt() {
    use quick_xml_global_no_alt::{Foo, Inner1Type, Inner2Type};

    let obj = Foo {
        inner_1: Inner1Type {
            a: "Bar String".into(),
        },
        inner_2: Inner2Type {
            b: "Baz String".into(),
        },
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Foo",
        "tests/feature/namespaces_qualified/example/global_no_alt.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_global_no_alt {
    #![allow(unused_imports)]

    include!("expected/quick_xml_global_no_alt.rs");
}
