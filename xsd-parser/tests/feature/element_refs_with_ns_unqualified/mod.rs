use xsd_parser::{
    config::{GeneratorFlags, NamespaceIdent},
    pipeline::renderer::NamespaceSerialization,
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::USE_MODULES)
        .with_generate([(
            IdentType::Element,
            Some(NamespaceIdent::namespace(b"Foo")),
            "Outer",
        )])
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj {
    ($obj:expr) => {{
        let obj = $obj;

        assert_eq!(obj.inner.a, "Text in A");
        assert_eq!(obj.note, "Text in Note");
        assert_eq!(obj.plain, "Text in Plain");
        assert_eq!(obj.typed, "Text in Typed");
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use $module::{bar::InnerType, Outer};

        Outer {
            inner: InnerType {
                a: "Text in A".into(),
            },
            note: "Text in Note".into(),
            plain: "Text in Plain".into(),
            typed: "Text in Typed".into(),
        }
    }};
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/element_refs_with_ns_unqualified/schema.xsd",
        "tests/feature/element_refs_with_ns_unqualified/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    let obj = crate::utils::quick_xml_read_test::<quick_xml::Outer, _>(
        "tests/feature/element_refs_with_ns_unqualified/example/default.xml",
    );

    check_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    crate::utils::quick_xml_write_test(
        &test_obj!(quick_xml),
        "foo:Outer",
        "tests/feature/element_refs_with_ns_unqualified/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    include!("expected/quick_xml.rs");
}

/* quick_xml_dynamic */

#[test]
fn generate_quick_xml_dynamic() {
    generate_test(
        "tests/feature/element_refs_with_ns_unqualified/schema.xsd",
        "tests/feature/element_refs_with_ns_unqualified/expected/quick_xml_dynamic.rs",
        config().with_quick_xml_config(NamespaceSerialization::Dynamic, None, false, false),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_dynamic() {
    crate::utils::quick_xml_write_test(
        &test_obj!(quick_xml_dynamic),
        "foo:Outer",
        "tests/feature/element_refs_with_ns_unqualified/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml_dynamic {
    include!("expected/quick_xml_dynamic.rs");
}
