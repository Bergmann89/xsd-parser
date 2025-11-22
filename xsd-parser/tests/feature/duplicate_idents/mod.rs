use xsd_parser::{config::GeneratorFlags, Config};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::USE_MODULES | GeneratorFlags::USE_SCHEMA_MODULES)
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj_foo {
    ($obj:expr) => {{
        let obj = $obj;

        assert_eq!(obj.inner.a, "a");
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj_foo {
    ($module:ident) => {{
        use $module::foo::{Outer, Inner};

        Outer {
            inner: Inner {
                a: "a".into(),
            },
        }
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! check_obj_bar {
    ($obj:expr) => {{
        let obj = $obj;

        assert_eq!(obj.inner.b, "b");
    }};
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! test_obj_bar {
    ($module:ident) => {{
        use $module::bar::{Outer, Inner};

        Outer {
            inner: Inner {
                b: "b".into(),
            },
        }
    }};
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/duplicate_idents/schema.xsd",
        "tests/feature/duplicate_idents/expected/default.rs",
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
        "tests/feature/duplicate_idents/schema.xsd",
        "tests/feature/duplicate_idents/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_foo() {
    use quick_xml::foo::Outer;

    let obj = crate::utils::quick_xml_read_test::<Outer, _>(
        "tests/feature/duplicate_idents/example/foo.xml",
    );

    check_obj_foo!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_foo() {
    let obj = test_obj_foo!(quick_xml);

    crate::utils::quick_xml_write_test(
        &obj,
        "Outer",
        "tests/feature/duplicate_idents/example/foo.xml",
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml_bar() {
    use quick_xml::bar::Outer;

    let obj = crate::utils::quick_xml_read_test::<Outer, _>(
        "tests/feature/duplicate_idents/example/bar.xml",
    );

    check_obj_bar!(obj);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml_bar() {
    let obj = test_obj_bar!(quick_xml);

    crate::utils::quick_xml_write_test(
        &obj,
        "Outer",
        "tests/feature/duplicate_idents/example/bar.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}
