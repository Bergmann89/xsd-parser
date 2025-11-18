use xsd_parser::{
    config::{GeneratorFlags, NamespaceIdent},
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

        assert_eq!(obj.bar_inner.a, "Text in A");
        assert_eq!(obj.baz_inner.b, "Text in B");
        assert_eq!(obj.biz_inner.c, "Text in C");
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj {
    ($module:ident) => {{
        use $module::{
            bar::InnerType as BarInner, baz::InnerType as BazInner, biz::InnerType as BizInner,
            Outer,
        };

        Outer {
            bar_inner: BarInner {
                a: "Text in A".into(),
            },
            baz_inner: BazInner {
                b: "Text in B".into(),
            },
            biz_inner: BizInner {
                c: "Text in C".into(),
            },
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/element_refs_with_ns/schema.xsd",
        "tests/feature/element_refs_with_ns/expected/default.rs",
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
        "tests/feature/element_refs_with_ns/schema.xsd",
        "tests/feature/element_refs_with_ns/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Outer;

    let obj = crate::utils::quick_xml_read_test::<Outer, _>(
        "tests/feature/element_refs_with_ns/example/default.xml",
    );

    check_obj!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    let obj = test_obj!(quick_xml);

    crate::utils::quick_xml_write_test(
        &obj,
        "Outer",
        "tests/feature/element_refs_with_ns/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
